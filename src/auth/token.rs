use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError},
};

use reqwest::Method;
use tracing::{debug, warn};

use crate::{
    TidalError,
    auth::TidalAuth,
    client::models::responses::ClientCredentialsTokenResponse,
    requests::{self, TidalRequest},
};

impl TidalAuth {
    pub(crate) fn is_token_auth(&self) -> bool {
        self.api_token_auth
    }

    pub(crate) fn is_token_expired(&self) -> Result<bool, SystemTimeError> {
        if let (Some(expiry), Some(last_refresh)) = (self.refresh_expiry, self.last_refresh_time) {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs();
            Ok(now >= last_refresh + expiry)
        } else {
            Ok(true)
        }
    }

    pub async fn get_access_token(&self) -> Result<ClientCredentialsTokenResponse, TidalError> {
        debug!("requesting access token via client credentials flow");
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
        let json: ClientCredentialsTokenResponse = res.json().await?;

        debug!(
            expires_in = json.expires_in,
            token_type = %json.token_type,
            "received access token response"
        );
        Ok(json)
    }

    /// Checks if the access token is still valid
    pub async fn check_login(&self) -> Result<bool, requests::RequestClientError> {
        if !self.is_logged_in() {
            debug!("check_login: no access token present");
            return Ok(false);
        }

        let req = TidalRequest::new(
            Method::GET,
            format!("/users/{}/subscription", self.user_id.unwrap()),
        );

        let res = self.rq.request(req).await?;
        if res.status().is_success() {
            debug!("check_login: subscription endpoint accepted token");
            Ok(true)
        } else {
            warn!(
                status = res.status().as_u16(),
                "check_login: token validation request failed"
            );
            Ok(false)
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.access_token.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::TidalAuth;

    #[test]
    fn token_is_expired_when_no_refresh_metadata_exists() {
        let auth = TidalAuth::with_access_token("access_token".to_string());
        assert!(auth.is_token_expired().expect("time check should succeed"));
    }

    #[test]
    fn token_is_not_expired_before_expiry_window() {
        let mut auth = TidalAuth::with_access_token("access_token".to_string());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_secs();

        auth.last_refresh_time = Some(now - 5);
        auth.refresh_expiry = Some(60);

        assert!(!auth.is_token_expired().expect("time check should succeed"));
    }

    #[test]
    fn token_is_expired_after_expiry_window() {
        let mut auth = TidalAuth::with_access_token("access_token".to_string());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_secs();

        auth.last_refresh_time = Some(now - 120);
        auth.refresh_expiry = Some(60);

        assert!(auth.is_token_expired().expect("time check should succeed"));
    }
}
