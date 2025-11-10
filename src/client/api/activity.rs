use std::collections::HashMap;

use crate::{
    client::{models::activity::ActivityTimeline, tidal::TidalClient},
    error::TidalError,
    requests::TidalRequest,
};

impl TidalClient {
    pub async fn get_activity_timeline(&mut self) -> Result<ActivityTimeline, TidalError> {
        let url = "/my-activity/timeline".to_string();

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("locale".to_string(), self.session.locale.clone());

        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V2_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: ActivityTimeline = serde_json::from_str(&body)?;

        Ok(json)
    }
}
