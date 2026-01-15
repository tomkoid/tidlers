use std::collections::HashMap;

use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};

/// HTTP client wrapper for making API requests
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RequestClient {
    base_url: String,
    user_agent: String,

    #[serde(skip)]
    client: reqwest::Client,
}

/// Represents an HTTP request to the Tidal API
#[derive(Clone, Debug)]
pub struct TidalRequest {
    pub method: reqwest::Method,
    pub path: String,
    pub form: Option<Vec<HashMap<String, String>>>,
    pub params: Option<HashMap<String, String>>,
    pub basic_auth: Option<BasicAuth>,
    pub access_token: Option<String>,
    pub data: Option<String>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub base_url: Option<String>,
    pub send_params_as_form: bool,
}

/// HTTP Basic authentication credentials
#[derive(Clone, Debug)]
pub struct BasicAuth {
    pub name: String,
    pub pass: String,
}

impl BasicAuth {
    pub fn new(name: String, pass: String) -> BasicAuth {
        BasicAuth { name, pass }
    }
}

impl TidalRequest {
    pub fn new(method: reqwest::Method, path: String) -> TidalRequest {
        TidalRequest {
            method,
            path,
            params: None,
            form: None,
            basic_auth: None,
            access_token: None,
            data: None,
            headers: None,
            base_url: None,
            send_params_as_form: false,
        }
    }
}

/// Errors that can occur during HTTP requests
#[derive(thiserror::Error, Debug)]
pub enum RequestClientError {
    #[error("failed to parse url with params error")]
    URLParamsParseError(#[from] url::ParseError),
    #[error("failed to do request")]
    RequestError(#[from] reqwest::Error),
    #[error("invalid http method")]
    InvalidMethod,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("unauthorized - invalid credentials or can not access resource")]
    Unauthorized,
    #[error("timeout")]
    Timeout,
    #[error("failed to parse response")]
    ParseError(String),

    #[error("http status code error: {0}")]
    StatusCode(reqwest::StatusCode),
}

impl RequestClient {
    /// Creates a new RequestClient with the specified base URL
    pub fn new(base_url: String) -> RequestClient {
        let client = reqwest::Client::new();
        RequestClient {
            base_url,
            client,
            user_agent: "Mozilla/5.0 (Linux; Android 12; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/91.0.4472.114 Safari/537.36".to_string(),
        }
    }

    /// Internal method to execute HTTP requests with all configured options
    async fn requests_basic(
        &self,
        request: TidalRequest,
    ) -> Result<reqwest::Response, RequestClientError> {
        let mut req_form: HashMap<String, String> = HashMap::new();
        let mut req_params: HashMap<String, String> = HashMap::new();

        if let Some(ref form) = request.form {
            for map in form {
                for (key, value) in map.clone() {
                    req_form.insert(key, value);
                }
            }
        }

        if let Some(params) = request.params {
            for (key, value) in params {
                req_params.insert(key, value);
            }
        }

        // println!("{:?}", req_form);

        let base_url = request.base_url.unwrap_or(self.base_url.clone());

        if request.send_params_as_form {
            for (key, value) in req_params.drain() {
                req_form.insert(key, value);
            }
        }

        let url = format!("{base_url}{}", request.path);
        let url_w_params = reqwest::Url::parse_with_params(&url, &req_params)?.to_string();

        // println!("Request URL: {}", url_w_params.to_string());

        let method_req: Result<reqwest::RequestBuilder, RequestClientError> = match request.method {
            Method::GET => Ok(self.client.get(url_w_params)),
            Method::DELETE => Ok(self.client.delete(url_w_params)),
            Method::PUT => Ok(self.client.put(url_w_params)),
            Method::POST => Ok(self.client.post(url_w_params)),
            _ => Err(RequestClientError::InvalidMethod),
        };

        let req = method_req?.header("User-Agent", self.user_agent.clone());

        let req = if let Some(data) = request.data {
            req.body(data)
        } else {
            req
        };

        let req = if request.form.is_some() || request.send_params_as_form {
            req.form(&req_form)
        } else {
            req
        };

        let req = if let Some(access_token) = request.access_token {
            req.bearer_auth(access_token)
        } else {
            req
        };

        let req = if let Some(basic_auth) = request.basic_auth {
            req.basic_auth(basic_auth.name, Some(basic_auth.pass))
        } else {
            req
        };

        let req = if let Some(headers) = request.headers {
            req.headers(headers)
        } else {
            req
        };

        let req = req.send().await?;

        if req.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(RequestClientError::Unauthorized);
        }

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(RequestClientError::StatusCode(
                reqwest::StatusCode::NOT_FOUND,
            ));
        }

        Ok(req)
        // Ok(())
    }

    /// Executes an HTTP request and returns the response
    pub async fn request(
        &self,
        request: TidalRequest,
    ) -> Result<reqwest::Response, RequestClientError> {
        let req = self.requests_basic(request).await?;

        Ok(req)
    }
}
