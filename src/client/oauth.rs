use std::{collections::HashMap, time::SystemTimeError};

use reqwest::Method;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::{
    client::{
        TidalClient,
        models::{
            responses::{OAuthTokenResponse, OAuthPendingAuthorizationResponse, OAuthDeviceAuthorizationResponse},
            user::User,
        },
    },
    error::TidalError,
    requests::{self, TidalRequest},
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
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let oauth = client.get_oauth_link().await?;
    /// println!("Please visit: {}", oauth.verification_uri_complete);
    /// println!("User code: {}", oauth.user_code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_oauth_link(&self) -> Result<OAuthDeviceAuthorizationResponse, TidalError> {
        debug!("requesting OAuth device authorization link");
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

        let json: OAuthDeviceAuthorizationResponse =
            serde_json::from_str(&body).map_err(|e| TidalError::InvalidResponse(e.to_string()))?;

        debug!(
            expires_in = json.expires_in,
            interval = json.interval,
            "received OAuth device authorization link"
        );
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
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let mut client = TidalClient::new(&auth);
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
    ) -> Result<OAuthTokenResponse, TidalError> {
        debug!(
            device_code_len = device_code.len(),
            expires_in, interval, "starting OAuth device polling"
        );
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
        let mut attempt = 0_u64;
        while expiry > 0 {
            attempt += 1;
            debug!(
                attempt,
                remaining_seconds = expiry,
                "polling OAuth token endpoint"
            );
            // OAuth Device Authorization Grant uses HTTP 400 with a body of
            // `{"error":"authorization_pending", ...}` as the *normal* "keep
            // polling" signal (RFC 8628 §3.5).  After the request layer was
            // changed to turn every 4xx into a hard `StatusCode` error, this
            // loop short-circuits before the body is ever inspected and
            // surfaces a fatal error on the very first poll.
            //
            // Recover the body from the error's `body_snippet` (truncated to
            // 1024 chars by the request layer, but the OAuth response payload
            // is well under 200 chars) so the parsing logic below can
            // distinguish `authorization_pending` from a real failure.
            let body: Vec<u8> = match self.rq.request(req.clone()).await {
                Ok(res) => res.bytes().await?.to_vec(),
                Err(requests::RequestClientError::StatusCode {
                    status,
                    body_snippet,
                    ..
                }) if status == reqwest::StatusCode::BAD_REQUEST => body_snippet.into_bytes(),
                Err(e) => return Err(TidalError::RequestClient(e)),
            };
            // println!("oauth check response: {}", String::from_utf8_lossy(&body));
            let json: Result<OAuthTokenResponse, _> = serde_json::from_slice(&body);
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
                    info!(
                        attempt,
                        user_id = json.user_id,
                        "OAuth flow completed successfully"
                    );
                    return Ok(json);
                }
                Err(_) => {
                    let json_waiting: Result<OAuthPendingAuthorizationResponse, _> =
                        serde_json::from_slice(&body);

                    match json_waiting {
                        Ok(_) => {
                            if let Some(tx) = &status_tx {
                                let _ = tx.send(OAuthStatus::Waiting);
                            }
                            debug!(attempt, "OAuth authorization still pending");
                        }
                        Err(e) => {
                            if let Some(tx) = &status_tx {
                                let _ = tx.send(OAuthStatus::Error(e.to_string()));
                            }
                            warn!(attempt, error = %e, "unexpected OAuth polling response payload");
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            expiry -= interval;
        }

        warn!("OAuth polling timed out before authorization completed");
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
        debug!(
            user_id,
            access_token_len = access_token.len(),
            refresh_token_len = refresh_token.len(),
            expires_in,
            "applying manual OAuth login state"
        );
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
