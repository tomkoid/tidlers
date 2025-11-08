use crate::{auth::init::TidalAuth, config::TidalConfig, requests::RequestClient};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalSession {
    pub config: TidalConfig,
    pub auth: TidalAuth,
}

impl TidalSession {
    pub fn new(credentials: &TidalAuth) -> TidalSession {
        let api_v1_location = "https://api.tidal.com/v1".to_string();
        TidalSession {
            config: TidalConfig::new(),
            auth: credentials.clone(),
        }
    }
}
