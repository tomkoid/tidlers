use tidlers::TidalClient;

use crate::args::VideoCommands;

pub async fn execute(
    tidal: &mut TidalClient,
    track_id: String,
    command: VideoCommands,
) -> eyre::Result<()> {
    match command {
        VideoCommands::Info { quality } => {
            tidal.set_video_quality(quality.to_api_quality());
            let video_info = tidal.get_video(track_id.clone()).await?;

            println!("Video Info:\n{:#?}\n", video_info);

            let playback_info = tidal.get_video_postpaywall_playback_info(track_id).await?;
            println!("Playback Info:\n{:#?}", playback_info);
        }
    };

    Ok(())
}
