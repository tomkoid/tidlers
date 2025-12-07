use color_eyre::eyre::Result;
use tidlers::auth::init::TidalAuth;
use tidlers::client::TidalClient;

use crate::{oauth_handler::setup_oauth_status_listener, save::get_session_data};

pub async fn handle_auth() -> Result<Option<TidalAuth>> {
    let auth: TidalAuth;

    // check for saved session data
    let saved_session_data = get_session_data();

    // if we have saved session data, load it, otherwise do oauth flow
    if saved_session_data.is_some() {
        println!("found saved session data");
        return Ok(None);
    } else {
        auth = TidalAuth::with_oauth();
    }

    Ok(Some(auth))
}

pub async fn handle_oauth_flow(tidal_client: &mut TidalClient) -> Result<()> {
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
