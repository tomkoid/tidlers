use crate::args::AlbumCommands;
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    album_id: String,
    command: AlbumCommands,
) -> eyre::Result<()> {
    match command {
        AlbumCommands::Info => {
            let info = tidal.get_album(album_id).await?;
            println!("{:#?}", info);
        }
        
        AlbumCommands::Items => {
            let items = tidal.get_album_items(album_id, Some(10), Some(0)).await?;
            println!("{:#?}", items);
        }
        
        AlbumCommands::Credits => {
            let credits = tidal.get_album_credits(album_id).await?;
            println!("{:#?}", credits);
        }
        
        AlbumCommands::ItemsCredits => {
            let items_credits = tidal.get_album_items_credits(album_id, None, None).await?;
            println!("{:#?}", items_credits);
        }
    }
    
    Ok(())
}
