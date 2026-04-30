use serde::{Deserialize, Serialize};

use crate::client::models::collection::Creator;

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
pub struct PlaylistCollectionItem {
    pub trn: String,
    pub item_type: String,
    pub added_at: String,
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub data: PlaylistData,
}
