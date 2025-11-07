use color_eyre::eyre::Result;
use tidlers::{
    auth::{init::TidalAuth, oauth::OAuthStatus},
    client::tidal::TidalClient,
};

use crate::save::{get_session_data, save_session_data};

mod save;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    // check for saved session data
    let saved_session_data = get_session_data();

    let mut auth: TidalAuth;

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

        // create a channel to receive oauth status updates (success, pending, etc)
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<OAuthStatus>();

        // spawn a task to listen for oauth status updates
        tokio::spawn(async move {
            loop {
                if let Some(status) = rx.recv().await {
                    println!("oauth status: {:?}", status);
                    if status == OAuthStatus::Success {
                        println!("you have successfully authorized the application!");
                        break;
                    }
                }
            }
        });

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

    // create tidal client
    let mut tidal = TidalClient::new(&auth);

    println!("logged in");
    println!("checking login..");
    println!(
        "status: {:?}",
        tidal.session.auth.check_login().await.is_ok()
    );
    println!("getting user info..");
    tidal.fetch_user_info().await?;

    println!("user info: {:#?}", tidal.user_info);

    Ok(())
}
