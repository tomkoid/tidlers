use crate::{
    client::{TidalClient, models::subscription::UserSubscriptionResponse},
    error::TidalError,
};

impl TidalClient {
    /// Retrieves the current user's subscription details.
    pub async fn subscription(&self) -> Result<UserSubscriptionResponse, TidalError> {
        let url = format!("/users/{}/subscription", self.user_id()?);

        self.request(reqwest::Method::GET, url)
            .with_country_code()
            .send()
            .await
    }
}
