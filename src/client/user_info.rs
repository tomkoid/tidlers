use crate::{client::tidal::TidalClient, requests::TidalRequest, responses::UserInfoResponse};

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub country_code: String,
}

impl TidalClient {
    pub async fn fetch_user_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "/users/me".to_string();

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        req.access_token = self.session.auth.access_token.clone();
        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: UserInfoResponse = serde_json::from_str(&body)?;

        let user_info = UserInfo {
            user_id: json.data.id,
            country_code: json.data.attributes.country,
        };

        self.user_info = Some(user_info);

        Ok(())
    }
}
