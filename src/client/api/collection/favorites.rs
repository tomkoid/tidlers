use crate::{
    TidalClient, TidalError, client::models::collection::favorites::FavoriteResourceType,
    urls::API_V1_LOCATION,
};

impl TidalClient {
    fn get_resouce_id_param(&self, resource: &FavoriteResourceType) -> &'static str {
        match resource {
            FavoriteResourceType::Tracks => "trackIds",
            FavoriteResourceType::Albums => "albumIds",
            FavoriteResourceType::Artists => "artistIds",
        }
    }

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

        let resource_id_param = self.get_resouce_id_param(&resource);

        let url = format!("/users/{user_id}/favorites/{resource}");
        self.request(reqwest::Method::POST, url)
            .with_country_code()
            .with_form_param(resource_id_param, resource_id.to_string())
            .with_base_url(API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(())
    }

    /// Removes a resource from user's favorite items
    pub async fn remove_from_favorites(
        &self,
        resource: FavoriteResourceType,
        resource_id: u32,
    ) -> Result<(), TidalError> {
        let user_id = self
            .session
            .auth
            .user_id
            .ok_or_else(|| TidalError::NotAuthenticated)?;

        let resource_id_param = self.get_resouce_id_param(&resource);

        let url = format!("/users/{user_id}/favorites/{resource}/{resource_id}");
        self.request(reqwest::Method::DELETE, url)
            .with_country_code()
            .with_param(resource_id_param, resource_id.to_string())
            .with_base_url(API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(())
    }
}
