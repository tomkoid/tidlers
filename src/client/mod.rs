pub mod api;
pub mod auth;
pub mod data;
pub mod models;
pub mod oauth;

use crate::{
    auth::init::TidalAuth,
    client::models::{
        playback::{AudioQuality, PlaybackMode},
        user::User,
    },
    error::TidalError,
    page::TidalPage,
    requests::{self, RequestClient},
    session::TidalSession,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalClient {
    pub user_info: Option<User>,

    pub session: TidalSession,
    pub page: TidalPage,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) rq: requests::RequestClient,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) debug_mode: bool,
}

impl TidalClient {
    pub const API_V1_LOCATION: &'static str = "https://api.tidal.com/v1";
    pub const API_V2_LOCATION: &'static str = "https://api.tidal.com/v2";
    pub const OPEN_API_V2_LOCATION: &'static str = "https://openapi.tidal.com/v2";

    pub fn new(credentials: &TidalAuth) -> TidalClient {
        let session = TidalSession::new(credentials);
        let rq = RequestClient::new(Self::API_V1_LOCATION.to_string());
        TidalClient {
            user_info: None,
            session,
            page: TidalPage::new(),
            rq,
            debug_mode: false,
        }
    }

    pub fn waiting_for_oauth_login(&self) -> bool {
        self.session.auth.oauth_login && self.session.auth.access_token.is_none()
    }

    fn check_auth(&self) -> Result<bool, TidalError> {
        if self.session.auth.access_token.is_none() || !self.page.is_access_token_set() {
            Err(TidalError::NotAuthenticated)
        } else {
            Ok(true)
        }
    }

    pub fn set_audio_quality(&mut self, quality: AudioQuality) {
        self.session.audio_quality = quality;
    }

    pub fn set_playback_mode(&mut self, playback_mode: PlaybackMode) {
        self.session.playback_mode = playback_mode;
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug_mode = debug;
    }

    pub async fn home(&self) -> Result<(), TidalError> {
        self.check_auth()?;

        println!("home");
        self.page.r_get("pages/home");
        Ok(())
        // Ok(self.page.get("pages/home").await?)
    }
}

impl Default for TidalClient {
    fn default() -> Self {
        Self::new(&TidalAuth::new())
    }
}
