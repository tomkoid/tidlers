use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::client::models::artist::ArtistRole;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArtistsMonthlyResponse {
    pub activity: Option<TopArtistsActivity>,
    pub artists: Option<Vec<ArtistStreamStats>>,
    pub disclaimers: Option<Vec<Disclaimer>>,
    pub metadata: TopArtistsMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArtistsActivity {
    #[serde(rename = "@class")]
    pub class: String,
    pub activity_type: String,
    pub artists: Vec<TopArtistSummary>,
    pub images: Vec<ActivityImage>,
    pub month: u32,
    pub total_nr_of_days_left: Option<u32>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    pub year: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArtistSummary {
    pub id: u64,
    pub name: String,
    pub nr_of_streams: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityImage {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistStreamStats {
    pub artist: Artist,
    pub nr_of_streams: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub artist_roles: Vec<ArtistRole>,
    pub artist_types: Vec<String>,
    pub double_popularity: f64,
    pub id: u64,
    pub mixes: Option<HashMap<String, String>>,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: u32,
    pub selected_album_cover_fallback: Option<String>,
    pub vibrant_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disclaimer {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopArtistsMetadata {
    pub timeline: Vec<ActivityTimelineEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimelineResponse {
    timeline: Vec<ActivityTimelineEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimelineEntry {
    year: i32,
    month: u32,
    title: String,
}
