use std::{collections::HashMap, time::SystemTimeError};

use reqwest::Method;
use tokio::sync::mpsc;

use crate::{
    client::{TidalClient, models::user::User},
    error::TidalError,
    requests::{self, TidalRequest},
    responses::{AuthResponse, AuthResponseWaiting, OAuthLinkResponse},
};

/// Status updates during the OAuth flow
#[derive(Debug, Clone, PartialEq)]
pub enum OAuthStatus {
    Waiting,
    Error(String),
    Success,
}

impl TidalClient {
    /// Initiates the OAuth device authorization flow and returns the verification link
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_oauth();
    /// let client = TidalClient::new(&auth);
    ///
    /// let oauth = client.get_oauth_link().await?;
    /// println!("Please visit: {}", oauth.verification_uri_complete);
    /// println!("User code: {}", oauth.user_code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_oauth_link(&self) -> Result<OAuthLinkResponse, TidalError> {
        if self.session.auth.is_token_auth() {
            return Err(TidalError::InvalidArgument(
                "Client secret provided, you should probably use get_access_token instead.\nIf you want to login with OAuth2, use TidalAuth::with_oauth()".to_string()
            ));
        }

        if self.session.auth.client_id.is_empty() {
            return Err(TidalError::InvalidArgument(
                "No client ID provided, cannot get OAuth link.".to_string(),
            ));
        }

        if !self.session.auth.oauth_login {
            return Err(TidalError::InvalidArgument(
                "OAuth login not enabled in TidalAuth, cannot get OAuth link. Use TidalAuth::with_oauth() to enable it.".to_string()
            ));
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.session.auth.client_id.clone());
        form.insert("scope".to_string(), "r_usr w_usr w_sub".to_string());

        let mut req = TidalRequest::new(Method::POST, "/device_authorization".to_string());
        req.form = Some(vec![form]);
        req.send_params_as_form = true;
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let res = self.rq.request(req).await?;
        let body = res.text().await?;

        let json: OAuthLinkResponse =
            serde_json::from_str(&body).map_err(|e| TidalError::InvalidResponse(e.to_string()))?;

        Ok(json)
    }

    /// Polls the OAuth endpoint until the user completes authentication or the request times out
    ///
    /// # Arguments
    ///
    /// * `device_code` - Device code from `get_oauth_link()`
    /// * `expires_in` - Expiration time in seconds
    /// * `interval` - Polling interval in seconds
    /// * `status_tx` - Optional channel to receive status updates
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_oauth();
    /// let mut client = TidalClient::new(&auth);
    ///
    /// let oauth = client.get_oauth_link().await?;
    /// println!("Visit: {}", oauth.verification_uri_complete);
    ///
    /// let auth_response = client.wait_for_oauth(
    ///     &oauth.device_code,
    ///     oauth.expires_in,
    ///     oauth.interval,
    ///     None
    /// ).await?;
    ///
    /// println!("Logged in as: {}", auth_response.user.username);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_oauth(
        &mut self,
        device_code: &str,
        expires_in: u64,
        interval: u64,
        status_tx: Option<mpsc::UnboundedSender<OAuthStatus>>,
    ) -> Result<AuthResponse, TidalError> {
        if self.session.auth.is_token_auth() {
            return Err(TidalError::InvalidArgument(
                "Client secret provided, cannot use this function.".to_string(),
            ));
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.session.auth.client_id.clone());
        form.insert(
            "client_secret".to_string(),
            self.session.auth.client_secret.clone(),
        );
        form.insert("device_code".to_string(), device_code.to_string());
        form.insert(
            "grant_type".to_string(),
            "urn:ietf:params:oauth:grant-type:device_code".to_string(),
        );
        form.insert("scope".to_string(), "r_usr w_usr w_sub".to_string());

        let mut req = TidalRequest::new(Method::POST, "/token".to_string());
        req.form = Some(vec![form]);
        req.send_params_as_form = true;
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let mut expiry = expires_in;
        while expiry > 0 {
            let res = self.rq.request(req.clone()).await?;
            let body = res.bytes().await?;
            // println!("oauth check response: {}", res.text().await?);
            let json: Result<AuthResponse, _> = serde_json::from_slice(&body);
            match json {
                Ok(json) => {
                    if let Some(tx) = &status_tx {
                        let _ = tx.send(OAuthStatus::Success);
                    }
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs();

                    self.session.auth.access_token = Some(json.access_token.clone());
                    self.session.auth.refresh_token = Some(json.refresh_token.clone());
                    self.session.auth.refresh_expiry = Some(json.expires_in);
                    self.session.auth.last_refresh_time = Some(now);
                    self.session.auth.user_id = Some(json.user_id);
                    self.user_info = Some(json.user.clone());
                    return Ok(json);
                }
                Err(_) => {
                    let json_waiting: Result<AuthResponseWaiting, _> =
                        serde_json::from_slice(&body);

                    match json_waiting {
                        Ok(_) => {
                            if let Some(tx) = &status_tx {
                                let _ = tx.send(OAuthStatus::Waiting);
                            }
                        }
                        Err(e) => {
                            if let Some(tx) = &status_tx {
                                let _ = tx.send(OAuthStatus::Error(e.to_string()));
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            expiry -= interval;
        }

        Err(TidalError::RequestClient(
            requests::RequestClientError::Timeout,
        ))
    }

    /// Manually log in with OAuth tokens
    /// Use this after obtaining the tokens from the OAuth flow
    pub async fn oauth_manual_login(
        &mut self,
        access_token: String,
        refresh_token: String,
        expires_in: u64,
        user_id: u64,
        user: User,
    ) -> Result<(), SystemTimeError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        self.session.auth.access_token = Some(access_token);
        self.session.auth.refresh_token = Some(refresh_token);
        self.session.auth.refresh_expiry = Some(expires_in);
        self.session.auth.last_refresh_time = Some(now);
        self.session.auth.user_id = Some(user_id);
        self.user_info = Some(user.clone());

        Ok(())
    }
}
