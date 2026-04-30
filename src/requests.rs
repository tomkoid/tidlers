use std::collections::HashMap;

use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

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
    pub(crate) fn new(name: String, pass: String) -> BasicAuth {
        BasicAuth { name, pass }
    }
}

impl TidalRequest {
    pub(crate) fn new(method: reqwest::Method, path: String) -> TidalRequest {
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

    #[error("http {status} for {url}: {body_snippet}")]
    StatusCode {
        status: reqwest::StatusCode,
        url: String,
        body_snippet: String,
    },
}

impl RequestClient {
    const ERROR_BODY_SNIPPET_MAX_CHARS: usize = 1024;

    fn error_body_snippet(body: &str) -> String {
        if body.trim().is_empty() {
            return "<empty response body>".to_string();
        }

        let body_len = body.chars().count();
        let mut snippet = body
            .chars()
            .take(Self::ERROR_BODY_SNIPPET_MAX_CHARS)
            .collect::<String>();

        if body_len > Self::ERROR_BODY_SNIPPET_MAX_CHARS {
            snippet.push_str("...(truncated)");
        }

        snippet
    }

    /// Creates a new RequestClient with the specified base URL
    pub(crate) fn new(base_url: String) -> RequestClient {
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
        let method = request.method.clone();
        let path = request.path.clone();
        let has_access_token = request.access_token.is_some();
        let has_basic_auth = request.basic_auth.is_some();
        let send_params_as_form = request.send_params_as_form;

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
        debug!(
            method = %method,
            path = %path,
            url = %url_w_params,
            params_count = req_params.len(),
            form_count = req_form.len(),
            has_access_token,
            has_basic_auth,
            send_params_as_form,
            "sending HTTP request"
        );

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

        let start = std::time::Instant::now();
        let req = req.send().await?;
        let req_status = req.status();
        debug!(
            method = %method,
            url = %req.url(),
            status = req_status.as_u16(),
            elapsed_ms = start.elapsed().as_millis(),
            "received HTTP response"
        );

        if req_status.is_client_error() || req_status.is_server_error() {
            if req_status == reqwest::StatusCode::UNAUTHORIZED {
                warn!(method = %method, url = %req.url(), "received unauthorized HTTP response");
                return Err(RequestClientError::Unauthorized);
            }

            let req_url = req.url().to_string();
            let body = req
                .text()
                .await
                .unwrap_or_else(|_| "<failed to read response body>".to_string());
            warn!(
                method = %method,
                url = %req_url,
                status = req_status.as_u16(),
                body_bytes = body.len(),
                "received HTTP error response"
            );

            return Err(RequestClientError::StatusCode {
                status: req_status,
                url: req_url,
                body_snippet: Self::error_body_snippet(&body),
            });
        }

        Ok(req)
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

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    use reqwest::Method;

    use super::{RequestClient, RequestClientError, TidalRequest};

    fn spawn_one_shot_http_server(raw_response: &'static str) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind test listener");
        let addr = listener.local_addr().expect("failed to get listener addr");

        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("failed to accept connection");
            let mut buffer = [0_u8; 2048];
            let _ = stream.read(&mut buffer);
            stream
                .write_all(raw_response.as_bytes())
                .expect("failed to write test response");
            stream.flush().expect("failed to flush test response");
        });

        (format!("http://{}", addr), handle)
    }

    #[test]
    fn error_body_snippet_handles_empty_body() {
        assert_eq!(
            RequestClient::error_body_snippet("  \n\t "),
            "<empty response body>"
        );
    }

    #[test]
    fn error_body_snippet_truncates_long_body() {
        let long_body = "a".repeat(RequestClient::ERROR_BODY_SNIPPET_MAX_CHARS + 1);
        let snippet = RequestClient::error_body_snippet(&long_body);

        assert_eq!(
            snippet.len(),
            RequestClient::ERROR_BODY_SNIPPET_MAX_CHARS + "...(truncated)".len()
        );
        assert!(snippet.ends_with("...(truncated)"));
    }

    #[tokio::test]
    async fn request_returns_unauthorized_on_401() {
        let (base_url, handle) = spawn_one_shot_http_server(
            "HTTP/1.1 401 Unauthorized\r\nContent-Length: 12\r\nConnection: close\r\n\r\nunauthorized",
        );
        let client = RequestClient::new(base_url);
        let request = TidalRequest::new(Method::GET, "/test".to_string());

        let result = client.request(request).await;
        handle.join().expect("test server thread failed");

        assert!(matches!(result, Err(RequestClientError::Unauthorized)));
    }

    #[tokio::test]
    async fn request_returns_status_error_with_context_on_non_401_error() {
        let (base_url, handle) = spawn_one_shot_http_server(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 18\r\nConnection: close\r\n\r\ninternal failure!!",
        );
        let client = RequestClient::new(base_url);
        let request = TidalRequest::new(Method::GET, "/boom".to_string());

        let result = client.request(request).await;
        handle.join().expect("test server thread failed");

        let err = result.expect_err("request should fail");
        match err {
            RequestClientError::StatusCode {
                status,
                url,
                body_snippet,
            } => {
                assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
                assert!(url.contains("/boom"));
                assert_eq!(body_snippet, "internal failure!!");
            }
            other => panic!("expected StatusCode error, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn request_accepts_non_200_success_status() {
        let (base_url, handle) = spawn_one_shot_http_server(
            "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        let client = RequestClient::new(base_url);
        let request = TidalRequest::new(Method::GET, "/ok".to_string());

        let result = client.request(request).await;
        handle.join().expect("test server thread failed");

        let response = result.expect("request should succeed");
        assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
    }
}
