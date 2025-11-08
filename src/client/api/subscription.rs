use std::collections::HashMap;

use crate::{
    client::{models::subscription::SubscriptionInfo, tidal::TidalClient},
    error::TidalError,
    requests::TidalRequest,
};

impl TidalClient {
    pub async fn subscription(&mut self) -> Result<SubscriptionInfo, TidalError> {
        let url = format!(
            "/users/{}/subscription",
            self.user_info.as_ref().unwrap().user_id
        );

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: SubscriptionInfo = serde_json::from_str(&body)?;

        Ok(json)
    }
}
