use crate::{
    auth::init::TidalAuth, client::models::track::AudioQuality, config::TidalConfig,
    requests::RequestClient,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalSession {
    pub config: TidalConfig,
    pub auth: TidalAuth,

    #[serde(default = "default_audio_quality")]
    pub audio_quality: AudioQuality,
}

fn default_audio_quality() -> AudioQuality {
    AudioQuality::High
}

impl TidalSession {
    pub fn new(credentials: &TidalAuth) -> TidalSession {
        let api_v1_location = "https://api.tidal.com/v1".to_string();
        TidalSession {
            config: TidalConfig::new(),
            auth: credentials.clone(),

            audio_quality: AudioQuality::High,
        }
    }
}
