use std::collections::HashMap;

use reqwest::Method;
use tokio::sync::mpsc;

use crate::{
    auth::init::TidalAuth,
    requests::{self, TidalRequest},
    responses::{AuthResponse, AuthResponseWaiting, OAuthLinkResponse},
};

#[derive(Debug, Clone, PartialEq)]
pub enum OAuthStatus {
    Waiting,
    Error(String),
    Success,
}

impl TidalAuth {
    pub async fn get_oauth_link(&self) -> Result<OAuthLinkResponse, requests::RequestClientError> {
        if self.is_token_auth() {
            eprintln!(
                "Client secret provided, you should probably use get_access_token instead.\nIf you want to login with OAuth2, use TidalCredentials::new()"
            );
            return Err(requests::RequestClientError::InvalidCredentials);
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.client_id.clone());
        form.insert("scope".to_string(), "r_usr w_usr w_sub".to_string());

        let mut req = TidalRequest::new(Method::POST, "/device_authorization".to_string());
        req.form = Some(vec![form]);
        req.send_params_as_form = true;
        req.base_url = Some("https://auth.tidal.com/v1/oauth2".to_string());

        let res = self.rq.request(req).await?;
        let body = res.text().await?;

        let json: OAuthLinkResponse = serde_json::from_str(&body).map_err(|e| {
            eprintln!("Error parsing OAuth link response: {e}\nResponse body: {body}");
            requests::RequestClientError::ParseError(e.to_string())
        })?;

        Ok(json)
    }

    pub async fn wait_for_oauth(
        &mut self,
        device_code: &str,
        expires_in: u64,
        interval: u64,
        status_tx: Option<mpsc::UnboundedSender<OAuthStatus>>,
    ) -> Result<AuthResponse, requests::RequestClientError> {
        if self.is_token_auth() {
            eprintln!("Client secret provided, cannot use this function.");
            return Err(requests::RequestClientError::InvalidCredentials);
        }

        let mut form = HashMap::new();
        form.insert("client_id".to_string(), self.client_id.clone());
        form.insert("client_secret".to_string(), self.client_secret.clone());
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
                    self.access_token = Some(json.access_token.clone());
                    self.refresh_token = Some(json.refresh_token.clone());
                    self.user_id = Some(json.user_id);
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

        Err(requests::RequestClientError::Timeout)
    }
}
