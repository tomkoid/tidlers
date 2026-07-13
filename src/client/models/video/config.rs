use crate::client::models::playback::{AssetPresentation, PlaybackMode, VideoQuality};

#[derive(Clone, Default)]
pub struct VideoPlaybackInfoConfig {
    pub video_quality: Option<VideoQuality>,
    pub playback_mode: Option<PlaybackMode>,
    pub asset_presentation: Option<AssetPresentation>,
}
