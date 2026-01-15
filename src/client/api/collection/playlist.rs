use crate::{
    client::{
        TidalClient,
        models::{
            collection::{PlaylistCollectionItem, SharingLevel},
            playlist::{
                PlaylistInfo, PlaylistItemsResponse, PlaylistsResponse, PublicUserPlaylistsResponse,
            },
        },
    },
    error::TidalError,
    ids::PlaylistId,
};

impl TidalClient {
    /// Creates a new playlist in the user's collection
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # use tidlers::client::models::collection::SharingLevel;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playlist = client.create_playlist(
    ///     "My Playlist",
    ///     "A cool playlist",
    ///     Some(SharingLevel::Private),
    ///     None
    /// ).await?;
    /// println!("Created playlist: {}", playlist.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_playlist(
        &self,
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

    /// Lists all playlists for the authenticated user
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playlists = client.list_playlists().await?;
    /// for playlist in playlists.items {
    ///     println!("{}: {} tracks", playlist.title, playlist.number_of_tracks);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_playlists(&self) -> Result<PlaylistsResponse, TidalError> {
        let url = format!("/users/{}/playlists", self.user_id()?);

        self.request(reqwest::Method::GET, url)
            .with_country_code()
            .send()
            .await
    }

    /// Lists public playlists for a specific user
    pub async fn list_public_playlists(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<PublicUserPlaylistsResponse, TidalError> {
        let url = format!("/user-playlists/{}/public", self.user_id()?);

        self.request(reqwest::Method::GET, url)
            .with_country_code()
            .with_param("limit", limit.unwrap_or(50).to_string())
            .with_param("offset", offset.unwrap_or(0).to_string())
            .with_base_url(Self::API_V2_LOCATION)
            .send()
            .await
    }

    pub async fn get_playlist(
        &self,
        playlist_id: impl Into<PlaylistId>,
    ) -> Result<PlaylistInfo, TidalError> {
        let playlist_id = playlist_id.into();
        self.request(reqwest::Method::GET, format!("/playlists/{}/", playlist_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_playlist_items(
        &self,
        playlist_id: impl Into<PlaylistId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<PlaylistItemsResponse, TidalError> {
        let playlist_id = playlist_id.into();
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/playlists/{}/items", playlist_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }

    pub async fn get_playlist_recommendations_items(
        &self,
        playlist_id: impl Into<PlaylistId>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<PlaylistItemsResponse, TidalError> {
        let playlist_id = playlist_id.into();
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        self.request(
            reqwest::Method::GET,
            format!("/playlists/{}/recommendations/items", playlist_id),
        )
        .with_country_code()
        .with_param("limit", limit.to_string())
        .with_param("offset", offset.to_string())
        .send()
        .await
    }
}
