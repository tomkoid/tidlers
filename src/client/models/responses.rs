use serde::{Deserialize, Serialize};

use crate::client::models::user::User;

/// Generic wrapper for Tidal API responses with data and links
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct ApiDataResponse<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    pub data: T,
    pub links: ApiLinks,
}

/// Links included in API responses
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ApiLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

/// Response from the access token endpoint
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClientCredentialsTokenResponse {
    pub scope: String,
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u64,
}

/// Response from the refresh token endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenGrantResponse {
    pub scope: String,
    pub user: User,
    #[serde(rename = "clientName")]
    pub client_name: String,
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i32,
    pub user_id: i64,
}

/// Response containing OAuth device authorization information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthDeviceAuthorizationResponse {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUri")]
    pub verification_uri: String,
    #[serde(rename = "verificationUriComplete")]
    pub verification_uri_complete: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    #[serde(rename = "interval")]
    pub interval: u64,
}

/// Successful authentication response with tokens and user information
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "user")]
    pub user: User,
    #[serde(rename = "clientName")]
    pub client_name: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    #[serde(rename = "user_id")]
    pub user_id: u64,
}

/// Response when OAuth authentication is still pending
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthPendingAuthorizationResponse {
    pub status: u64,
    pub error: String,
    pub sub_status: u64,
    pub error_description: String,
}
