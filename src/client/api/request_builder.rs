use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{client::TidalClient, error::TidalError, requests::TidalRequest};

pub struct ApiRequestBuilder<'a> {
    client: &'a mut TidalClient,
    method: Method,
    url: String,
    params: HashMap<String, String>,
    base_url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,
    add_country_code: bool,
    add_locale: bool,
}

impl<'a> ApiRequestBuilder<'a> {
    pub fn new(client: &'a mut TidalClient, method: Method, url: impl Into<String>) -> Self {
        Self {
            client,
            method,
            url: url.into(),
            params: HashMap::new(),
            base_url: None,
            headers: None,
            add_country_code: false,
            add_locale: false,
        }
    }

    pub fn with_country_code(mut self) -> Self {
        self.add_country_code = true;
        self
    }

    pub fn with_locale(mut self) -> Self {
        self.add_locale = true;
        self
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params.extend(params);
        self
    }

    pub fn with_optional_param(
        mut self,
        key: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        if let Some(v) = value {
            self.params.insert(key.into(), v.into());
        }
        self
    }

    pub fn with_headers(mut self, headers: reqwest::header::HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub async fn send<T: DeserializeOwned>(mut self) -> Result<T, TidalError> {
        if self.add_country_code {
            self.params.insert(
                "countryCode".to_string(),
                self.client.user_info.as_ref().unwrap().country_code.clone(),
            );
        }

        if self.add_locale {
            self.params
                .insert("locale".to_string(), self.client.session.locale.clone());
        }

        let mut req = TidalRequest::new(self.method, self.url);
        req.params = Some(self.params);
        req.access_token = self.client.session.auth.access_token.clone();
        req.base_url = self.base_url;
        req.headers = self.headers;

        let resp = self.client.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn send_raw(mut self) -> Result<String, TidalError> {
        if self.add_country_code {
            self.params.insert(
                "countryCode".to_string(),
                self.client.user_info.as_ref().unwrap().country_code.clone(),
            );
        }

        if self.add_locale {
            self.params
                .insert("locale".to_string(), self.client.session.locale.clone());
        }

        let mut req = TidalRequest::new(self.method, self.url);
        req.params = Some(self.params);
        req.access_token = self.client.session.auth.access_token.clone();
        req.base_url = self.base_url;
        req.headers = self.headers;

        let resp = self.client.rq.request(req).await?;
        Ok(resp.text().await?)
    }
}

impl TidalClient {
    pub fn request(&mut self, method: Method, url: impl Into<String>) -> ApiRequestBuilder<'_> {
        ApiRequestBuilder::new(self, method, url)
    }
}
