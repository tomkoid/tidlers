pub mod api;
pub mod auth;
pub mod data;
pub mod models;
pub mod oauth;

use crate::{
    auth::TidalAuth,
    client::models::{
        playback::{AudioQuality, PlaybackMode},
        user::User,
    },
    error::TidalError,
    requests::{self, RequestClient},
    session::TidalSession,
    urls::API_V1_LOCATION,
};

/// Main client for interacting with the Tidal API
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TidalClient {
    pub user_info: Option<User>,

    pub session: TidalSession,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) rq: requests::RequestClient,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) debug_mode: bool,
}

impl TidalClient {
    /// Creates a new TidalClient with the provided authentication credentials
    ///
    /// # Example
    ///
    /// ```
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// let auth = TidalAuth::with_oauth();
    /// let client = TidalClient::new(&auth);
    /// ```
    pub fn new(credentials: &TidalAuth) -> TidalClient {
        let session = TidalSession::new(credentials);
        let rq = RequestClient::new(API_V1_LOCATION.to_string());
        TidalClient {
            user_info: None,
            session,
            rq,
            debug_mode: false,
        }
    }

    /// Checks if the client is waiting for OAuth login completion
    pub fn waiting_for_oauth_login(&self) -> bool {
        self.session.auth.oauth_login && self.session.auth.access_token.is_none()
    }

    fn check_auth(&self) -> Result<bool, TidalError> {
        if self.session.auth.access_token.is_none() {
            Err(TidalError::NotAuthenticated)
        } else {
            Ok(true)
        }
    }

    pub(crate) fn user_id(&self) -> Result<u64, TidalError> {
        self.user_info
            .as_ref()
            .map(|u| u.user_id)
            .ok_or(TidalError::NotAuthenticated)
    }

    /// Sets the audio quality preference for playback
    ///
    /// # Example
    ///
    /// ```
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # use tidlers::client::models::playback::AudioQuality;
    /// let auth = TidalAuth::with_oauth();
    /// let mut client = TidalClient::new(&auth);
    /// client.set_audio_quality(AudioQuality::HiRes);
    /// ```
    pub fn set_audio_quality(&mut self, quality: AudioQuality) {
        self.session.audio_quality = quality;
    }

    /// Sets the time offset for the session
    pub fn set_time_offset(&mut self, time_offset: String) {
        self.session.time_offset = time_offset;
    }

    /// Sets the playback mode (stream or offline)
    pub fn set_playback_mode(&mut self, playback_mode: PlaybackMode) {
        self.session.playback_mode = playback_mode;
    }

    /// Enables or disables debug mode for verbose logging
    pub fn set_debug_mode(&mut self, debug_mode: bool) {
        tracing::debug!(enabled = debug_mode, "setting client debug mode");
        self.debug_mode = debug_mode;
    }

    pub async fn home(&self) -> Result<(), TidalError> {
        self.check_auth()?;

        // TODO: Implement home page functionality
        // self.page.r_get("pages/home");
        Ok(())
        // Ok(self.page.get("pages/home").await?)
    }
}

impl Default for TidalClient {
    fn default() -> Self {
        Self::new(&TidalAuth::new())
    }
}
