use std::collections::HashMap;

use reqwest::Method;
use tracing::{debug, info};

use crate::{
    client::{TidalClient, models::responses::RefreshTokenGrantResponse},
    error::TidalError,
    requests::{self, TidalRequest},
    urls::OAUTH2_V1_LOCATION,
};

impl TidalClient {
    /// Refreshes the access token using the refresh token
    ///
    /// Returns `true` if the token was refreshed, `false` if it was still valid.
    ///
    /// # Arguments
    ///
    /// * `force` - If true, refreshes even if the current token hasn't expired
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let mut client = TidalClient::new(&auth);
    /// // Refresh if expired
    /// if client.refresh_access_token(false).await? {
    ///     println!("Token refreshed");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh_access_token(&mut self, force: bool) -> Result<bool, TidalError> {
        debug!(force, "refresh_access_token called");
        if self.session.auth.refresh_token.is_none() {
            return Err(TidalError::Other(
                "No refresh token available, cannot refresh access token.".to_string(),
            ));
        }

        let is_expired = self.session.auth.is_token_expired()?;
        if force || is_expired {
            debug!(force, is_expired, "refreshing access token");
            let mut form = HashMap::new();
            form.insert("grant_type".to_string(), "refresh_token".to_string());
            form.insert(
                "refresh_token".to_string(),
                self.session.auth.refresh_token.clone().unwrap(),
            );
            let mut req = TidalRequest::new(Method::POST, "/token".to_string());
            req.form = Some(vec![form]);
            req.basic_auth = Some(requests::BasicAuth::new(
                self.session.auth.client_id.clone(),
                self.session.auth.client_secret.clone(),
            ));
            req.base_url = Some(OAUTH2_V1_LOCATION.to_string());

            let res = self.rq.request(req).await?;
            let body = res.text().await?;
            let json: RefreshTokenGrantResponse = serde_json::from_str(&body)?;
            debug!(
                expires_in = json.expires_in,
                token_type = %json.token_type,
                user_id = %json.user_id,
                "received refresh token response"
            );

            // update the access token and refresh token
            self.session.auth.access_token = Some(json.access_token.clone());
            self.session.auth.refresh_expiry = Some(json.expires_in as u64);
            self.session.auth.last_refresh_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
            );
            self.session.auth.user_id = Some(json.user_id.try_into()?);
            info!("access token refreshed successfully");

            return Ok(true);
        }

        debug!("access token refresh skipped because token is still valid");
        Ok(false)
    }
}
