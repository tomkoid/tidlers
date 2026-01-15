use serde::{Deserialize, Serialize};

/// Represents a Tidal user account
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: u64,
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
    pub birthday: u64,
    #[serde(rename = "channelId")]
    pub channel_id: u64,
    #[serde(rename = "parentId")]
    pub parent_id: u64,
    #[serde(rename = "acceptedEULA")]
    pub accepted_eula: bool,
    #[serde(rename = "created")]
    pub created: u64,
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "facebookUid")]
    pub facebook_uid: u64,
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

/// Basic user information
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    pub country: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}
