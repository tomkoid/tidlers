use crate::{
    client::{TidalClient, models::collection::CollectionTracksResponse},
    error::TidalError,
};

impl TidalClient {
    pub async fn get_collection_favorites(
        &mut self,
        limit: Option<u32>,
    ) -> Result<CollectionTracksResponse, TidalError> {
        let url = format!(
            "/users/{}/favorites/tracks",
            self.user_info.as_ref().unwrap().user_id
        );

        let body: String = self
            .request(reqwest::Method::GET, url)
            .with_country_code()
            .with_locale()
            .with_param("limit", limit.unwrap_or(9999).to_string())
            .with_base_url(Self::API_V1_LOCATION)
            .send_raw()
            .await?;

        Ok(serde_json::from_str(&body)?)
    }
}
