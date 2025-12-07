use std::collections::HashMap;

use crate::{
    client::{TidalClient, models::home::HomeFeed},
    error::TidalError,
    requests::TidalRequest,
    utils::debug_json_str,
};

impl TidalClient {
    pub async fn get_home_feed(
        &mut self,
        limit: u32,
        time_offset: Option<String>,
    ) -> Result<HomeFeed, TidalError> {
        let url = format!("/home/feed/STATIC");

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("locale".to_string(), self.session.locale.clone());
        params.insert("limit".to_string(), limit.to_string());
        params.insert("deviceType".to_string(), "PHONE".to_string());
        params.insert("platform".to_string(), "ANDROID".to_string());
        if let Some(to) = time_offset {
            params.insert("timeOffset".to_string(), to);
        }

        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert(
            "x-tidal-client-version",
            reqwest::header::HeaderValue::from_str("2.171.1").unwrap(),
        );

        req.params = Some(params);
        req.headers = Some(header_map);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V2_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }
}
