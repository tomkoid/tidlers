use crate::{
    auth::init::TidalAuth,
    client::models::playback::{AudioQuality, PlaybackMode},
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

    #[serde(default = "default_playback_mode")]
    pub playback_mode: PlaybackMode,
}

fn default_locale() -> String {
    "en_US".to_string()
}

fn default_time_offset() -> String {
    "+00:00".to_string()
}

fn default_audio_quality() -> AudioQuality {
    AudioQuality::High
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
            playback_mode: PlaybackMode::Stream,
        }
    }
}
