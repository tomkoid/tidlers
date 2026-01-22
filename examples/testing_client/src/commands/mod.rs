mod activity;
mod album;
mod artist;
mod folder;
mod playlist;
mod search;
mod track;

use crate::{args::Commands, save::remove_session_data};
use tidlers::{TidalClient, client::models::track::config::UserUploadsInclude};

pub async fn execute_command(mut tidal: TidalClient, command: Commands) -> eyre::Result<()> {
    match command {
        Commands::UserInfo => {
            println!("{:#?}", tidal.user_info);
        }

        Commands::Playlists => {
            let favorites = tidal.get_collection_favorites(Some(50)).await?;
            println!("{:#?}", favorites);
        }

        Commands::Collection { command } => {
            execute_collection_command(&mut tidal, command).await?;
        }

        Commands::Activity { year, month } => {
            activity::execute(&mut tidal, year, month).await?;
        }

        Commands::Artist { artist_id, command } => {
            artist::execute(&mut tidal, artist_id, command).await?;
        }

        Commands::Subscription => {
            let subscription = tidal.subscription().await?;
            println!("{:#?}", subscription);
        }

        Commands::ArrivalMixes => {
            let mixes = tidal.get_arrival_mixes().await?;
            for mix in mixes.data {
                println!("{} - ID: {}", mix.data_type, mix.id);
            }
        }

        Commands::Uploads => {
            let uploads = tidal
                .get_user_uploads(UserUploadsInclude::default(), None)
                .await?;
            println!("{:#?}", uploads);
        }

        Commands::Track { track_id, quality } => {
            track::execute(&mut tidal, track_id, quality).await?;
        }

        Commands::Album { command, album_id } => {
            album::execute(&mut tidal, album_id, command).await?;
        }

        Commands::Logout => {
            if tidal.logout().await.is_ok() {
                remove_session_data();
                println!("Successfully logged out");
            } else {
                println!("Failed to logout");
            }
        }

        Commands::Home => {
            let home_feed = tidal.get_home_feed(20).await;
            println!("{:#?}", home_feed);
        }

        Commands::Search { command, query } => {
            search::execute(&mut tidal, query, command).await?;
        }
    }

    Ok(())
}

async fn execute_collection_command(
    tidal: &mut TidalClient,
    command: crate::args::CollectionCommands,
) -> eyre::Result<()> {
    use crate::args;

    match command {
        args::CollectionCommands::Artists => {
            let artists = tidal.get_collection_artists(50).await?;
            println!("{:#?}", artists);
        }

        args::CollectionCommands::Favorites => {
            let favorites = tidal.get_collection_favorites(Some(20)).await?;
            println!("{:#?}", favorites);
        }

        args::CollectionCommands::Playlist { command } => {
            playlist::execute(tidal, command).await?;
        }

        args::CollectionCommands::Folder { command } => {
            folder::execute(tidal, command).await?;
        }
    }

    Ok(())
}
