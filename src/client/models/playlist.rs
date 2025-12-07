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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylistsResponse {
    pub items: Vec<PublicUserPlaylistItem>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylistItem {
    pub playlist: PublicUserPlaylist,
    pub follow_info: FollowInfo,
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylist {
    pub uuid: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub creator: PlaylistCreatorInfo,
    pub curators: Vec<Curator>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCreatorInfo {
    pub id: i64,
    pub name: Option<String>,
    pub picture: Option<String>,
    #[serde(rename = "type")]
    pub creator_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Curator {
    pub id: i64,
    pub name: String,
    pub handle: String,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowInfo {
    pub nr_of_followers: i64,
    pub tidal_resource_name: String,
    pub followed: bool,
    pub follow_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: i64,
    pub name: String,
    pub color: Vec<serde_json::Value>,
}
