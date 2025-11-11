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
