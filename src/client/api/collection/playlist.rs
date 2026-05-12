use crate::{
    client::{
        TidalClient,
        models::{
            OrderDirection,
            collection::{SharingLevel, playlist::CollectionPlaylistEntry},
            playlist::{
                PlaylistItemsOrder, PlaylistItemsResponse, PlaylistItemsWithEtag, PlaylistResponse,
                PublicUserPlaylistsResponse, UserPlaylistsResponse,
            },
        },
    },
    error::TidalError,
    ids::PlaylistId,
    urls::API_V2_LOCATION,
};
use reqwest::header::{HeaderMap, HeaderValue, IF_NONE_MATCH};

impl TidalClient {
    /// Creates a new playlist in the user's collection
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::TidalClient;
    /// # use tidlers::auth::TidalAuth;
    /// # use tidlers::client::models::collection::SharingLevel;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playlist = client.create_playlist(
    ///     "My Playlist",
    ///     "A cool playlist",
    ///     Some(SharingLevel::Private),
    ///     None
    /// ).await?;
    /// println!("Created playlist: {}", playlist.data.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_playlist(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        sharing_level: Option<SharingLevel>,
        parent_id: Option<String>,
    ) -> Result<CollectionPlaylistEntry, TidalError> {
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
        .with_base_url(API_V2_LOCATION)
        .send()
        .await
    }

    /// Lists all playlists for the authenticated user
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playlists = client.list_playlists().await?;
    /// for playlist in playlists.items {
    ///     println!("{}: {} tracks", playlist.title, playlist.number_of_tracks);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_playlists(&self) -> Result<UserPlaylistsResponse, TidalError> {
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
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }

    pub async fn get_playlist(
        &self,
        playlist_id: impl Into<PlaylistId>,
    ) -> Result<PlaylistResponse, TidalError> {
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
        order: Option<PlaylistItemsOrder>,
        order_direction: Option<OrderDirection>,
    ) -> Result<PlaylistItemsResponse, TidalError> {
        let response = self
            .get_playlist_items_with_etag(playlist_id, limit, offset, order, order_direction)
            .await?;
        Ok(response.items)
    }

    pub async fn get_playlist_items_with_etag(
        &self,
        playlist_id: impl Into<PlaylistId>,
        limit: Option<u64>,
        offset: Option<u64>,
        order: Option<PlaylistItemsOrder>,
        order_direction: Option<OrderDirection>,
    ) -> Result<PlaylistItemsWithEtag, TidalError> {
        let playlist_id = playlist_id.into();
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        let (items, etag) = self
            .request(
                reqwest::Method::GET,
                format!("/playlists/{}/items", playlist_id),
            )
            .with_country_code()
            .with_param("limit", limit.to_string())
            .with_param("offset", offset.to_string())
            .with_param(
                "order",
                order.unwrap_or(PlaylistItemsOrder::Index).to_string(),
            )
            .with_param(
                "orderDirection",
                order_direction
                    .unwrap_or(OrderDirection::Ascending)
                    .to_string(),
            )
            .send_with_etag()
            .await?;

        let etag = etag.ok_or_else(|| {
            TidalError::InvalidResponse(
                "missing ETag header in get_playlist_items response".to_string(),
            )
        })?;

        Ok(PlaylistItemsWithEtag { items, etag })
    }

    pub async fn add_items_to_playlist(
        &self,
        playlist_id: impl Into<PlaylistId>,
        item_ids: Vec<String>,
        index: Option<u64>,
    ) -> Result<(), TidalError> {
        let playlist_id = playlist_id.into();
        let playlist_items = self
            .get_playlist_items_with_etag(playlist_id.clone(), Some(1), Some(0), None, None)
            .await?;
        self.add_items_to_playlist_with_etag(playlist_id, item_ids, index, &playlist_items.etag)
            .await?;

        Ok(())
    }

    /// Removes items queried by index from playlist
    /// NOTE: Index numbers are counted from 0!
    pub async fn remove_items_from_playlist(
        &self,
        playlist_id: impl Into<PlaylistId>,
        indices: Vec<u64>,
        order: Option<PlaylistItemsOrder>,
        order_direction: Option<OrderDirection>,
    ) -> Result<(), TidalError> {
        let playlist_id = playlist_id.into();
        let playlist_items = self
            .get_playlist_items_with_etag(playlist_id.clone(), Some(1), Some(0), None, None)
            .await?;
        self.remove_items_from_playlist_with_etag(
            playlist_id,
            indices,
            order,
            order_direction,
            &playlist_items.etag,
        )
        .await?;

        Ok(())
    }

    pub async fn add_items_to_playlist_with_etag(
        &self,
        playlist_id: impl Into<PlaylistId>,
        item_ids: Vec<String>,
        index: Option<u64>,
        etag: &str,
    ) -> Result<(), TidalError> {
        let playlist_id = playlist_id.into();
        let mut headers = HeaderMap::new();
        let etag_value = HeaderValue::from_str(etag).map_err(|error| {
            TidalError::InvalidArgument(format!("invalid etag header value: {error}"))
        })?;
        headers.insert(IF_NONE_MATCH, etag_value);

        self.request(
            reqwest::Method::POST,
            format!("/playlists/{}/items", playlist_id),
        )
        .with_country_code()
        .with_locale()
        .with_form_param("itemIds", item_ids.join(","))
        .with_form_param("toIndex", index.unwrap_or(0).to_string())
        .with_form_param("onArtifactNotFound", "SKIP".to_string())
        .with_headers(headers)
        .send_raw()
        .await?;

        Ok(())
    }

    /// Removes item queried by index from playlist with an ETag
    /// NOTE: Index numbers are counted from 0!
    pub async fn remove_items_from_playlist_with_etag(
        &self,
        playlist_id: impl Into<PlaylistId>,
        indices: Vec<u64>,
        order: Option<PlaylistItemsOrder>,
        order_direction: Option<OrderDirection>,
        etag: &str,
    ) -> Result<(), TidalError> {
        let playlist_id = playlist_id.into();
        let mut headers = HeaderMap::new();
        let etag_value = HeaderValue::from_str(etag).map_err(|error| {
            TidalError::InvalidArgument(format!("invalid etag header value: {error}"))
        })?;
        headers.insert(IF_NONE_MATCH, etag_value);

        self.request(
            reqwest::Method::DELETE,
            format!(
                "/playlists/{}/items/{}",
                playlist_id,
                indices
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        )
        .with_country_code()
        .with_locale()
        .with_param(
            "order",
            order.unwrap_or(PlaylistItemsOrder::Index).to_string(),
        )
        .with_param(
            "orderDirection",
            order_direction
                .unwrap_or(OrderDirection::Ascending)
                .to_string(),
        )
        .with_headers(headers)
        .send_raw()
        .await?;

        Ok(())
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
