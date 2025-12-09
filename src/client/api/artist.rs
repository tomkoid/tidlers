use crate::{
    client::{
        TidalClient,
        models::{
            artist::{
                ArtistAlbumsResponse, ArtistBioResponse, ArtistLinksResponse, ArtistResponse,
                ArtistTopTracksResponse, ArtistVideosResponse,
            },
            mixes::TrackMixInfo,
        },
    },
    error::TidalError,
    ids::ArtistId,
};

impl TidalClient {
    pub async fn get_artist(&mut self, artist_id: impl Into<ArtistId>) -> Result<ArtistResponse, TidalError> {
        let artist_id = artist_id.into();
        self.request(reqwest::Method::GET, format!("/artists/{}", artist_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_artist_bio(
        &mut self,
        artist_id: impl Into<ArtistId>,
    ) -> Result<ArtistBioResponse, TidalError> {
        let artist_id = artist_id.into();
        self.request(reqwest::Method::GET, format!("/artists/{}/bio", artist_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_artist_links(
        &mut self,
        artist_id: impl Into<ArtistId>,
    ) -> Result<ArtistLinksResponse, TidalError> {
        let artist_id = artist_id.into();
        self.request(
            reqwest::Method::GET,
            format!("/artists/{}/links", artist_id),
        )
        .with_country_code()
        .send()
        .await
    }

    pub async fn get_artist_tracks(
        &mut self,
        artist_id: impl Into<ArtistId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ArtistTopTracksResponse, TidalError> {
        let artist_id = artist_id.into();
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

    pub async fn get_artist_albums(
        &mut self,
        artist_id: impl Into<ArtistId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ArtistAlbumsResponse, TidalError> {
        let artist_id = artist_id.into();
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/artists/{}/albums", artist_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }

    pub async fn get_artist_videos(
        &mut self,
        artist_id: impl Into<ArtistId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<ArtistVideosResponse, TidalError> {
        let artist_id = artist_id.into();
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/artists/{}/videos", artist_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }

    pub async fn get_artist_mix(&mut self, artist_id: impl Into<ArtistId>) -> Result<TrackMixInfo, TidalError> {
        let artist_id = artist_id.into();
        self.request(reqwest::Method::GET, format!("/artists/{}/mix", artist_id))
            .with_country_code()
            .send()
            .await
    }
}
