use crate::{
    client::{TidalClient, models::collection::track::CollectionFavoriteTracksResponse},
    error::TidalError,
    urls::API_V1_LOCATION,
};

impl TidalClient {
    /// Retrieves the user's favorite tracks with optional limit (default 100) and offset (default
    /// 0)
    pub async fn get_collection_favorites(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<CollectionFavoriteTracksResponse, TidalError> {
        let url = format!("/users/{}/favorites/tracks", self.user_id()?);

        let body = self
            .request(reqwest::Method::GET, url)
            .with_country_code()
            .with_locale()
            .with_param("limit", limit.unwrap_or(100).to_string())
            .with_param("ofset", offset.unwrap_or(0).to_string())
            .with_base_url(API_V1_LOCATION)
            .send()
            .await?;

        Ok(body)
    }
}
