use crate::{
    client::{TidalClient, models::collection::artist::CollectionArtistsResponse},
    error::TidalError,
    urls::API_V2_LOCATION,
};

impl TidalClient {
    /// Gets one page of the artists in the user's collection (followed artists).
    ///
    /// Pass `None` as `cursor` for the first page, then each response's
    /// [`cursor`](CollectionArtistsResponse::cursor) for the next, until it is
    /// `None`.
    pub async fn get_collection_artists(
        &self,
        limit: u32,
        cursor: Option<String>,
    ) -> Result<CollectionArtistsResponse, TidalError> {
        self.request(reqwest::Method::GET, "/my-collection/artists/folders")
            .with_country_code()
            .with_locale()
            .with_param("limit", limit.to_string())
            .with_optional_param("cursor", cursor)
            .with_param("order", "DATE")
            .with_param("folderId", "root")
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }
}
