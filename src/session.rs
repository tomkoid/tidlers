use crate::{
    auth::init::TidalAuth, client::models::track::AudioQuality, config::TidalConfig,
    requests::RequestClient,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalSession {
    pub config: TidalConfig,
    pub auth: TidalAuth,

    pub audio_quality: AudioQuality,
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
