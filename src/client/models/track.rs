use std::collections::HashMap;

use crate::client::models::{album::Album, artist::Artist, media::MediaMetadata};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub duration: u64,
    #[serde(rename = "replayGain")]
    pub replay_gain: f64,
    pub peak: f32,
    #[serde(rename = "allowStreaming")]
    pub allow_streaming: bool,
    #[serde(rename = "streamReady")]
    pub stream_ready: bool,
    #[serde(rename = "payToStream")]
    pub pay_to_stream: bool,
    #[serde(rename = "adSupportedStreamReady")]
    pub ad_supported_stream_ready: bool,
    #[serde(rename = "djReady")]
    pub dj_ready: bool,
    #[serde(rename = "stemReady")]
    pub stem_ready: bool,
    #[serde(rename = "streamStartDate")]
    pub stream_start_date: String,
    #[serde(rename = "premiumStreamingOnly")]
    pub premium_streaming_only: bool,
    #[serde(rename = "trackNumber")]
    pub track_number: u32,
    #[serde(rename = "volumeNumber")]
    pub volume_number: u32,
    pub version: Option<String>,
    pub popularity: u32,
    pub copyright: String,
    pub bpm: Option<f32>,
    pub url: String,
    pub isrc: String,
    pub editable: bool,
    pub explicit: bool,
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(rename = "audioModes")]
    pub audio_modes: Vec<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: MediaMetadata,
    pub upload: bool,
    #[serde(rename = "accessType")]
    pub access_type: String,
    pub spotlighted: bool,
    #[serde(rename = "dateAdded")]
    pub date_added: Option<String>,
    pub index: Option<u64>,
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub mixes: Option<HashMap<String, String>>,
    #[serde(rename = "itemUuid")]
    pub item_uuid: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TrackOld {
//     pub id: i64,
//     pub title: String,
//     pub duration: i32,
//     pub replay_gain: f64,
//     pub peak: f64,
//     pub allow_streaming: bool,
//     pub stream_ready: bool,
//     pub pay_to_stream: bool,
//     pub ad_supported_stream_ready: bool,
//     pub dj_ready: bool,
//     pub stem_ready: bool,
//     pub stream_start_date: String,
//     pub premium_streaming_only: bool,
//     pub track_number: i32,
//     pub volume_number: i32,
//     pub version: Option<String>,
//     pub popularity: i32,
//     pub copyright: String,
//     pub bpm: Option<i32>,
//     pub description: Option<String>,
//     pub url: String,
//     pub isrc: String,
//     pub editable: bool,
//     pub explicit: bool,
//     pub audio_quality: String,
//     pub audio_modes: Vec<String>,
//     pub media_metadata: MediaMetadata,
//     pub upload: bool,
//     pub access_type: String,
//     pub spotlighted: bool,
//     pub artist: Artist,
//     pub artists: Vec<Artist>,
//     pub album: Album,
//     pub mixes: Option<Mixes>,
//     pub date_added: Option<String>,
//     pub index: Option<i64>,
//     pub item_uuid: Option<String>,
// }
//
