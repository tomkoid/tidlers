use crate::save::remove_session_data;
use clap::Parser;
use color_eyre::eyre::Result;
use tidlers::client::{TidalClient, models::playback::AudioQuality};

use crate::{args::Args, auth::handle_auth, save::save_session_data};

mod args;
mod auth;
mod oauth_handler;
mod save;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

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

    // if waiting for oauth login, handle oauth flow
    if tidal.waiting_for_oauth_login() {
        println!("handling oauth flow..");
        auth::handle_oauth_flow(&mut tidal).await?;
        println!("oauth flow complete");
    }

    println!("logged in");

    // refresh user info for all commands
    tidal.refresh_user_info().await?;
    save_session_data(&tidal.get_json());

    // execute command
    match args.command {
        args::Commands::UserInfo => {
            println!("user info: {:#?}", tidal.user_info);
        }

        args::Commands::Playlists => {
            println!("getting collection favorites (includes playlists)..");
            let favorites = tidal.get_collection_favorites(Some(50)).await?;
            println!("favorites: {:#?}", favorites);
        }

        args::Commands::Playlist { playlist_id } => {
            println!("getting playlist info for playlist uuid: {}..", playlist_id);
            let playlist_info = tidal.get_playlist(playlist_id.clone()).await?;
            let playlist_items = tidal
                .get_playlist_items(playlist_id, Some(10), Some(0))
                .await?;
            println!("playlist info: {:?}", playlist_info);
            println!("playlist items: {:#?}", playlist_items);
        }

        args::Commands::Collection => {
            println!("getting collection artists..");
            let collection_artists = tidal.get_collection_artists(50).await?;
            println!("collection artists: {:#?}", collection_artists);

            println!("getting collection favorites..");
            let collection_favorites = tidal.get_collection_favorites(Some(20)).await?;
            println!("collection favorites: {:#?}", collection_favorites);
        }

        args::Commands::Activity => {
            println!("getting timeline..");
            let timeline = tidal.get_activity_timeline().await?;
            println!("timeline: {:#?}", timeline);

            println!("getting top artists..");
            let top_artists = tidal.get_activity_top_artists(2025, 11).await?;
            println!("top artists: {:#?}", top_artists);
        }

        args::Commands::Subscription => {
            println!("getting subscription info..");
            let subscription = tidal.subscription().await?;
            println!("subscription info: {:#?}", subscription);
        }

        args::Commands::ArrivalMixes => {
            println!("getting new arrival mixes..");
            let am = tidal.get_arrival_mixes().await?;
            for mix in am.data {
                println!("mix: {} - id: {}", mix.data_type, mix.id);
            }
        }

        args::Commands::Track { track_id } => {
            println!("getting track info and track mix for track id: {}..", track_id);
            let track_info = tidal.get_track(track_id.clone()).await?;
            let track_mix = tidal.get_track_mix(track_id.clone()).await?;
            println!("track info: {:#?}", track_info);
            println!("track mix: {:?}", track_mix);

            println!("getting playback info for track id..");
            tidal.set_audio_quality(AudioQuality::HiRes);
            let playback_info = tidal
                .get_track_postpaywall_playback_info(track_id)
                .await?;
            println!("playback info: {:#?}", playback_info);
        }

        args::Commands::Album { album_id } => {
            println!("getting album info and items for album id: {}..", album_id);
            let album_info = tidal.get_album(album_id.clone()).await?;
            let album_items = tidal
                .get_album_items(album_id, Some(10), Some(0))
                .await?;
            println!("album info: {:?}", album_info);
            println!("album items: {:#?}", album_items);
        }

        args::Commands::Logout => {
            println!("trying to logout..");
            let logout = tidal.logout().await;
            if logout.is_ok() {
                println!("successfully logged out!");
                remove_session_data();
            } else {
                println!("failed to logout: {:?}", logout.err());
            }
        }
    }

    Ok(())
}
