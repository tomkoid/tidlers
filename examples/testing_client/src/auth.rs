use tidlers::client::TidalClient;
use tidlers::{TidalError, auth::init::TidalAuth};

use crate::{oauth_handler::setup_oauth_status_listener, save::get_session_data};

pub async fn handle_auth() -> Option<TidalAuth> {
    // check for saved session data
    let saved_session_data = get_session_data();

    // if we have saved session data, load it, otherwise do oauth flow
    let auth = if saved_session_data.is_some() {
        println!("found saved session data");
        return None;
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
