mod activity;
mod album;
mod artist;
mod folder;
mod mix;
mod playlist;
mod search;
mod track;

use crate::{args::Commands, save::remove_session_data};
use tidlers::{TidalClient, client::models::track::config::UserUploadsIncludeOptions};

pub async fn execute_command(mut tidal: TidalClient, command: Commands) -> eyre::Result<()> {
    match command {
        Commands::UserInfo => {
            println!("{:#?}", tidal.user_info);
        }

        Commands::User { user_id } => {
            let user_v1 = tidal.get_user_v1(user_id.clone()).await?;
            println!("v1: {:#?}", user_v1);

            let user_v2 = tidal.get_user_v2(user_id).await?;
            println!("v2: {:#?}", user_v2);
        }

        Commands::Playlists => {
            let playlists = tidal.list_playlists().await?;
            println!("{:#?}", playlists);
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

        Commands::Mix { command } => {
            mix::execute(&mut tidal, command).await?;
        }

        Commands::Uploads => {
            let uploads = tidal
                .get_user_uploads(UserUploadsIncludeOptions::default(), None)
                .await?;
            println!("{:#?}", uploads);
        }

        Commands::Track { track_id, command } => {
            track::execute(&mut tidal, track_id, command).await?;
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

        args::CollectionCommands::Tracks { limit, offset } => {
            let favorites = tidal
                .get_collection_track_favorites(Some(limit), Some(offset))
                .await?;
            println!("{:#?}", favorites);
        }

        args::CollectionCommands::Albums { limit, offset } => {
            let favorites = tidal
                .get_collection_album_favorites(Some(limit), Some(offset))
                .await?;
            println!("{:#?}", favorites);
        }

        args::CollectionCommands::Playlist { command } => {
            playlist::execute(tidal, command).await?;
        }

        args::CollectionCommands::Favorite {
            resource_type,
            id,
            remove,
        } => {
            match remove {
                true => {
                    tidal
                        .remove_from_favorites(
                            resource_type.to_favorite_resource_type(),
                            id.parse()?,
                        )
                        .await?;
                    println!("Removed from favorites");
                }
                false => {
                    // continue to add to favorites
                    tidal
                        .add_to_favorites(resource_type.to_favorite_resource_type(), id.parse()?)
                        .await?;
                    println!("Added to favorites");
                }
            }

            return Ok(());
        }

        args::CollectionCommands::Folder { command } => {
            folder::execute(tidal, command).await?;
        }
    }

    Ok(())
}
