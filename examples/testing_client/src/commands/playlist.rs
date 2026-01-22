use crate::args::{ArgSharingLevel, PlaylistCommands};
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    command: PlaylistCommands,
) -> eyre::Result<()> {
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
            let items = tidal.get_playlist_items(playlist_id, Some(10), Some(0)).await?;
            println!("{:#?}", items);
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
