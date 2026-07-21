use crate::{
    auth::TidalAuth,
    client::models::playback::{AudioQuality, PlaybackMode, VideoQuality},
};

/// Contains session configuration for a Tidal client
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalSession {
    pub auth: TidalAuth,

    #[serde(default = "default_locale")]
    pub locale: String,

    #[serde(default = "default_time_offset")]
    pub time_offset: String,

    #[serde(default = "default_audio_quality")]
    pub audio_quality: AudioQuality,

    #[serde(default = "default_video_quality")]
    pub video_quality: VideoQuality,

    #[serde(default = "default_playback_mode")]
    pub playback_mode: PlaybackMode,
}

fn default_locale() -> String {
    "en_US".to_string()
}

fn default_time_offset() -> String {
    chrono::Local::now().format("%:z").to_string()
}

fn default_audio_quality() -> AudioQuality {
    AudioQuality::High
}

fn default_video_quality() -> VideoQuality {
    VideoQuality::High
}

fn default_playback_mode() -> PlaybackMode {
    PlaybackMode::Stream
}

impl TidalSession {
    /// Creates a new session with the provided authentication credentials
    pub(crate) fn new(credentials: &TidalAuth) -> TidalSession {
        TidalSession {
            auth: credentials.clone(),
            locale: default_locale(),
            time_offset: default_time_offset(),

            audio_quality: AudioQuality::High,
            video_quality: VideoQuality::High,
            playback_mode: PlaybackMode::Stream,
        }
    }
}
