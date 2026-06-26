use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tracing::{debug, warn};

use crate::{
    client::TidalClient, error::TidalError, requests::TidalRequest, utils::debug_json_str,
};

/// Builder for constructing API requests with fluent interface
pub struct ApiRequestBuilder<'a> {
    client: &'a TidalClient,
    method: Method,
    url: String,
    params: HashMap<String, String>,
    form_params: HashMap<String, String>,
    base_url: Option<String>,
    headers: reqwest::header::HeaderMap,
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
            form_params: HashMap::new(),
            base_url: None,
            headers: reqwest::header::HeaderMap::new(),
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

    /// Adds `deviceType` and `platform` query parameters and `x-tidal-client-version` header to the request
    pub(crate) fn with_web_stuff(mut self) -> Self {
        self.params
            .insert("deviceType".to_string(), "BROWSER".to_string());
        self.params
            .insert("platform".to_string(), "WEB".to_string());

        self.headers.insert(
            "x-tidal-client-version",
            reqwest::header::HeaderValue::from_static("2026.1.6"),
        );

        self
    }

    /// Adds an x-www-form-urlencoded body parameter to the request
    pub(crate) fn with_form_param(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.form_params.insert(key.into(), value.into());
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
        self.headers = headers;
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

        let mut req = TidalRequest::new(self.method, self.url.clone());
        req.params = Some(self.params);
        req.form = (!self.form_params.is_empty()).then_some(vec![self.form_params]);
        req.access_token = self.client.session.auth.access_token.clone();
        req.base_url = self.base_url;
        req.headers = Some(self.headers);

        debug!(
            method = %req.method,
            path = %self.url,
            params_count = req.params.as_ref().map_or(0, |p| p.len()),
            has_custom_base_url = req.base_url.is_some(),
            has_headers = req.headers.is_some(),
            "sending API request"
        );
        let resp = self.client.rq.request(req).await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(TidalError::NotFound);
        }

        let status = resp.status();
        let response_url = resp.url().to_string();
        let body = resp.text().await?;
        debug!(
            path = %self.url,
            response_url = %response_url,
            status = status.as_u16(),
            body_bytes = body.len(),
            "received API response"
        );

        if self.request_debug {
            debug_json_str(&body);
        }

        let mut deserializer = serde_json::Deserializer::from_str(&body);
        serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
            let response_body = if body.trim().is_empty() {
                "<empty response body>"
            } else {
                body.as_str()
            };
            let path = error.path().to_string();
            let path = if path.is_empty() {
                "<root>".to_string()
            } else {
                path
            };
            let inner = error.into_inner();
            warn!(
                path = %self.url,
                response_url = %response_url,
                status = status.as_u16(),
                "failed to deserialize API response body"
            );

            TidalError::InvalidResponse(format!(
                "failed to parse JSON response from {response_url} (status {status}): {inner} (path: {path})\nresponse body: {response_body}"
            ))
        })
    }

    /// Executes the request, deserializes the response, and returns ETag response header
    pub(crate) async fn send_with_etag<T: DeserializeOwned>(
        mut self,
    ) -> Result<(T, Option<String>), TidalError> {
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

        let mut req = TidalRequest::new(self.method, self.url.clone());
        req.params = Some(self.params);
        req.form = (!self.form_params.is_empty()).then_some(vec![self.form_params]);
        req.access_token = self.client.session.auth.access_token.clone();
        req.base_url = self.base_url;
        req.headers = Some(self.headers);

        debug!(
            method = %req.method,
            path = %self.url,
            params_count = req.params.as_ref().map_or(0, |p| p.len()),
            has_custom_base_url = req.base_url.is_some(),
            has_headers = req.headers.is_some(),
            "sending API request"
        );
        let resp = self.client.rq.request(req).await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(TidalError::NotFound);
        }

        let status = resp.status();
        let response_url = resp.url().to_string();
        let etag = resp
            .headers()
            .get(reqwest::header::ETAG)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = resp.text().await?;
        debug!(
            path = %self.url,
            response_url = %response_url,
            status = status.as_u16(),
            body_bytes = body.len(),
            "received API response"
        );

        if self.request_debug {
            debug_json_str(&body);
        }

        let mut deserializer = serde_json::Deserializer::from_str(&body);
        let parsed = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
            let response_body = if body.trim().is_empty() {
                "<empty response body>"
            } else {
                body.as_str()
            };
            let path = error.path().to_string();
            let path = if path.is_empty() {
                "<root>".to_string()
            } else {
                path
            };
            let inner = error.into_inner();
            warn!(
                path = %self.url,
                response_url = %response_url,
                status = status.as_u16(),
                "failed to deserialize API response body"
            );

            TidalError::InvalidResponse(format!(
                "failed to parse JSON response from {response_url} (status {status}): {inner} (path: {path})\nresponse body: {response_body}"
            ))
        })?;

        Ok((parsed, etag))
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
        req.form = (!self.form_params.is_empty()).then_some(vec![self.form_params]);
        req.access_token = self.client.session.auth.access_token.clone();
        req.base_url = self.base_url;
        req.headers = Some(self.headers);

        debug!(
            method = %req.method,
            path = %req.path,
            params_count = req.params.as_ref().map_or(0, |p| p.len()),
            has_custom_base_url = req.base_url.is_some(),
            has_headers = req.headers.is_some(),
            "sending raw API request"
        );
        let resp = self.client.rq.request(req).await?;
        let status = resp.status();
        let response_url = resp.url().to_string();
        let body = resp.text().await?;
        debug!(
            response_url = %response_url,
            status = status.as_u16(),
            body_bytes = body.len(),
            "received raw API response"
        );
        Ok(body)
    }
}

impl TidalClient {
    pub(crate) fn request(&self, method: Method, url: impl Into<String>) -> ApiRequestBuilder<'_> {
        ApiRequestBuilder::new(self, method, url, self.debug_mode)
    }
}
