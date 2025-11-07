use thiserror::Error;

#[derive(Error, Debug)]
pub enum TidalError {
    #[error("not authenticated, either session and/or page doesn't have access token set")]
    NotAuthenticated,

    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("request client error: {0}")]
    RequestClient(#[from] crate::requests::RequestClientError),

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("invalid response from API: {0}")]
    InvalidResponse(String),

    #[error("logout failed: {0}")]
    Logout(String),
}
