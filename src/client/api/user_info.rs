use serde::{Deserialize, Serialize};

use crate::{
    client::{
        models::user::{UserData, UserInfo},
        tidal::TidalClient,
    },
    error::TidalError,
    requests::TidalRequest,
    responses::TidalGenericResponse,
};

impl TidalClient {
    pub async fn fetch_user_info(&mut self) -> Result<(), TidalError> {
        let url = "/users/me".to_string();

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::OPEN_API_V2_LOCATION.to_string());
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
