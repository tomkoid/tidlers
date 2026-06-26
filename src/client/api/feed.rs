use crate::{
    TidalClient, TidalError,
    client::models::feed::{ActivityFeedResponse, FeedActivity},
    urls::API_V2_LOCATION,
};

impl TidalClient {
    /// Retrieves the raw activity feed payload.
    pub async fn get_activity_feed_raw(&self) -> Result<ActivityFeedResponse, TidalError> {
        self.request(reqwest::Method::GET, "/feed/activities")
            .with_country_code()
            .with_locale()
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }

    /// Retrieves the activity feed as a flat list of activities.
    pub async fn get_activity_feed(&self) -> Result<Vec<FeedActivity>, TidalError> {
        let response = self.get_activity_feed_raw().await?;
        Ok(response.into_activities())
    }
}
