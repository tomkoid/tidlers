use crate::{TidalClient, TidalError, client::models::user::UserV1Response};

impl TidalClient {
    /// Gets information about a user by their user ID, this is not the same as the current logged
    /// in user, for that use `get_user_info`
    /// This only returns the first and last name of the user
    pub async fn get_user_v1(&self, user_id: String) -> Result<UserV1Response, TidalError> {
        self.request(reqwest::Method::GET, format!("/users/{user_id}"))
            .with_country_code()
            .send()
            .await
    }
}
