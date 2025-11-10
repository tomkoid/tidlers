use color_eyre::eyre::Result;
use tidlers::client::{
    models::playback::{AudioQuality, PlaybackMode},
    tidal::TidalClient,
};

use crate::{
    auth::handle_auth,
    save::{remove_session_data, save_session_data},
};

mod auth;
mod oauth_handler;
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
        let mut cl = TidalClient::from_serialized(&saved_session_data)?;

        let refreshed = cl.refresh_access_token(false).await?;
        if refreshed {
            println!("token refreshed from saved session data");
        } else {
            println!("using saved session data");
        }

        cl
    };

    // if waiting for oauth login, handle oauth flow
    if tidal.waiting_for_oauth_login() {
        println!("handling oauth flow..");
        auth::handle_oauth_flow(&mut tidal).await?;
        println!("oauth flow complete");
    }

    println!("logged in");
    println!("checking login..");
    println!(
        "status: {:?}",
        tidal.session.auth.check_login().await.is_ok()
    );

    println!("getting new user info..");
    tidal.refresh_user_info().await?;
    save_session_data(&tidal.get_json());

    println!("user info: {:#?}", tidal.user_info);

    println!("getting subscription info..");
    let subscription = tidal.subscription().await?;
    println!("subscription info: {:#?}", subscription);

    println!("getting new arrival mixes..");
    let am = tidal.get_arrival_mixes().await?;
    for mix in am.data {
        println!("mix: {} - id: {}", mix.data_type, mix.id);
    }

    let track_id = "66035607";
    println!("getting track info and track mix for track id..");
    let track_info = tidal.get_track(track_id.to_string()).await?;
    let track_mix = tidal.get_track_mix(track_id.to_string()).await?;
    println!("track info: {:#?}", track_info);
    println!("track mix: {:?}", track_mix);

    let playlist_uuid = "28a73f00-5988-4621-aaa4-966c6eaea651";
    println!("getting playlist info for playlist uuid..");
    let playlist_info = tidal.get_playlist(playlist_uuid.to_string()).await?;
    let playlist_items = tidal
        .get_playlist_items(playlist_uuid.to_string(), Some(10), Some(0))
        .await?;
    println!("playlist info: {:?}", playlist_info);
    println!("playlist items: {:#?}", playlist_items);

    let album_id = "341764695";
    println!("getting album info and items for album id..");
    let album_info = tidal.get_album(album_id.to_string()).await?;
    let album_items = tidal
        .get_album_items(album_id.to_string(), Some(10), Some(0))
        .await?;
    println!("album info: {:?}", album_info);
    println!("album items: {:#?}", album_items);

    println!("getting playback info for track id..");
    tidal.set_audio_quality(AudioQuality::HiRes);
    let playback_info = tidal
        .get_track_postpaywall_playback_info(track_id.to_string())
        .await?;
    println!("playback info: {:#?}", playback_info);

    println!("getting timeline..");
    let timeline = tidal.get_activity_timeline().await?;
    println!("timeline: {:#?}", timeline);

    println!("getting top artists..");
    let top_artists = tidal.get_activity_top_artists(2025, 11).await?;
    println!("top artists: {:#?}", top_artists);

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
