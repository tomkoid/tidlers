use crate::client::{
    TidalClient,
    models::{
        collection::{FolderCollectionItem, PlaylistCollectionItem, SharingLevel},
        playlist::PlaylistInfo,
    },
};
use std::collections::HashMap;

use crate::{
    client::models::collection::{CollectionArtistsResponse, CollectionTracksResponse},
    error::TidalError,
    requests::TidalRequest,
    utils::debug_json_str,
};

impl TidalClient {
    pub async fn create_playlist(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
        sharing_level: Option<SharingLevel>,
        parent_id: Option<String>,
    ) -> Result<PlaylistCollectionItem, TidalError> {
        let url = format!("/my-collection/playlists/folders/create-playlist");

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("name".to_string(), title.into());
        params.insert("description".to_string(), description.into());
        params.insert(
            "folderId".to_string(),
            parent_id.unwrap_or("root".to_string()),
        );
        params.insert(
            "isPublic".to_string(),
            (sharing_level.unwrap_or(SharingLevel::Private) == SharingLevel::Public).to_string(),
        );

        req.method = reqwest::Method::PUT;
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V2_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn create_folder(
        &mut self,
        title: impl Into<String>,
        parent_id: Option<String>,
    ) -> Result<FolderCollectionItem, TidalError> {
        let url = format!("/my-collection/playlists/folders/create-folder");

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("name".to_string(), title.into());
        params.insert(
            "folderId".to_string(),
            parent_id.unwrap_or("root".to_string()),
        );

        req.method = reqwest::Method::PUT;
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V2_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_collection_artists(
        &mut self,
        limit: u32,
    ) -> Result<CollectionArtistsResponse, TidalError> {
        let url = "/my-collection/artists/folders".to_string();

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("locale".to_string(), self.session.locale.clone());
        params.insert("limit".to_string(), limit.to_string());
        params.insert("order".to_string(), "DATE".to_string());
        params.insert("folderId".to_string(), "root".to_string());

        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V2_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: CollectionArtistsResponse = serde_json::from_str(&body)?;

        Ok(json)
    }

    pub async fn get_collection_favorites(
        &mut self,
        limit: Option<u32>,
    ) -> Result<CollectionTracksResponse, TidalError> {
        let limit = limit.unwrap_or(9999);
        let url = format!(
            "/users/{}/favorites/tracks",
            self.user_info.as_ref().unwrap().user_id
        );

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("locale".to_string(), self.session.locale.clone());
        params.insert("limit".to_string(), limit.to_string());

        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::API_V1_LOCATION.to_string());

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;
        debug_json_str(&body);

        let json: CollectionTracksResponse = serde_json::from_str(&body)?;

        Ok(json)
    }
}
