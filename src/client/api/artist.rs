use crate::{
    client::{
        TidalClient,
        models::artist::{ArtistResponse, ArtistTopTracksResponse},
    },
    error::TidalError,
};

impl TidalClient {
    pub async fn get_artist(&mut self, artist_id: String) -> Result<ArtistResponse, TidalError> {
        self.request(reqwest::Method::GET, format!("/artists/{}", artist_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_artist_tracks(
        &mut self,
        artist_id: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ArtistTopTracksResponse, TidalError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/artists/{}/toptracks", artist_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }
}
