use crate::args::FolderCommands;
use tidlers::{
    TidalClient,
    client::models::collection::folder::{FolderOrder, FolderOrderDirection},
};

pub async fn execute(tidal: &mut TidalClient, command: FolderCommands) -> eyre::Result<()> {
    match command {
        FolderCommands::Create { name, parent_id } => {
            let folder = tidal.create_folder(name.clone(), None).await?;
            let parent = parent_id.as_deref().unwrap_or("root");
            println!("Created folder '{}' in '{}':\n{:#?}", name, parent, folder);
        }

        FolderCommands::Remove { id } => {
            tidal.remove_folder(&id).await?;
            println!("Removed folder '{}'", id);
        }

        FolderCommands::Flattened => {
            let flattened_folders = tidal
                .flattened_folders(
                    None,
                    None,
                    Some(FolderOrder::Date),
                    Some(FolderOrderDirection::Ascending),
                )
                .await?;
            println!("Flattened folders: {:#?}", flattened_folders);
        }
    }

    Ok(())
}
