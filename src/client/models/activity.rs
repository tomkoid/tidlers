use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArtistsResponse {
    pub activity: Activity,
    pub artists: Vec<ArtistWithStreams>,
    pub disclaimers: Vec<Disclaimer>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "@class")]
    pub class: String,
    pub activity_type: String,
    pub artists: Vec<TopArtist>,
    pub images: Vec<Image>,
    pub month: u32,
    pub nr_of_days_left: u32,
    pub subtitle: String,
    pub title: String,
    pub year: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArtist {
    pub id: u64,
    pub name: String,
    pub nr_of_streams: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithStreams {
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
#[serde(rename_all = "camelCase")]
pub struct ArtistRole {
    pub category: String,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disclaimer {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub timeline: Vec<ActivityTimelineItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimeline {
    timeline: Vec<ActivityTimelineItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityTimelineItem {
    year: i32,
    month: u32,
    title: String,
}
