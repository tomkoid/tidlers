use std::collections::HashMap;

use reqwest::Method;

use crate::{
    client::TidalClient,
    error::TidalError,
    requests::{self, TidalRequest},
    responses::RefreshTokenResponse,
};

impl TidalClient {
    /// Refreshes the access token using the refresh token
    pub async fn refresh_access_token(&mut self, force: bool) -> Result<bool, TidalError> {
        if self.session.auth.refresh_token.is_none() {
            eprintln!("No refresh token available, cannot refresh access token.");
            return Err(TidalError::RequestClient(
                requests::RequestClientError::InvalidCredentials,
            ));
        }

        if force || self.session.auth.is_token_expired() {
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
            req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

            let res = self.rq.request(req).await?;
            let body = res.text().await?;
            let json: RefreshTokenResponse = serde_json::from_str(&body)?;

            // update the access token and refresh token
            self.session.auth.access_token = Some(json.access_token.clone());

            return Ok(true);
        }

        Ok(false)
    }
}
