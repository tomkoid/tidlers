use crate::{TidalClient, TidalError, client::models::ResourceType, urls::API_V1_LOCATION};

impl TidalClient {
    fn get_resource_id_param(&self, resource: &ResourceType) -> &'static str {
        match resource {
            ResourceType::Tracks => "trackIds",
            ResourceType::Albums => "albumIds",
            ResourceType::Artists => "artistIds",
            ResourceType::Playlists => "uuid",
        }
    }

    /// Adds a resource into user's favorite items
    pub async fn add_to_favorites(
        &self,
        resource: ResourceType,
        resource_id: impl Into<String>,
    ) -> Result<(), TidalError> {
        let user_id = self
            .session
            .auth
            .user_id
            .ok_or_else(|| TidalError::NotAuthenticated)?;

        let resource_id_param = self.get_resource_id_param(&resource);

        let url = format!("/users/{user_id}/favorites/{resource}");
        self.request(reqwest::Method::POST, url)
            .with_country_code()
            .with_form_param(resource_id_param, resource_id)
            .with_base_url(API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(())
    }

    /// Removes a resource from user's favorite items
    pub async fn remove_from_favorites(
        &self,
        resource: ResourceType,
        resource_id: impl Into<String>,
    ) -> Result<(), TidalError> {
        let user_id = self
            .session
            .auth
            .user_id
            .ok_or_else(|| TidalError::NotAuthenticated)?;

        let resource_id_param = self.get_resource_id_param(&resource);
        let res_id = resource_id.into();

        let url = format!("/users/{user_id}/favorites/{resource}/{res_id}");
        self.request(reqwest::Method::DELETE, url)
            .with_country_code()
            .with_param(resource_id_param, res_id)
            .with_base_url(API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(())
    }
}
