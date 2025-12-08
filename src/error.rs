use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum TidalError {
    #[error("resource not found")]
    NotFound,

    #[error("not authenticated, either session and/or page doesn't have access token set")]
    NotAuthenticated,

    #[error("HTTP request failed: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    Request(#[from] reqwest::Error),

    #[error("failed to parse JSON response: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    JsonParse(#[from] serde_json::Error),

    #[error("request client error: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    RequestClient(#[from] crate::requests::RequestClientError),

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("invalid response from API: {0}")]
    InvalidResponse(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("logout failed: {0}")]
    Logout(String),

    #[error("failed to parse integer: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("base64 decode error: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("string from utf8 error: {0}")]
    #[serde(serialize_with = "serialize_generic")]
    StringFromUTF8Error(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    Other(String),
}

fn serialize_generic<S>(error: &dyn std::error::Error, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}
