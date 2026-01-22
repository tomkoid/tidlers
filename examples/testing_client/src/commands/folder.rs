use crate::args::FolderCommands;
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    command: FolderCommands,
) -> eyre::Result<()> {
    match command {
        FolderCommands::Create { name, parent_id } => {
            let folder = tidal.create_folder(name.clone(), None).await?;
            let parent = parent_id.as_deref().unwrap_or("root");
            println!("Created folder '{}' in '{}':\n{:#?}", name, parent, folder);
        }
    }
    
    Ok(())
}
