use crate::{
    args::{self, ArgSharingLevel, Commands},
    save::remove_session_data,
};
use chrono::Datelike;
use tidlers::{
    TidalClient,
    client::models::search::config::{SearchConfig, SearchSuggestionsConfig},
};

pub async fn execute_command(mut tidal: TidalClient, command: Commands) -> eyre::Result<()> {
    match command {
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
                args::PlaylistCommands::RecommendationsItems { playlist_id } => {
                    println!(
                        "getting playlist recommendation items for playlist uuid: {}..",
                        playlist_id
                    );
                    let recommendation_items = tidal
                        .get_playlist_recommendations_items(playlist_id, Some(10), Some(0))
                        .await?;
                    println!("playlist recommendation items: {:#?}", recommendation_items);
                }
            },
            args::CollectionCommands::Folder { command } => match command {
                args::FolderCommands::Create { name, parent_id } => {
                    println!(
                        "creating folder with name: {} in parent id: {}..",
                        name,
                        parent_id.as_deref().unwrap_or("root")
                    );
                    let folder = tidal.create_folder(name, None).await?;
                    println!("created folder: {:#?}", folder);
                }
            },
        },

        args::Commands::Activity { year, month } => {
            let now = chrono::Utc::now();
            let year = year.unwrap_or(now.year());
            let month = month.unwrap_or(now.month());

            if !(1..=12).contains(&month) {
                eprintln!("invalid month: {}. must be between 1 and 12", month);
                return Ok(());
            }

            println!("getting timeline..");
            let timeline = tidal.get_activity_timeline().await?;
            println!("timeline: {:#?}", timeline);

            println!("getting top artists..");
            let top_artists = tidal.get_activity_top_artists(year, month).await?;
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

        args::Commands::Uploads => {
            println!("getting user uploads..");
            let uu = tidal.get_user_uploads(None).await?;
            println!("user uploads: {uu:#?}");
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

        args::Commands::Home => {
            println!("getting home feed..");
            let hf = tidal.get_home_feed(20).await;
            println!("home feed: {hf:#?}");
        }

        args::Commands::Search { command, query } => match command {
            args::SearchCommands::Direct => {
                println!("searching for query {query}..");
                let results = tidal
                    .search(SearchConfig {
                        query: query.clone(),
                        limit: 10,
                        ..Default::default()
                    })
                    .await?;
                println!("results: {results:?}")
            }

            args::SearchCommands::Suggestions => {
                println!("getting search suggestions for query {query}..");
                let results = tidal
                    .search_suggestion(SearchSuggestionsConfig {
                        query: query.clone(),
                        ..Default::default()
                    })
                    .await?;
                println!("results: {results:?}")
            }
        },
    }

    Ok(())
}
