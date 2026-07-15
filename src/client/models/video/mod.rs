use crate::client::models::{album::Album, artist::Artist};

pub mod config;
pub mod playback;

/// Represents a video
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: u64,
    pub title: String,
    pub volume_number: u32,
    pub track_number: u32,
    pub release_date: String,
    pub image_path: Option<String>, // not sure about this, might change this later
    pub image_id: Option<String>,
    pub vibrant_color: Option<String>,
    pub duration: u64,
    pub quality: String,
    pub stream_ready: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: Option<String>,
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
