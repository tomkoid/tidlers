use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::models::track::Track;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionTracksResponse {
    pub items: Vec<CollectionTrackItem>,
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionTrackItem {
    pub created: String,
    pub item: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistsResponse {
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: String,
    pub items: Vec<CollectionArtistMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoritesResponse {
    #[serde(rename = "lastModifiedAt")]
    pub items: Vec<CollectionArtistMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistMetadata {
    pub trn: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "addedAt")]
    pub added_at: String,
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub data: CollectionArtistData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistData {
    pub id: i64,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: i64,
    #[serde(rename = "doublePopularity")]
    pub double_popularity: f64,
    #[serde(rename = "artistTypes")]
    pub artist_types: Vec<String>,
    #[serde(rename = "artistRoles")]
    pub artist_roles: Vec<CollectionArtistRole>,
    pub mixes: Option<HashMap<String, String>>,
    #[serde(rename = "vibrantColor")]
    pub vibrant_color: Option<String>,
    #[serde(rename = "selectedAlbumCoverFallback")]
    pub selected_album_cover_fallback: Option<String>,
    pub handle: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i64>,
    pub url: String,
    pub trn: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistRole {
    #[serde(rename = "categoryId")]
    pub category_id: i64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollectionItem {
    pub trn: String,
    pub item_type: String,
    pub added_at: String,
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub data: PlaylistData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistData {
    pub uuid: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub creator: Creator,
    pub curators: Vec<serde_json::Value>,
    pub content_behavior: String,
    pub sharing_level: String,
    pub status: String,
    pub source: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub square_image: String,
    pub custom_image_url: Option<String>,
    pub url: String,
    pub created: String,
    pub last_updated: String,
    pub last_item_added_at: Option<String>,
    pub duration: i64,
    pub number_of_tracks: i64,
    pub number_of_videos: i64,
    pub promoted_artists: Vec<serde_json::Value>,
    pub trn: String,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub id: i64,
    pub name: Option<String>,
    pub picture: Option<String>,
    #[serde(rename = "type")]
    pub creator_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SharingLevel {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderCollectionItem {
    pub added_at: String,
    pub data: FolderData,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub trn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderData {
    pub created_at: String,
    pub id: String,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub total_number_of_items: i64,
    pub trn: String,
}
