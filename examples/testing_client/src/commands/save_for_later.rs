use tidlers::{
    TidalClient, client::models::collection::save_for_later::config::GetSaveForLaterItemsConfig,
};

use crate::args::SaveForLaterCommands;

pub async fn execute(tidal: &mut TidalClient, command: SaveForLaterCommands) -> eyre::Result<()> {
    match command {
        SaveForLaterCommands::Add { id, resource_type } => {
            let response = tidal
                .add_save_for_later_items(id, resource_type.to_resource_type())
                .await?;
            println!("{:#?}", response);
        }
        SaveForLaterCommands::List => {
            let items = tidal
                .get_save_for_later_items(GetSaveForLaterItemsConfig::default())
                .await?;
            println!("{:#?}", items);
        }
    }

    Ok(())
}
