use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{
    client::TidalClient, error::TidalError, requests::TidalRequest, utils::debug_json_str,
};

/// Builder for constructing API requests with fluent interface
pub struct ApiRequestBuilder<'a> {
    client: &'a TidalClient,
    method: Method,
    url: String,
    params: HashMap<String, String>,
    base_url: Option<String>,
    headers: Option<reqwest::header::HeaderMap>,
    add_country_code: bool,
    add_locale: bool,
    request_debug: bool,
}

impl<'a> ApiRequestBuilder<'a> {
    pub(crate) fn new(
        client: &'a TidalClient,
        method: Method,
        url: impl Into<String>,
        request_debug: bool,
    ) -> Self {
        Self {
            client,
            method,
            url: url.into(),
            params: HashMap::new(),
            base_url: None,
            headers: None,
            add_country_code: false,
            add_locale: false,
            request_debug,
        }
    }

    /// Adds the country code parameter to the request
    pub(crate) fn with_country_code(mut self) -> Self {
        self.add_country_code = true;
        self
    }

    /// Adds the locale parameter to the request
    pub(crate) fn with_locale(mut self) -> Self {
        self.add_locale = true;
        self
    }

    /// Sets a custom base URL for this request
    pub(crate) fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Adds a query parameter to the request
    pub(crate) fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Adds multiple query parameters from a HashMap
    pub(crate) fn _with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params.extend(params);
        self
    }

    /// Adds a query parameter only if the value is Some
    pub(crate) fn with_optional_param(
        mut self,
        key: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        if let Some(v) = value {
            self.params.insert(key.into(), v.into());
        }
        self
    }

    /// Sets custom headers for the request
    pub(crate) fn with_headers(mut self, headers: reqwest::header::HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Executes the request and deserializes the response into type T
    pub(crate) async fn send<T: DeserializeOwned>(mut self) -> Result<T, TidalError> {
        if self.add_country_code {
            if let Some(user_info) = &self.client.user_info {
                self.params
                    .insert("countryCode".to_string(), user_info.country_code.clone());
            } else {
                return Err(TidalError::NotAuthenticated);
            }
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

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(TidalError::NotFound);
        }

        let body = resp.text().await?;

        if self.request_debug {
            debug_json_str(&body);
        }

        Ok(serde_json::from_str(&body)?)
    }

    /// Executes the request and returns the raw response as a String
    pub(crate) async fn send_raw(mut self) -> Result<String, TidalError> {
        if self.add_country_code {
            if let Some(user_info) = &self.client.user_info {
                self.params
                    .insert("countryCode".to_string(), user_info.country_code.clone());
            } else {
                return Err(TidalError::NotAuthenticated);
            }
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
    pub(crate) fn request(&self, method: Method, url: impl Into<String>) -> ApiRequestBuilder<'_> {
        ApiRequestBuilder::new(self, method, url, self.debug_mode)
    }
}
