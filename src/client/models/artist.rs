use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::models::album::Album;

use super::{album::ArtistAlbum, track::Track};

/// Represents an artist's role category
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRole {
    pub category: String,
    pub category_id: i32,
}

/// Represents a music artist
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: String,
    pub picture: Option<String>,
}

/// Detailed artist information response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistResponse {
    pub artist_roles: Vec<ArtistRole>,
    pub artist_types: Vec<String>,
    pub handle: Option<String>,
    pub id: u64,
    pub mixes: HashMap<String, String>,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: u32,
    pub selected_album_cover_fallback: Option<String>,
    pub spotlighted: bool,
    pub url: String,
    pub user_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistTopTracksResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistBioResponse {
    pub source: String,
    pub last_updated: String,
    pub text: String,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistLink {
    pub url: String,
    pub site_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistLinksResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<ArtistLink>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbumsResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<ArtistAlbum>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistVideo {
    pub id: u64,
    pub title: String,
    pub volume_number: u32,
    pub track_number: u32,
    pub release_date: String,
    pub image_path: Option<String>,
    pub image_id: Option<String>,
    pub vibrant_color: Option<String>,
    pub duration: u32,
    pub quality: String,
    pub stream_ready: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub allow_streaming: bool,
    pub explicit: bool,
    pub popularity: u32,
    #[serde(rename = "type")]
    pub video_type: String,
    pub ads_url: Option<String>,
    pub ads_pre_paywall_only: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistVideosResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<ArtistVideo>,
}
