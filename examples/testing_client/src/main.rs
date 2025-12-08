use crate::{args::ArgSharingLevel, save::remove_session_data};
use clap::Parser;
use color_eyre::eyre::Result;
use tidlers::client::TidalClient;

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

    tidal.set_debug_mode(args.debug);

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

        args::Commands::Collection { command } => match command {
            args::CollectionCommands::Artists => {
                println!("getting collection artists..");
                let collection_artists = tidal.get_collection_artists(50).await?;
                println!("collection artists: {:#?}", collection_artists);
            }
            args::CollectionCommands::Favorites => {
                println!("getting collection favorites..");
                let collection_favorites = tidal.get_collection_favorites(Some(20)).await?;
                println!("collection favorites: {:#?}", collection_favorites);
            }
            args::CollectionCommands::Playlist { command } => match command {
                args::PlaylistCommands::Create {
                    description,
                    folder_id,
                    name,
                    sharing_level,
                } => {
                    let sharing_level = match sharing_level.unwrap_or(ArgSharingLevel::Private) {
                        ArgSharingLevel::Private => {
                            tidlers::client::models::collection::SharingLevel::Private
                        }
                        ArgSharingLevel::Public => {
                            tidlers::client::models::collection::SharingLevel::Public
                        }
                    };
                    println!("creating playlist with title: {}..", name);
                    let playlist = tidal
                        .create_playlist(name, description, Some(sharing_level), folder_id)
                        .await?;
                    println!("created playlist: {:#?}", playlist);
                }
                args::PlaylistCommands::Info { playlist_id } => {
                    println!("getting playlist info for playlist uuid: {}..", playlist_id);
                    let playlist_info = tidal.get_playlist(playlist_id.clone()).await?;
                    println!("playlist info: {:?}", playlist_info);
                }
                args::PlaylistCommands::Items { playlist_id } => {
                    println!(
                        "getting playlist items for playlist uuid: {}..",
                        playlist_id
                    );
                    let playlist_items = tidal
                        .get_playlist_items(playlist_id, Some(10), Some(0))
                        .await?;
                    println!("playlist items: {:#?}", playlist_items);
                }
                args::PlaylistCommands::List => {
                    println!("listing playlists..");
                    let playlists = tidal.list_playlists().await?;
                    println!("playlists: {:#?}", playlists);
                }
                args::PlaylistCommands::ListPublic => {
                    println!("listing public playlists..");
                    let playlists = tidal.list_public_playlists(None, None).await?;
                    println!("playlists: {:#?}", playlists);
                }
            },
            args::CollectionCommands::Folder { command } => match command {
                args::FolderCommands::Create { name, parent_id } => {
                    println!(
                        "creating folder with name: {} in parent id: {}..",
                        name,
                        parent_id.as_ref().map(|s| s.as_str()).unwrap_or("root")
                    );
                    let folder = tidal.create_folder(name, None).await?;
                    println!("created folder: {:#?}", folder);
                }
            },
        },

        args::Commands::Activity => {
            println!("getting timeline..");
            let timeline = tidal.get_activity_timeline().await?;
            println!("timeline: {:#?}", timeline);

            println!("getting top artists..");
            let top_artists = tidal.get_activity_top_artists(2025, 11).await?;
            println!("top artists: {:#?}", top_artists);
        }

        args::Commands::Artist { artist_id, command } => match command {
            args::ArtistCommands::Info => {
                println!("getting artist info for artist id: {}..", artist_id);
                let artist_info = tidal.get_artist(artist_id.clone()).await?;
                println!("artist info: {:#?}", artist_info);
            }
            args::ArtistCommands::TopTracks => {
                println!("getting artist tracks for artist id: {}..", artist_id);
                let artist_tracks = tidal.get_artist_tracks(artist_id, Some(2), None).await?;
                println!("artist tracks: {:#?}", artist_tracks);
            }
            args::ArtistCommands::Bio => {
                println!("getting artist bio for artist id: {}..", artist_id);
                let artist_bio = tidal.get_artist_bio(artist_id).await?;
                println!("artist bio: {:#?}", artist_bio);
            }
            args::ArtistCommands::Links => {
                println!("getting artist links for artist id: {}..", artist_id);
                let artist_links = tidal.get_artist_links(artist_id).await?;
                println!("artist links: {:#?}", artist_links);
            }

            args::ArtistCommands::Albums => {
                println!("getting artist albums for artist id: {}..", artist_id);
                let artist_albums = tidal.get_artist_albums(artist_id, None, None).await?;
                println!("artist albums: {:#?}", artist_albums);
            }

            args::ArtistCommands::Mix => {
                println!("getting artist mix for artist id: {}..", artist_id);
                let artist_mix = tidal.get_artist_mix(artist_id).await?;
                println!("artist mix: {:#?}", artist_mix);
            }

            args::ArtistCommands::Videos => {
                println!("getting artist videos for artist id: {}..", artist_id);
                let artist_videos = tidal.get_artist_videos(artist_id, None, None).await?;
                println!("artist videos: {:#?}", artist_videos);
            }
        },

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

        args::Commands::Track { track_id, quality } => {
            println!(
                "getting track info and track mix for track id: {}..",
                track_id
            );
            let track_info = tidal.get_track(track_id.clone()).await?;
            let track_mix = tidal.get_track_mix(track_id.clone()).await?;
            println!("track info: {:#?}", track_info);
            println!("track mix: {:?}", track_mix);

            println!("getting playback info for track id..");
            tidal.set_audio_quality(quality.to_api_quality());
            let playback_info = tidal.get_track_postpaywall_playback_info(track_id).await?;
            println!("playback info: {:#?}", playback_info);
        }

        args::Commands::Album { command, album_id } => match command {
            args::AlbumCommands::Info => {
                println!("getting album info for album id: {}..", album_id);
                let album_info = tidal.get_album(album_id.clone()).await?;
                println!("album info: {:?}", album_info);
            }
            args::AlbumCommands::Items => {
                println!("getting album items for album id: {}..", album_id);
                let album_items = tidal
                    .get_album_items(album_id.clone(), Some(10), Some(0))
                    .await?;
                println!("album items: {:#?}", album_items);
            }
            args::AlbumCommands::Credits => {
                println!("getting album credits for album id: {}..", album_id);
                let album_credits = tidal.get_album_credits(album_id.clone()).await?;
                println!("album credits: {:#?}", album_credits);
            }
            args::AlbumCommands::ItemsCredits => {
                println!(
                    "getting album items with credits for album id: {}..",
                    album_id
                );
                let album_items_credits =
                    tidal.get_album_items_credits(album_id, None, None).await?;
                println!("album items with credits: {:#?}", album_items_credits);
            }
        },

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
