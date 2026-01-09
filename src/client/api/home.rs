use crate::{
    client::{TidalClient, models::home::HomeFeedPhone},
    error::TidalError,
};

impl TidalClient {
    /// Uses the WEB_API_V2_LOCATION/home/feed/static with deviceType=PHONE
    pub async fn get_home_feed_phone(&self, limit: u32) -> Result<HomeFeedPhone, TidalError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-tidal-client-version",
            reqwest::header::HeaderValue::from_str("2.171.1").unwrap(),
        );

        self.request(reqwest::Method::GET, "/home/feed/static")
            .with_country_code()
            .with_locale()
            .with_param("limit", limit.to_string())
            .with_param("deviceType", "PHONE")
            .with_param("platform", "ANDROID")
            .with_param("timeOffset", self.session.time_offset.clone())
            .with_headers(headers)
            .with_base_url(Self::WEB_API_V2_LOCATION)
            .send()
            .await
    }
}
