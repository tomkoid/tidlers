use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    auth::client_credentials::get_client_credentials,
    error::TidalError,
    requests::{self, TidalRequest},
    responses::{AccessTokenResponse, RefreshTokenResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TidalAuth {
    pub client_id: String,
    pub client_secret: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<u64>,

    #[serde(
        skip_serializing,
        skip_deserializing,
        default = "auth_default_request_client"
    )]
    pub rq: requests::RequestClient,

    pub api_token_auth: bool,
}

impl TidalAuth {
    pub fn new() -> Self {
        Self {
            access_token: None,
            api_token_auth: false,
            ..Default::default()
        }
    }

    pub fn with_access_token(access_token: String) -> Self {
        Self {
            access_token: Some(access_token),
            api_token_auth: false,
            ..Default::default()
        }
    }

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

    pub async fn get_access_token(
        &self,
    ) -> Result<AccessTokenResponse, requests::RequestClientError> {
        if !self.is_token_auth() {
            eprintln!(
                "No client secret provided, can't use get_access_token, use TidalCredentials::with_token to use this.\nIf you want to login with OAuth2, use TidalCredentials::new()"
            );
            return Err(requests::RequestClientError::InvalidCredentials);
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
            user_id: None,
            api_token_auth: false,
            rq,
        }
    }
}

pub fn auth_default_request_client() -> requests::RequestClient {
    requests::RequestClient::new("https://api.tidal.com/v1/".to_string())
}
