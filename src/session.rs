use std::collections::HashMap;

use reqwest::Method;

use crate::{
    config::TidalConfig,
    credentials::TidalCredentials,
    requests::{self, RequestClient, TidalRequest},
    responses::{AccessTokenResponse, AuthResponse, AuthResponseWaiting, OAuthLinkResponse},
};

#[derive(Debug, Clone)]
pub struct TidalSession {
    pub rq: RequestClient,
    pub config: TidalConfig,
    pub credentials: TidalCredentials,
}

impl TidalSession {
    pub fn new(credentials: &TidalCredentials) -> TidalSession {
        let api_v1_location = "https://api.tidal.com/v1/".to_string();
        TidalSession {
            rq: RequestClient::new(api_v1_location),
            config: TidalConfig::new(),
            credentials: credentials.clone(),
        }
    }

    pub async fn get_oauth_link(&self) -> Result<OAuthLinkResponse, requests::RequestClientError> {
        if self.credentials.is_token_auth() {
            eprintln!(
                "Client secret provided, you should probably use get_access_token instead.\nIf you want to login with OAuth2, use TidalCredentials::new()"
            );
            return Err(requests::RequestClientError::InvalidCredentials);
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.credentials.client_id.clone());
        form.insert("scope".to_string(), "r_usr w_usr w_sub".to_string());

        let mut req = TidalRequest::new(Method::POST, "/device_authorization".to_string());
        req.form = Some(vec![form]);
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let res = self.rq.request(req).await?;
        // println!("oauth response: {}", res.text().await?);
        let json = res.json().await?;

        Ok(json)
    }

    pub async fn wait_for_oauth(
        &mut self,
        device_code: &str,
        expires_in: u64,
        interval: u64,
    ) -> Result<AuthResponse, requests::RequestClientError> {
        if self.credentials.is_token_auth() {
            eprintln!("Client secret provided, cannot use this function.");
            return Err(requests::RequestClientError::InvalidCredentials);
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.credentials.client_id.clone());
        form.insert(
            "client_secret".to_string(),
            self.credentials.client_secret.clone(),
        );
        form.insert("device_code".to_string(), device_code.to_string());
        form.insert(
            "grant_type".to_string(),
            "urn:ietf:params:oauth:grant-type:device_code".to_string(),
        );
        form.insert("scope".to_string(), "r_usr w_usr w_sub".to_string());

        let mut req = TidalRequest::new(Method::POST, "/token".to_string());
        req.form = Some(vec![form]);
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let mut expiry = expires_in;
        while expiry > 0 {
            let res = self.rq.request(req.clone()).await?;
            let body = res.bytes().await?;
            // println!("oauth check response: {}", res.text().await?);
            let json: Result<AuthResponse, _> = serde_json::from_slice(&body);
            match json {
                Ok(json) => {
                    println!("oauth check response: {json:?}");
                    self.credentials.access_token = Some(json.access_token.clone());
                    return Ok(json);
                }
                Err(_) => {
                    let json_waiting: Result<AuthResponseWaiting, _> =
                        serde_json::from_slice(&body);

                    match json_waiting {
                        Ok(_) => {
                            eprintln!("waiting for oauth..");
                        }
                        Err(e) => {
                            eprintln!("waiting for oauth: error: {e}");
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            expiry -= interval;
        }

        Err(requests::RequestClientError::Timeout)
    }

    pub async fn get_access_token(
        &self,
    ) -> Result<AccessTokenResponse, requests::RequestClientError> {
        if !self.credentials.is_token_auth() {
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
            self.credentials.client_id.clone(),
            self.credentials.client_secret.clone(),
        ));
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let res = self.rq.request(req).await?;
        let json: AccessTokenResponse = res.json().await?;

        Ok(json)
    }

    pub fn is_logged_in(&self) -> bool {
        self.credentials.access_token.is_some()
    }
}
