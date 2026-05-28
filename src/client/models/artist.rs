use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::models::album::Album;

use super::{album::ArtistAlbum, track::Track};

/// Represents an artist's role category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRole {
    pub category: String,
    pub category_id: i32,
}

// "artistRoles": null,
// "artistTypes": [
//   "ARTIST",
//   "CONTRIBUTOR"
// ],
// "banner": null,
// "handle": null,
// "id": 15078437,
// "mixes": {
//   "ARTIST_MIX": "000914bdf1141cb958e3a0aefc8a92"
// },
// "name": "Natori",
// "picture": null,
// "popularity": 0,
// "relationType": "SIMILAR_ARTIST",
// "selectedAlbumCoverFallback": null,
// "spotlighted": false,
// "type": null,
// "url": "http://www.tidal.com/artist/15078437",
// "userId": null

/// Represents a music artist
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    pub picture: Option<String>,

    // TODO implement these fields:
    // mixes
    // banner
    // selectedAlbumCoverFallback
    pub user_id: Option<u64>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
    pub artist_roles: Option<Vec<ArtistRole>>,
    // TODO: make a type for artist type
    pub artist_types: Option<Vec<String>>,
    // TODO: make a type for relation type
    pub relation_type: Option<String>,
    pub spotlighted: Option<bool>,
    pub url: Option<String>,
}

/// Detailed artist information response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistResponse {
    pub artist_roles: Vec<ArtistRole>,
    pub artist_types: Vec<String>,
    pub handle: Option<String>,
    pub id: u64,
    pub mixes: Option<HashMap<String, String>>,
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
pub struct ArtistBiographyResponse {
    pub source: String,
    pub last_updated: String,
    pub text: String,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistExternalLink {
    pub url: String,
    pub site_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistLinksResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<ArtistExternalLink>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarArtistsResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<Artist>,
}
