use crate::{
    TidalClient, TidalError, client::models::collection::favorites::FavoriteResourceType,
    urls::API_V1_LOCATION,
};

impl TidalClient {
    /// Adds a resource into user's favorite items
    pub async fn add_to_favorites(
        &self,
        resource: FavoriteResourceType,
        resource_id: u32,
    ) -> Result<(), TidalError> {
        let user_id = self
            .session
            .auth
            .user_id
            .ok_or_else(|| TidalError::NotAuthenticated)?;

        let resource_id_param = match resource {
            FavoriteResourceType::Tracks => "trackIds",
            FavoriteResourceType::Albums => "albumIds",
        };

        let url = format!("/users/{}/favorites/{}", user_id, resource);
        self.request(reqwest::Method::POST, url)
            .with_country_code()
            .with_param(resource_id_param, resource_id.to_string())
            .with_params_as_form()
            .with_base_url(API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(())
    }
}
