use std::collections::HashMap;

use crate::client::models::{album::Album, artist::Artist, media::MediaMetadata};

pub mod config;
pub mod playback;
pub mod user_uploads;

/// Represents a track
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub duration: u64,
    pub replay_gain: f64,
    pub peak: f32,
    pub allow_streaming: bool,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: Option<String>,
    pub premium_streaming_only: bool,
    pub track_number: u32,
    pub volume_number: u32,
    pub version: Option<String>,
    pub popularity: u32,
    pub copyright: Option<String>,
    pub bpm: Option<f32>,
    pub key: Option<String>,
    pub key_scale: Option<String>,
    pub url: String,
    pub isrc: Option<String>,
    pub editable: bool,
    pub explicit: bool,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub media_metadata: Option<MediaMetadata>,
    pub upload: bool,
    pub access_type: Option<String>,
    pub spotlighted: Option<bool>,
    pub date_added: Option<String>,
    pub index: Option<u64>,
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    pub mixes: Option<HashMap<String, String>>,
    pub item_uuid: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackRadioResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<Track>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResponse {
    pub track_id: u32,
    pub lyrics_provider: String,
    pub provider_commontrack_id: String,
    pub provider_lyrics_id: String,
    pub lyrics: String,
    #[serde(rename = "isRightToLeft")]
    pub right_to_left: bool,
}

#[cfg(test)]
mod tests {
    use crate::client::models::track::{
        DashManifest, JsonTrackManifest, ParsedTrackManifest, TrackPlaybackInfoResponse,
    };

    #[test]
    fn playback_info_helpers_work_for_json_manifest() {
        let manifest = JsonTrackManifest {
            mime_type: "audio/flac".to_string(),
            codecs: "flac".to_string(),
            encryption_type: "NONE".to_string(),
            urls: vec!["https://example.com/a.flac".to_string()],
        };

        let playback = TrackPlaybackInfoResponse {
            track_id: 1,
            asset_presentation: "FULL".to_string(),
            audio_mode: "STEREO".to_string(),
            audio_quality: "LOSSLESS".to_string(),
            manifest_mime_type: "application/json".to_string(),
            manifest_hash: "hash".to_string(),
            manifest: Some(manifest.clone()),
            manifest_parsed: Some(ParsedTrackManifest::Json(manifest)),
            album_replay_gain: 0.0,
            album_peak_amplitude: 0.0,
            track_replay_gain: 0.0,
            track_peak_amplitude: 0.0,
        };

        assert_eq!(
            playback.get_primary_url().as_deref(),
            Some("https://example.com/a.flac")
        );
        assert_eq!(playback.get_mime_type().as_deref(), Some("audio/flac"));
        assert_eq!(playback.get_codecs().as_deref(), Some("flac"));
    }

    #[test]
    fn dash_manifest_segment_url_uses_template() {
        let dash = DashManifest {
            mime_type: "audio/mp4".to_string(),
            codecs: "flac".to_string(),
            urls: vec![],
            bitrate: Some(1),
            initialization_url: Some("init.mp4".to_string()),
            media_url_template: Some("seg-$Number$.m4s".to_string()),
            timescale: Some(1),
            duration: Some(1),
            start_number: Some(1),
        };

        assert_eq!(dash.get_segment_url(42).as_deref(), Some("seg-42.m4s"));
    }
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
