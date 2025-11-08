use serde::{Deserialize, Serialize};

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
