use crate::{
    auth::init::TidalAuth,
    client::models::playback::{AudioQuality, PlaybackMode},
    config::TidalConfig,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalSession {
    pub config: TidalConfig,
    pub auth: TidalAuth,

    #[serde(default = "default_audio_quality")]
    pub audio_quality: AudioQuality,

    #[serde(default = "default_playback_mode")]
    pub playback_mode: PlaybackMode,
}

fn default_audio_quality() -> AudioQuality {
    AudioQuality::High
}

fn default_playback_mode() -> PlaybackMode {
    PlaybackMode::Stream
}

impl TidalSession {
    pub fn new(credentials: &TidalAuth) -> TidalSession {
        let api_v1_location = "https://api.tidal.com/v1".to_string();
        TidalSession {
            config: TidalConfig::new(),
            auth: credentials.clone(),

            audio_quality: AudioQuality::High,
            playback_mode: PlaybackMode::Stream,
        }
    }
}
