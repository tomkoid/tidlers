use serde::{Deserialize, Serialize};

use crate::client::models::track::Track;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsResponse {
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: String,
    pub cut: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistsResponse {
    pub items: Vec<PlaylistInfo>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistInfo {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: u64,
    #[serde(rename = "numberOfVideos")]
    pub number_of_videos: u64,
    pub creator: PlaylistCreator,
    pub description: String,
    pub duration: u64,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub created: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    #[serde(rename = "publicPlaylist")]
    pub public_playlist: bool,
    pub url: String,
    pub image: String,
    pub popularity: u64,
    #[serde(rename = "squareImage")]
    pub square_image: String,
    #[serde(rename = "customImageUrl")]
    pub custom_image_url: Option<String>,
    #[serde(rename = "promotedArtists")]
    pub promoted_artists: Vec<String>,
    #[serde(rename = "lastItemAddedAt")]
    pub last_item_added_at: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistCreator {
    pub id: u64,
}
