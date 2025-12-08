use crate::{
    client::{
        TidalClient,
        models::album::{AlbumCreditsResponse, AlbumInfoResponse, AlbumItemsResponse},
    },
    error::TidalError,
};

impl TidalClient {
    pub async fn get_album(&mut self, album_id: String) -> Result<AlbumInfoResponse, TidalError> {
        self.request(reqwest::Method::GET, format!("/albums/{}/", album_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_album_items(
        &mut self,
        album_id: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<AlbumItemsResponse, TidalError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(reqwest::Method::GET, format!("/albums/{}/items", album_id))
            .with_country_code()
            .with_param("limit", limit.to_string())
            .with_param("offset", offset.to_string())
            .send()
            .await
    }

    pub async fn get_album_credits(
        &mut self,
        album_id: String,
    ) -> Result<AlbumCreditsResponse, TidalError> {
        self.request(
            reqwest::Method::GET,
            format!("/albums/{}/credits", album_id),
        )
        .with_country_code()
        .send()
        .await
    }
}
