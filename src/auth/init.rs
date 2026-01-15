use std::{collections::HashMap, time::SystemTime};

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    auth::client_credentials::get_client_credentials,
    error::TidalError,
    requests::{self, TidalRequest},
    responses::AccessTokenResponse,
};

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
/// use tidlers::auth::init::TidalAuth;
///
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
    /// use tidlers::auth::init::TidalAuth;
    ///
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
    /// use tidlers::{TidalClient, auth::init::TidalAuth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let auth = TidalAuth::with_oauth();
    ///     let mut client = TidalClient::new(&auth);
    ///     
    ///     let oauth = client.get_oauth_link().await.unwrap();
    ///     println!("Visit: {}", oauth.verification_uri_complete);
    ///     
    ///     client.wait_for_oauth(
    ///         &oauth.device_code,
    ///         oauth.expires_in,
    ///         oauth.interval,
    ///         None
    ///     ).await.unwrap();
    /// }
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
    /// use tidlers::auth::init::TidalAuth;
    ///
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
    /// use tidlers::{TidalClient, auth::init::TidalAuth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let auth = TidalAuth::with_api_token(
    ///         "client_id".to_string(),
    ///         "client_secret".to_string()
    ///     );
    ///     let client = TidalClient::new(&auth);
    ///     
    ///     // Get access token
    ///     let token = client.session.auth.get_access_token().await.unwrap();
    ///     println!("Access token: {}", token.access_token);
    /// }
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

    pub fn is_token_auth(&self) -> bool {
        self.api_token_auth
    }

    pub fn is_token_expired(&self) -> bool {
        if let (Some(expiry), Some(last_refresh)) = (self.refresh_expiry, self.last_refresh_time) {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= last_refresh + expiry
        } else {
            true
        }
    }

    pub async fn get_access_token(&self) -> Result<AccessTokenResponse, TidalError> {
        if !self.is_token_auth() {
            return Err(TidalError::InvalidArgument(
                "No client secret provided, can't use get_access_token, use TidalCredentials::with_token to use this.\nIf you want to login with OAuth2, use TidalCredentials::new()".to_string()
            ));
        }

        let mut form = HashMap::new();
        form.insert("grant_type".to_string(), "client_credentials".to_string());

        let mut req = TidalRequest::new(Method::POST, "/token".to_string());
        req.form = Some(vec![form]);
        req.basic_auth = Some(requests::BasicAuth::new(
            self.client_id.clone(),
            self.client_secret.clone(),
        ));
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let res = self.rq.request(req).await?;
        let json: AccessTokenResponse = res.json().await?;

        Ok(json)
    }

    /// Checks if the access token is still valid
    pub async fn check_login(&self) -> Result<bool, requests::RequestClientError> {
        if !self.is_logged_in() {
            return Ok(false);
        }

        let req = TidalRequest::new(
            Method::GET,
            format!("/users/{}/subscription", self.user_id.unwrap()),
        );

        let res = self.rq.request(req).await?;
        if res.status().is_success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.access_token.is_some()
    }
}

impl Default for TidalAuth {
    fn default() -> Self {
        let c_creds = get_client_credentials();
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

pub fn auth_default_request_client() -> requests::RequestClient {
    requests::RequestClient::new("https://api.tidal.com/v1/".to_string())
}
