use std::collections::HashMap;

use reqwest::Method;

#[derive(Clone, Debug)]
pub struct RequestClient {
    base_url: String,
    user_agent: String,
    client: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct TidalRequest {
    pub method: reqwest::Method,
    pub path: String,
    pub form: Option<Vec<HashMap<String, String>>>,
    pub basic_auth: Option<BasicAuth>,
    pub access_token: Option<String>,
    pub data: Option<String>,
    pub headers: Option<String>,
    pub base_url: Option<String>,
    pub enable_useful_params: bool,
}

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
            form: None,
            basic_auth: None,
            access_token: None,
            data: None,
            headers: None,
            base_url: None,
            enable_useful_params: false,
        }
    }
}

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
    #[error("timeout")]
    Timeout,
    #[error("failed to parse response")]
    ParseError(String),
}

impl RequestClient {
    pub fn new(base_url: String) -> RequestClient {
        let client = reqwest::Client::new();
        RequestClient {
            base_url,
            client,
            user_agent: "Mozilla/5.0 (Linux; Android 12; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/91.0.4472.114 Safari/537.36".to_string(),
        }
    }

    async fn requests_basic(
        &self,
        request: TidalRequest,
    ) -> Result<reqwest::Response, RequestClientError> {
        let mut req_form: HashMap<String, String> = HashMap::new();

        if request.enable_useful_params {
            req_form.insert("session_id".to_string(), "d".to_string());
            req_form.insert("countryCode".to_string(), "cz".to_string());
            req_form.insert("limit".to_string(), "10".to_string());
        }

        if let Some(params) = request.form {
            for map in params {
                for (key, value) in map {
                    req_form.insert(key, value);
                }
            }
        }

        // println!("{:?}", req_form);

        let base_url = request.base_url.unwrap_or(self.base_url.clone());

        let url = format!("{base_url}{}", request.path);
        let url_w_params = reqwest::Url::parse_with_params(&url, &req_form)?.to_string();

        // println!("Request URL: {}", url_w_params.to_string());

        let method_req: Result<reqwest::RequestBuilder, RequestClientError> = match request.method {
            Method::GET => Ok(self.client.get(url_w_params)),
            Method::POST => Ok(self.client.post(url_w_params)),
            _ => Err(RequestClientError::InvalidMethod),
        };

        let req = method_req?
            .form(&req_form)
            .header("User-Agent", self.user_agent.clone());

        let req = if let Some(data) = request.data {
            req.body(data)
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

        let req = req.send().await?;

        Ok(req)
        // Ok(())
    }

    pub async fn request(
        &self,
        request: TidalRequest,
    ) -> Result<reqwest::Response, RequestClientError> {
        let req = self.requests_basic(request).await?;

        Ok(req)
    }
}
