use color_eyre::eyre::Result;
use tidlers::client::tidal::TidalClient;

use crate::{auth::handle_auth, save::remove_session_data};

mod auth;
mod oauth_handler;
mod save;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    // handle authentication
    let auth = handle_auth().await?;

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

    println!("getting subscription info..");
    let subscription = tidal.subscription().await?;
    println!("subscription info: {:#?}", subscription);

    println!("getting new arrival mixes..");
    let am = tidal.get_arrival_mixes().await?;
    for mix in am.data {
        println!("mix: {} - id: {}", mix.data_type, mix.id);
    }

    let track_id = "456686484";
    println!("getting track info and track mix for track id..");
    let track_info = tidal.get_track(track_id.to_string()).await?;
    let track_mix = tidal.get_track_mix(track_id.to_string()).await?;
    println!("track info: {:#?}", track_info);
    println!("track mix: {:?}", track_mix);

    // println!("trying to logout..");
    // let logout = tidal.logout().await;
    // if logout.is_ok() {
    //     println!("successfully logged out!");
    //
    //     // invalidate saved session data
    //     remove_session_data();
    // } else {
    //     println!("failed to logout: {:?}", logout.err());
    // }

    Ok(())
}
