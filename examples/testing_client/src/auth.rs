use tidlers::client::TidalClient;
use tidlers::{TidalError, auth::TidalAuth};

use crate::{oauth_handler::setup_oauth_status_listener, save::get_session_data};

pub async fn handle_auth(use_pkce: bool) -> Option<TidalAuth> {
    // check for saved session data
    let saved_session_data = get_session_data();

    // if we have saved session data, load it, otherwise do oauth flow
    let auth = if saved_session_data.is_some() {
        println!("found saved session data");
        return None;
    } else if use_pkce {
        TidalAuth::with_pkce()
    } else {
        TidalAuth::with_oauth()
    };

    Some(auth)
}

pub async fn handle_oauth_flow(tidal_client: &mut TidalClient) -> Result<(), TidalError> {
    let oauth = tidal_client.get_oauth_link().await?;

    println!(
        "visit this link to login: https://{}",
        oauth.verification_uri_complete
    );

    // setup oauth status listener
    let tx = setup_oauth_status_listener();

    // wait for the user to authorize the app
    let auth_res = tidal_client
        .wait_for_oauth(
            &oauth.device_code,
            oauth.expires_in,
            oauth.interval,
            Some(tx),
        )
        .await?;

    println!("auth response: {auth_res:?}");

    Ok(())
}

pub async fn handle_pkce_flow(tidal_client: &mut TidalClient) -> Result<(), TidalError> {
    let pkce_url = tidal_client.initiate_pkce_login()?;

    println!("visit this link to login: {pkce_url}");
    println!("after authenticating, paste the full redirect URL here and press enter:");

    let mut redirect_url = String::new();
    std::io::stdin()
        .read_line(&mut redirect_url)
        .map_err(|e| TidalError::Other(format!("failed to read redirect URL from stdin: {e}")))?;

    tidal_client.finish_pkce_login(redirect_url.trim()).await?;

    Ok(())
}
