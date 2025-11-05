use crate::{auth::init::TidalAuth, config::TidalConfig, requests::RequestClient};

#[derive(Debug, Clone)]
pub struct TidalSession {
    pub rq: RequestClient,
    pub config: TidalConfig,
    pub auth: TidalAuth,
}

impl TidalSession {
    pub fn new(credentials: &TidalAuth) -> TidalSession {
        let api_v1_location = "https://api.tidal.com/v1/".to_string();
        TidalSession {
            rq: RequestClient::new(api_v1_location),
            config: TidalConfig::new(),
            auth: credentials.clone(),
        }
    }
}
