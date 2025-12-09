use crate::{
    client::{TidalClient, models::subscription::SubscriptionInfo},
    error::TidalError,
};

impl TidalClient {
    pub async fn subscription(&self) -> Result<SubscriptionInfo, TidalError> {
        let url = format!(
            "/users/{}/subscription",
            self.user_info.as_ref().unwrap().user_id
        );

        self.request(reqwest::Method::GET, url)
            .with_country_code()
            .send()
            .await
    }
}
