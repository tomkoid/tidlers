#[derive(Debug, Clone, serde::Deserialize)]
pub struct AccessTokenResponse {
    pub scope: String,
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct OAuthLinkResponse {
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
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
    pub expires_in: i64,
    #[serde(rename = "user_id")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "address")]
    pub address: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "postalcode")]
    pub postal_code: Option<String>,
    #[serde(rename = "usState")]
    pub us_state: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "birthday")]
    pub birthday: i64,
    #[serde(rename = "channelId")]
    pub channel_id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    #[serde(rename = "acceptedEULA")]
    pub accepted_eula: bool,
    #[serde(rename = "created")]
    pub created: i64,
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "facebookUid")]
    pub facebook_uid: i64,
    #[serde(rename = "appleUid")]
    pub apple_uid: Option<String>,
    #[serde(rename = "googleUid")]
    pub google_uid: Option<String>,
    #[serde(rename = "accountLinkCreated")]
    pub account_link_created: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "newUser")]
    pub new_user: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponseWaiting {
    pub status: u64,
    pub error: String,
    pub sub_status: u64,
    pub error_description: String,
}
