use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum TidalError {
    #[error("not authenticated, either session and/or page doesn't have access token set")]
    NotAuthenticated,

    #[error("HTTP request failed: {0}")]
    #[serde(serialize_with = "serialize_reqwest_error")]
    Request(#[from] reqwest::Error),

    #[error("failed to parse JSON response: {0}")]
    #[serde(serialize_with = "serialize_json_error")]
    JsonParse(#[from] serde_json::Error),

    #[error("request client error: {0}")]
    #[serde(serialize_with = "serialize_request_client_error")]
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
    #[serde(serialize_with = "serialize_parse_int_error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("base64 decode error: {0}")]
    #[serde(serialize_with = "serialize_base64_error")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("string from utf8 error: {0}")]
    #[serde(serialize_with = "serialize_from_utf8_error")]
    StringFromUTF8Error(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    Other(String),
}

// Helper functions to serialize non-serializable error types
fn serialize_reqwest_error<S>(error: &reqwest::Error, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

fn serialize_json_error<S>(error: &serde_json::Error, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

fn serialize_base64_error<S>(error: &base64::DecodeError, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

fn serialize_request_client_error<S>(
    error: &crate::requests::RequestClientError,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

fn serialize_parse_int_error<S>(
    error: &std::num::ParseIntError,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

fn serialize_from_utf8_error<S>(
    error: &std::string::FromUtf8Error,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}
