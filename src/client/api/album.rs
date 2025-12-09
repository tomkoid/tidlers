use crate::{
    client::{
        TidalClient,
        models::album::{
            AlbumCreditsResponse, AlbumInfoResponse, AlbumItemsResponse,
            AlbumItemsWithCreditsResponse,
        },
    },
    error::TidalError,
    ids::AlbumId,
};

impl TidalClient {
    pub async fn get_album(&self, album_id: impl Into<AlbumId>) -> Result<AlbumInfoResponse, TidalError> {
        let album_id = album_id.into();
        self.request(reqwest::Method::GET, format!("/albums/{}/", album_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_album_items(&self,
        album_id: impl Into<AlbumId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<AlbumItemsResponse, TidalError> {
        let album_id = album_id.into();
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

    pub async fn get_album_credits(&self,
        album_id: impl Into<AlbumId>,
    ) -> Result<AlbumCreditsResponse, TidalError> {
        let album_id = album_id.into();
        self.request(
            reqwest::Method::GET,
            format!("/albums/{}/credits", album_id),
        )
        .with_country_code()
        .send()
        .await
    }

    pub async fn get_album_items_credits(&self,
        album_id: impl Into<AlbumId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<AlbumItemsWithCreditsResponse, TidalError> {
        let album_id = album_id.into();
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/albums/{}/items/credits", album_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }
}
