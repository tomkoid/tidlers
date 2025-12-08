use serde::{Deserialize, Serialize};

use crate::client::models::{artist::Artist, media::MediaMetadata, track::Track};

/// Used generically to represent an album in various responses
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: Option<String>,
}

/// Response from TIDAL when requesting album info
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumInfoResponse {
    pub id: i64,
    pub title: String,
    pub duration: u64,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub allow_streaming: bool,
    pub premium_streaming_only: bool,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub number_of_volumes: u32,
    pub release_date: String,
    pub copyright: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub version: Option<String>,
    pub url: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub explicit: bool,
    pub upc: String,
    pub popularity: u32,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub upload: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbum {
    pub id: i64,
    pub title: String,
    pub duration: u64,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub allow_streaming: bool,
    pub premium_streaming_only: bool,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub number_of_volumes: u32,
    pub release_date: String,
    pub copyright: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub version: Option<String>,
    pub url: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub explicit: bool,
    pub upc: String,
    pub popularity: u32,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub media_metadata: MediaMetadata,
    pub upload: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsResponse {
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
    pub items: Vec<AlbumItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub album_type: String,
}

impl Album {
    pub fn get_cover_url(&self, size_x: u32, size_y: u32) -> String {
        // split string by dashes
        let cover_parts: Vec<&str> = self.cover.split('-').collect();
        let mut cover_path = String::new();
        for part in cover_parts.iter() {
            cover_path.push_str(part);
        }

        let size = format!("{}x{}", size_x, size_y);
        format!("https://resources.tidal.com/images/{}/{}", cover_path, size)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumCreditContributor {
    pub name: String,
    pub id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCredit {
    #[serde(rename = "type")]
    pub credit_type: String,
    pub contributors: Vec<AlbumCreditContributor>,
}

pub type AlbumCreditsResponse = Vec<AlbumCredit>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemWithCredits {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: String,
    pub credits: Vec<AlbumCredit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsWithCreditsResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<AlbumItemWithCredits>,
}
