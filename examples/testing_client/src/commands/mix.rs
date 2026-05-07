use tidlers::TidalClient;

use crate::args::MixCommands;

pub async fn execute(client: &TidalClient, command: MixCommands) -> eyre::Result<()> {
    match command {
        MixCommands::Items {
            mix_id,
            limit,
            offset,
        } => {
            let tracks = client
                .get_mix_tracks(mix_id, Some(limit), Some(offset))
                .await?;

            println!("{:#?}", tracks);
        }
        MixCommands::ArrivalMixes => {
            let mixes = client.get_arrival_mixes().await?;
            println!("{:#?}", mixes);
        }
    }

    Ok(())
}
