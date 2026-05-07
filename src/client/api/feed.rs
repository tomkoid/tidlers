use crate::{
    TidalClient, TidalError, client::models::feed::ActivityFeedResponse, urls::API_V2_LOCATION,
};

impl TidalClient {
    pub async fn get_activity_feed(&self) -> Result<ActivityFeedResponse, TidalError> {
        self.request(reqwest::Method::GET, "/feed/activities")
            .with_country_code()
            .with_locale()
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }
}
