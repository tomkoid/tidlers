use serde::{Deserialize, Serialize};

use crate::{auth::credentials::get_default_client_credentials, requests, urls::API_V1_LOCATION};

pub mod credentials;
pub mod init;
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
pub struct TidalAuth {
    pub client_id: String,
    pub client_secret: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub refresh_expiry: Option<u64>,
    pub last_refresh_time: Option<u64>,
    pub user_id: Option<u64>,

    pub oauth_login: bool,

    #[serde(
        skip_serializing,
        skip_deserializing,
        default = "auth_default_request_client"
    )]
    pub rq: requests::RequestClient,

    pub api_token_auth: bool,
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
}

impl Default for TidalAuth {
    fn default() -> Self {
        let c_creds = get_default_client_credentials();
        let rq = auth_default_request_client();
        Self {
            client_id: c_creds.0,
            client_secret: c_creds.1,
            access_token: None,
            refresh_token: None,
            refresh_expiry: None,
            last_refresh_time: None,
            user_id: None,
            api_token_auth: false,
            oauth_login: false,
            rq,
        }
    }
}

pub(crate) fn auth_default_request_client() -> requests::RequestClient {
    requests::RequestClient::new(API_V1_LOCATION.to_string())
}
