use color_eyre::eyre::Result;
use tidlers::{
    auth::init::TidalAuth,
    client::{TidalClient, oauth::OAuthStatus},
};

use crate::save::{get_session_data, save_session_data};

mod save;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    // handle authentication and create Tidal client
    let mut tidal = if let Some(auth) = handle_auth().await? {
        TidalClient::new(&auth)
    } else {
        let saved_session_data = save::get_session_data().unwrap();
        let mut cl = TidalClient::from_json(&saved_session_data)?;

        let refreshed = cl.refresh_access_token(false).await?;
        if refreshed {
            println!("token refreshed from saved session data");
        } else {
            println!("using saved session data");
        }

        cl
    };

    if !tidal.session.auth.is_logged_in() {
        let oauth = tidal.get_oauth_link().await?;

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
        let auth_res = tidal
            .wait_for_oauth(
                &oauth.device_code,
                oauth.expires_in,
                oauth.interval,
                Some(tx),
            )
            .await?;

        println!("auth response: {auth_res:?}");

        // serialize and save session data
        let session_data = serde_json::to_string(&tidal.session.auth)?;
        save_session_data(&session_data);
    }

    println!("logged in");
    println!("checking login..");
    println!(
        "status: {:?}",
        tidal.session.auth.check_login().await.is_ok()
    );
    println!("getting user info..");
    tidal.refresh_user_info().await?;

    println!("user info: {:#?}", tidal.user_info);

    Ok(())
}

async fn handle_auth() -> Result<Option<TidalAuth>> {
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
