use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::track::Track;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRole {
    pub category: String,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: String,
    pub picture: Option<String>,
}

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
