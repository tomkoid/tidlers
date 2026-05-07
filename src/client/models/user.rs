use serde::{Deserialize, Serialize};

/// Represents a Tidal user account
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: u64,
    pub email: String,
    pub country_code: String,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub username: String,
    pub address: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "postalcode")]
    pub postal_code: Option<String>,
    pub us_state: Option<String>,
    pub phone_number: Option<String>,
    pub birthday: u64,
    pub channel_id: u64,
    pub parent_id: u64,
    #[serde(rename = "acceptedEULA")]
    pub accepted_eula: bool,
    pub created: u64,
    pub updated: i64,
    pub facebook_uid: Option<u64>,
    pub apple_uid: Option<String>,
    pub google_uid: Option<String>,
    pub account_link_created: bool,
    pub email_verified: bool,
    pub new_user: bool,
}

/// Basic user information
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResource {
    pub id: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub attributes: UserProfileAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileAttributes {
    pub username: String,
    pub country: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserV1Response {
    pub id: u32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

//{...,"color":[],...,"prompts":[],"socialLinks":[],...}
// TODO: implement color, prompts and socialLinks
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserV2Response {
    pub user_id: u32,
    pub artist_id: u32,
    pub name: Option<String>,
    pub handle: Option<String>,
    pub picture: Option<UserPicture>,
    pub number_of_followers: u32,
    pub number_of_follows: u32,
    pub profile_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPicture {
    pub url: String,
}
