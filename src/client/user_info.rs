use serde::{Deserialize, Serialize};

use crate::{
    client::tidal::TidalClient, error::TidalError, requests::TidalRequest,
    responses::TidalGenericResponse,
};

impl TidalClient {
    pub async fn fetch_user_info(&mut self) -> Result<(), TidalError> {
        let url = "/users/me".to_string();

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        req.access_token = self.session.auth.access_token.clone();
        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: TidalGenericResponse<UserData> = serde_json::from_str(&body)?;

        let user_info = UserInfo {
            user_id: json.data.id,
            country_code: json.data.attributes.country,
        };

        self.user_info = Some(user_info);

        Ok(())
    }
}

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
