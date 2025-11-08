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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackPlaybackInfoPostPaywallResponse {
    pub track_id: u64,
    pub asset_presentation: String,
    pub audio_mode: String,
    pub audio_quality: String,
    pub manifest_mime_type: String,
    pub manifest_hash: String,
    #[serde(skip_deserializing, default)]
    pub manifest: Option<TrackManifest>,
    #[serde(skip_deserializing, default)]
    pub manifest_parsed: Option<ManifestType>,
    pub album_replay_gain: f64,
    pub album_peak_amplitude: f64,
    pub track_replay_gain: f64,
    pub track_peak_amplitude: f64,
}

impl TrackPlaybackInfoPostPaywallResponse {
    pub fn get_stream_urls(&self) -> Option<Vec<String>> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ManifestType::Json(json_manifest) => json_manifest.urls.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.urls.clone(),
        })
    }

    pub fn get_primary_url(&self) -> Option<String> {
        self.get_stream_urls()
            .and_then(|urls| urls.into_iter().next())
    }

    pub fn get_mime_type(&self) -> Option<String> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ManifestType::Json(json_manifest) => json_manifest.mime_type.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.mime_type.clone(),
        })
    }

    pub fn get_codecs(&self) -> Option<String> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ManifestType::Json(json_manifest) => json_manifest.codecs.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.codecs.clone(),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackManifest {
    pub mime_type: String,
    pub codecs: String,
    pub encryption_type: String,
    pub urls: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ManifestType {
    Json(TrackManifest),
    Dash(DashManifest),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DashManifest {
    pub mime_type: String,
    pub codecs: String,
    pub urls: Vec<String>,
    pub bitrate: Option<u32>,
    pub initialization_url: Option<String>,
    pub media_url_template: Option<String>,
}

impl DashManifest {
    /// Get the initialization segment URL (for DASH streaming)
    pub fn get_init_url(&self) -> Option<&String> {
        self.initialization_url
            .as_ref()
            .or_else(|| self.urls.first())
    }

    /// Get the media segment URL template (contains $Number$ placeholder)
    pub fn get_media_template(&self) -> Option<&String> {
        self.media_url_template
            .as_ref()
            .or_else(|| self.urls.get(1))
    }

    /// Get a specific segment URL by replacing $Number$ with the segment number
    pub fn get_segment_url(&self, segment_number: u32) -> Option<String> {
        self.get_media_template()
            .map(|template| template.replace("$Number$", &segment_number.to_string()))
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
