use crate::{
    client::{TidalClient, models::collection::CollectionArtistsResponse},
    error::TidalError,
};

impl TidalClient {
    pub async fn get_collection_artists(
        &mut self,
        limit: u32,
    ) -> Result<CollectionArtistsResponse, TidalError> {
        self.request(reqwest::Method::GET, "/my-collection/artists/folders")
            .with_country_code()
            .with_locale()
            .with_param("limit", limit.to_string())
            .with_param("order", "DATE")
            .with_param("folderId", "root")
            .with_base_url(Self::API_V2_LOCATION)
            .send()
            .await
    }
}
