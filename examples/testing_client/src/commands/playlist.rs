use crate::args::{ArgSharingLevel, PlaylistCommands};
use tidlers::{
    TidalClient,
    client::models::{OrderDirection, playlist::PlaylistItemsOrder},
};

pub async fn execute(tidal: &mut TidalClient, command: PlaylistCommands) -> eyre::Result<()> {
    match command {
        PlaylistCommands::Create {
            name,
            description,
            folder_id,
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

            let playlist = tidal
                .create_playlist(name.clone(), description, Some(sharing_level), folder_id)
                .await?;
            println!("Created playlist '{}':\n{:#?}", name, playlist);
        }

        PlaylistCommands::Info { playlist_id } => {
            let info = tidal.get_playlist(playlist_id).await?;
            println!("{:#?}", info);
        }

        PlaylistCommands::Items { playlist_id } => {
            let items = tidal
                .get_playlist_items(playlist_id, Some(10), Some(0), None, None)
                .await?;
            println!("{:#?}", items);
        }

        PlaylistCommands::AddItems {
            playlist_id,
            item_ids,
        } => {
            let item_ids: Vec<String> = item_ids
                .split(',')
                .map(|id| id.trim().parse())
                .collect::<Result<_, _>>()?;

            let playlist_items = tidal
                .get_playlist_items_with_etag(playlist_id.clone(), Some(1), None, None, None)
                .await?;
            let total_nr_items = playlist_items.items.total_number_of_items;

            tidal
                .add_items_to_playlist_with_etag(
                    playlist_id,
                    item_ids,
                    Some(total_nr_items),
                    &playlist_items.etag,
                )
                .await?;
            println!("Added items to playlist");
        }

        PlaylistCommands::RemoveItems {
            playlist_id,
            indices,
        } => {
            let playlist_items = tidal
                .get_playlist_items_with_etag(playlist_id.clone(), Some(1), None, None, None)
                .await?;

            tidal
                .remove_items_from_playlist_with_etag(
                    playlist_id,
                    indices.clone(),
                    Some(PlaylistItemsOrder::Index),
                    Some(OrderDirection::Ascending),
                    &playlist_items.etag,
                )
                .await?;
            println!(
                "Removed items with indices {} from playlist",
                indices
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
        }

        PlaylistCommands::List => {
            let playlists = tidal.list_playlists().await?;
            println!("{:#?}", playlists);
        }

        PlaylistCommands::ListPublic => {
            let playlists = tidal.list_public_playlists(None, None).await?;
            println!("{:#?}", playlists);
        }

        PlaylistCommands::RecommendationsItems { playlist_id } => {
            let items = tidal
                .get_playlist_recommendations_items(playlist_id, Some(10), Some(0))
                .await?;
            println!("{:#?}", items);
        }
    }

    Ok(())
}
