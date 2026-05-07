use crate::args::TrackCommands;
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    track_id: String,
    command: TrackCommands,
) -> eyre::Result<()> {
    match command {
        TrackCommands::Info { quality } => {
            let track_info = tidal.get_track(track_id.clone()).await?;
            let track_mix = tidal.get_track_mix(track_id.clone()).await?;

            println!("Track Info:\n{:#?}\n", track_info);
            println!("Track Mix:\n{:#?}\n", track_mix);

            tidal.set_audio_quality(quality.to_api_quality());
            let playback_info = tidal.get_track_postpaywall_playback_info(track_id).await?;
            println!("Playback Info:\n{:#?}", playback_info);
        }

        TrackCommands::Radio { limit, offset } => {
            let radio_items = tidal
                .get_track_radio(track_id, Some(limit), Some(offset))
                .await?;
            println!("Radio Items:\n{:#?}", radio_items);
        }
    };

    Ok(())
}
