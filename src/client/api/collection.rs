use crate::client::{
    TidalClient,
    models::collection::{FolderCollectionItem, PlaylistCollectionItem, SharingLevel},
};

use crate::{
    client::models::collection::{CollectionArtistsResponse, CollectionTracksResponse},
    error::TidalError,
};

impl TidalClient {
    pub async fn create_playlist(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
        sharing_level: Option<SharingLevel>,
        parent_id: Option<String>,
    ) -> Result<PlaylistCollectionItem, TidalError> {
        self.request(
            reqwest::Method::PUT,
            "/my-collection/playlists/folders/create-playlist",
        )
        .with_country_code()
        .with_param("name", title.into())
        .with_param("description", description.into())
        .with_param("folderId", parent_id.unwrap_or("root".to_string()))
        .with_param(
            "isPublic",
            (sharing_level.unwrap_or(SharingLevel::Private) == SharingLevel::Public).to_string(),
        )
        .with_base_url(Self::API_V2_LOCATION)
        .send()
        .await
    }

    pub async fn create_folder(
        &mut self,
        title: impl Into<String>,
        parent_id: Option<String>,
    ) -> Result<FolderCollectionItem, TidalError> {
        self.request(
            reqwest::Method::PUT,
            "/my-collection/playlists/folders/create-folder",
        )
        .with_country_code()
        .with_param("name", title.into())
        .with_param("folderId", parent_id.unwrap_or("root".to_string()))
        .with_base_url(Self::API_V2_LOCATION)
        .send()
        .await
    }

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
