use color_eyre::eyre::Result;
use tidlers::auth::init::TidalAuth;

use crate::{
    oauth_handler::setup_oauth_status_listener,
    save::{get_session_data, save_session_data},
};

pub async fn handle_auth() -> Result<TidalAuth> {
    let mut auth: TidalAuth;

    // check for saved session data
    let saved_session_data = get_session_data();

    // if we have saved session data, load it, otherwise do oauth flow
    if saved_session_data.is_some() {
        println!("found saved session data, loading...");
        auth = TidalAuth::from_serialized(&saved_session_data.unwrap())?;
    } else {
        auth = TidalAuth::new();
        let oauth = auth.get_oauth_link().await?;

        println!(
            "visit this link to login: https://{}",
            oauth.verification_uri_complete
        );

        // setup oauth status listener
        let tx = setup_oauth_status_listener();

        // wait for the user to authorize the app
        let auth_res = auth
            .wait_for_oauth(
                &oauth.device_code,
                oauth.expires_in,
                oauth.interval,
                Some(tx),
            )
            .await?;

        println!("auth response: {auth_res:?}");

        // serialize and save session data
        let session_data = auth.get_auth_json();
        save_session_data(&session_data);
    }

    Ok(auth)
}
