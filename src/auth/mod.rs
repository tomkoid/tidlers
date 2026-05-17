use serde::{Deserialize, Serialize};
use std::time::SystemTimeError;

use crate::{
    auth::{credentials::get_default_client_credentials, pkce::PkceConfig},
    requests,
    urls::API_V1_LOCATION,
};

pub mod credentials;
pub mod init;
pub mod pkce;
pub mod token;

/// Authentication credentials and configuration for the Tidal API client.
///
/// Returns a TidalAuth struct that can be used inside TidalClient::new(&TidalAuth)
/// If you want to use OAuth2 (recommended), use TidalAuth::with_oauth()
/// NOTE: TidalAuth::with_oauth() just enables OAuth2 login, you still need to get the OAuth link
/// and wait for the user to login using TidalClient::get_oauth_link() and
/// TidalClient::wait_for_oauth()
///
/// # Examples
///
/// ```no_run
/// # use tidlers::auth::TidalAuth;
/// // For OAuth2 login (recommended)
/// let auth = TidalAuth::with_oauth();
///
/// // For direct access token (if you already have one)
/// let auth = TidalAuth::with_access_token("your_token".to_string());
///
/// // For API token authentication (client credentials)
/// let auth = TidalAuth::with_api_token(
///     "client_id".to_string(),
///     "client_secret".to_string()
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TidalAuth {
    pub client_id: String,
    pub client_secret: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub refresh_expiry: Option<u64>,
    pub last_refresh_time: Option<u64>,
    pub client_name: Option<String>,
    pub user_id: Option<u64>,

    pub oauth_login: bool,
    pub api_token_auth: bool,
    pub pkce_login: bool,

    pub pkce_config: PkceConfig,

    #[serde(
        skip_serializing,
        skip_deserializing,
        default = "auth_default_request_client"
    )]
    pub rq: requests::RequestClient,
}
impl TidalAuth {
    /// Creates a new TidalAuth with default client credentials
    ///
    /// # Example
    ///
    /// ```
    /// # use tidlers::auth::TidalAuth;
    /// let auth = TidalAuth::new();
    /// ```
    pub fn new() -> Self {
        Self {
            api_token_auth: false,
            ..Default::default()
        }
    }

    /// Creates a TidalAuth configured for OAuth2 authentication flow
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tidlers::{TidalClient, auth::TidalAuth};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_oauth();
    /// let mut client = TidalClient::new(&auth);
    ///     
    /// let oauth = client.get_oauth_link().await?;
    /// println!("Visit: {}", oauth.verification_uri_complete);
    ///     
    /// client.wait_for_oauth(
    ///     &oauth.device_code,
    ///     oauth.expires_in,
    ///     oauth.interval,
    ///     None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_oauth() -> Self {
        Self {
            api_token_auth: false,
            oauth_login: true,
            ..Default::default()
        }
    }

    /// Creates a `TidalAuth` configured for PKCE-based OAuth2 authentication.
    ///
    /// Use this when you want to perform login through the browser-based PKCE flow.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tidlers::{TidalClient, auth::TidalAuth};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_pkce();
    /// let mut client = TidalClient::new(&auth);
    ///
    /// let login_url = client.initiate_pkce_login()?;
    /// println!("Visit: {}", login_url);
    ///
    /// // After redirect, pass the redirect URL you received:
    /// // client.finish_pkce_login("your_redirect_url").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_pkce() -> Self {
        Self {
            pkce_login: true,
            ..Default::default()
        }
    }

    /// Creates a TidalAuth with a pre-existing access token
    ///
    /// # Example
    ///
    /// ```
    /// # use tidlers::auth::TidalAuth;
    /// let auth = TidalAuth::with_access_token("your_access_token".to_string());
    /// ```
    pub fn with_access_token(access_token: String) -> Self {
        Self {
            access_token: Some(access_token),
            api_token_auth: false,
            ..Default::default()
        }
    }

    /// Creates a TidalAuth for API token (client credentials) authentication
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tidlers::{TidalClient, auth::TidalAuth};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_api_token(
    ///     "client_id".to_string(),
    ///     "client_secret".to_string()
    /// );
    /// let client = TidalClient::new(&auth);
    ///     
    /// // Get access token
    /// let token = client.session.auth.get_access_token().await?;
    /// println!("Access token: {}", token.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_api_token(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token: None,
            api_token_auth: true,
            ..Default::default()
        }
    }

    pub(crate) fn apply_access_token_state(
        &mut self,
        access_token: String,
        expires_in: u64,
        user_id: u64,
        client_name: Option<String>,
    ) -> Result<(), SystemTimeError> {
        self.access_token = Some(access_token);
        self.refresh_expiry = Some(expires_in);
        self.last_refresh_time = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        );
        self.user_id = Some(user_id);
        if let Some(client_name) = client_name {
            self.client_name = Some(client_name);
        }

        Ok(())
    }

    pub(crate) fn apply_oauth_token_state(
        &mut self,
        access_token: String,
        refresh_token: String,
        expires_in: u64,
        user_id: u64,
        client_name: Option<String>,
    ) -> Result<(), SystemTimeError> {
        self.refresh_token = Some(refresh_token);
        self.apply_access_token_state(access_token, expires_in, user_id, client_name)
    }
}

impl Default for TidalAuth {
    fn default() -> Self {
        let c_creds = get_default_client_credentials();
        let rq = auth_default_request_client();
        let pkce_config = PkceConfig::try_default().unwrap();

        Self {
            client_id: c_creds.0,
            client_secret: c_creds.1,
            client_name: None,
            access_token: None,
            refresh_token: None,
            refresh_expiry: None,
            last_refresh_time: None,
            user_id: None,
            api_token_auth: false,
            oauth_login: false,
            pkce_login: false,
            pkce_config,
            rq,
        }
    }
}

pub(crate) fn auth_default_request_client() -> requests::RequestClient {
    requests::RequestClient::new(API_V1_LOCATION.to_string())
}
