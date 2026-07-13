use tidlers::{TidalClient, client::models::video::config::VideoPlaybackInfoConfig};

use crate::args::VideoCommands;

pub async fn execute(
    tidal: &mut TidalClient,
    video_id: String,
    command: VideoCommands,
) -> eyre::Result<()> {
    match command {
        VideoCommands::Info { quality } => {
            tidal.set_video_quality(quality.to_api_quality());
            let video_info = tidal.get_video(video_id.clone()).await?;

            println!("Video Info:\n{:#?}\n", video_info);

            let playback_info = tidal
                .get_video_postpaywall_playback_info(
                    video_id,
                    Some(VideoPlaybackInfoConfig {
                        video_quality: Some(quality.to_api_quality()),
                        ..Default::default()
                    }),
                )
                .await?;
            println!("Playback Info:\n{:#?}", playback_info);
        }
    };

    Ok(())
}
