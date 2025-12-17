pub mod config;

use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsFull {
    pub tracks: Option<SearchSection<SearchTrackHit>>,
    pub uploads: Option<SearchSection<SearchTrackHit>>,
    pub albums: Option<SearchSection<SearchAlbumHit>>,
    pub playlists: Option<SearchSection<SearchPlaylistHit>>,
    pub videos: Option<SearchSection<SearchVideoHit>>,
    pub artists: Option<SearchSection<SearchArtistHit>>,
    pub genres: Option<Vec<String>>,
    #[serde(rename = "contentTypeFilters")]
    pub content_type_filters: Option<Vec<String>>,
    #[serde(rename = "topHits")]
    pub top_hits: Option<Vec<SearchTopHit>>,
    #[serde(rename = "queryId")]
    pub query_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSection<T> {
    pub items: Vec<T>,
    pub total_number_of_items: u32,
    pub cacheable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTrackHit {
    pub artifact_type: String,
    pub id: u64,
    pub editable: bool,
    pub title: String,
    pub album: SearchAlbumSummary,
    pub artists: Vec<SearchArtistSummary>,
    pub version: Option<String>,
    pub duration: u64,
    pub popularity: u32,
    pub double_popularity: f64,
    pub track_number: Option<u32>,
    pub volume_number: Option<u32>,
    pub explicit: bool,
    pub replay_gain: f64,
    pub audio_quality: Option<String>,
    #[serde(rename = "allowStreaming")]
    pub allow_streaming: Option<bool>,
    #[serde(rename = "streamStartDate")]
    pub stream_start_date: Option<String>,
    pub stream_ready: Option<bool>,
    pub audio_modes: Option<Vec<String>>,
    pub mixes: Option<HashMap<String, String>>,
    pub ad_supported_stream_ready: Option<bool>,
    pub media_metadata: Option<crate::client::models::media::MediaMetadata>,
    pub provider_name: Option<String>,
    pub dj_ready: Option<bool>,
    pub stem_ready: Option<bool>,
    pub pay_to_stream: Option<bool>,
    pub genres: Option<Vec<String>>,
    pub audio_analysis_attributes: Option<Value>,
    pub upload: Option<bool>,
    pub access_type: Option<String>,
    pub spotlighted: Option<bool>,
    pub created_at: Option<String>,
    pub user_id: Option<u64>,
    pub isrc: Option<String>,
    pub peak: Option<f64>,
    pub pre_paywall_presentation: Option<String>,
    pub copyright: Option<String>,
    pub first_available: Option<String>,
    pub linked_stereo_isrc: Option<String>,
    pub premium_streaming_only: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAlbumSummary {
    pub id: u64,
    pub title: String,
    pub version: Option<String>,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: Option<String>,
    pub upload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtistSummary {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub handle: Option<String>,
    pub picture: Option<String>,
    pub user_id: Option<u64>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAlbumHit {
    pub artifact_type: String,
    pub id: u64,
    pub title: String,
    pub artists: Vec<SearchArtistSummary>,
    pub duration: Option<u64>,
    pub cover: Option<String>,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub copyright: Option<String>,
    pub number_of_volumes: Option<u32>,
    pub number_of_tracks: Option<u32>,
    pub number_of_videos: Option<u32>,
    pub popularity: Option<u32>,
    pub double_popularity: Option<f64>,
    pub version: Option<String>,
    pub release_date: Option<String>,
    #[serde(rename = "type")]
    pub album_type: Option<String>,
    pub explicit: Option<bool>,
    pub upc: Option<String>,
    pub audio_quality: Option<String>,
    pub master_album: Option<SearchMasterAlbum>,
    pub allow_streaming: Option<bool>,
    pub stream_start_date: Option<String>,
    pub stream_ready: Option<bool>,
    pub pay_to_stream: Option<bool>,
    pub audio_modes: Option<Vec<String>>,
    pub ad_supported_stream_ready: Option<bool>,
    pub media_metadata: Option<crate::client::models::media::MediaMetadata>,
    pub provider_name: Option<String>,
    pub dj_ready: Option<bool>,
    pub stem_ready: Option<bool>,
    pub upload: Option<bool>,
    pub access_type: Option<String>,
    pub created_at: Option<String>,
    pub user_id: Option<u64>,
    pub premium_streaming_only: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMasterAlbum {
    pub id: String,
    pub release_date: Option<String>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPlaylistHit {
    pub artifact_type: String,
    pub uuid: String,
    pub title: String,
    pub number_of_tracks: Option<u32>,
    pub number_of_audio_tracks: Option<u32>,
    pub number_of_video_tracks: Option<u32>,
    pub description: Option<String>,
    pub duration: Option<u64>,
    pub last_updated: Option<String>,
    pub created: Option<String>,
    pub image: Option<String>,
    pub square_image: Option<String>,
    pub user_id: Option<u64>,
    pub created_by_artists: Option<Vec<SearchArtistSummary>>,
    pub popularity: Option<u32>,
    pub double_popularity: Option<f64>,
    pub public_playlist: Option<bool>,
    pub promoted_artists: Option<Vec<SearchArtistSummary>>,
    pub last_item_added_at: Option<String>,
    pub url: Option<String>,
    pub creator: Option<Value>,
    #[serde(rename = "type")]
    pub playlist_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchVideoHit {
    pub artifact_type: String,
    pub id: u64,
    pub audio_only_track_id: Option<u64>,
    pub duration: u64,
    pub ads_url: Option<String>,
    pub ads_pre_paywall_only: Option<bool>,
    pub title: String,
    pub artists: Vec<SearchArtistSummary>,
    pub album: Option<SearchAlbumSummary>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub video_type: Option<String>,
    pub volume_number: Option<u32>,
    pub track_number: Option<u32>,
    pub isrc: Option<String>,
    pub copyright: Option<String>,
    pub explicit: bool,
    pub release_date: Option<String>,
    pub image: Option<String>,
    pub vibrant_color: Option<String>,
    pub popularity: Option<u32>,
    pub double_popularity: Option<f64>,
    pub allow_streaming: Option<bool>,
    pub stream_start_date: Option<String>,
    pub stream_ready: Option<bool>,
    pub ad_supported_stream_ready: Option<bool>,
    pub provider_id: Option<u64>,
    pub provider_name: Option<String>,
    pub dj_ready: Option<bool>,
    pub stem_ready: Option<bool>,
    pub created_at: Option<String>,
    pub quality: Option<String>,
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtistHit {
    pub artifact_type: String,
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: Option<u32>,
    pub double_popularity: Option<f64>,
    pub artist_types: Option<Vec<String>>,
    pub artist_roles: Option<Vec<SearchArtistRole>>,
    pub mixes: Option<HashMap<String, String>>,
    pub vibrant_color: Option<String>,
    pub selected_album_cover_fallback: Option<String>,
    pub handle: Option<String>,
    pub user_id: Option<u64>,
    pub artwork_id: Option<String>,
    pub spotlighted: Option<bool>,
    pub contributions_enabled: Option<bool>,
    pub cash_app_onboarded: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchArtistRole {
    pub category_id: i64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTopHit {
    pub value: Value,
    #[serde(rename = "type")]
    pub hit_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionsFull {
    pub history: Vec<SearchSuggestionEntry>,
    pub suggestions: Vec<SearchSuggestionEntry>,
    #[serde(rename = "directHits")]
    pub direct_hits: Vec<SearchSuggestionDirectHit>,
    #[serde(rename = "suggestionUuid")]
    pub suggestion_uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionEntry {
    pub query: String,
    pub highlights: Vec<SearchHighlight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHighlight {
    pub start: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionDirectHit {
    pub value: Value,
    #[serde(rename = "type")]
    pub hit_type: String,
}

#[derive(Clone, Debug)]
pub enum SearchType {
    Albums,
    Artists,
    Playlists,
    TopHits,
    Tracks,
    Videos,
}

impl FromStr for SearchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "albums" => Ok(SearchType::Albums),
            "artists" => Ok(SearchType::Artists),
            "playlists" => Ok(SearchType::Playlists),
            "topHits" | "tophits" | "top_hits" => Ok(SearchType::TopHits),
            "tracks" => Ok(SearchType::Tracks),
            "videos" => Ok(SearchType::Videos),
            _ => Err(format!("unknown search type: {s}")),
        }
    }
}

impl Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SearchType::Albums => "albums",
            SearchType::Artists => "artists",
            SearchType::Playlists => "playlists",
            SearchType::TopHits => "topHits",
            SearchType::Tracks => "tracks",
            SearchType::Videos => "videos",
        };
        write!(f, "{}", s)
    }
}
