use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError},
};

use reqwest::Method;

use crate::{
    TidalError,
    auth::TidalAuth,
    requests::{self, TidalRequest},
    responses::AccessTokenResponse,
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
