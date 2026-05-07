use std::collections::HashMap;

use crate::client::models::{album::Album, artist::Artist, media::MediaMetadata};

pub mod config;
pub mod user_uploads;

/// Represents a music track
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
    pub stream_start_date: Option<String>,
    #[serde(rename = "premiumStreamingOnly")]
    pub premium_streaming_only: bool,
    #[serde(rename = "trackNumber")]
    pub track_number: u32,
    #[serde(rename = "volumeNumber")]
    pub volume_number: u32,
    pub version: Option<String>,
    pub popularity: u32,
    pub copyright: Option<String>,
    pub bpm: Option<f32>,
    pub key: Option<String>,
    #[serde(rename = "keyScale")]
    pub key_scale: Option<String>,
    pub url: String,
    pub isrc: Option<String>,
    pub editable: bool,
    pub explicit: bool,
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(rename = "audioModes")]
    pub audio_modes: Vec<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata>,
    pub upload: bool,
    #[serde(rename = "accessType")]
    pub access_type: Option<String>,
    pub spotlighted: Option<bool>,
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
pub struct TrackRadioResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<Track>,
}

pub type TrackPlaybackInfoPostPaywallResponse = TrackPlaybackInfoResponse;

/// Response containing track playback information including manifest data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackPlaybackInfoResponse {
    pub track_id: u64,
    pub asset_presentation: String,
    pub audio_mode: String,
    pub audio_quality: String,
    pub manifest_mime_type: String,
    pub manifest_hash: String,
    #[serde(skip_deserializing, default)]
    pub manifest: Option<JsonTrackManifest>,
    #[serde(skip_deserializing, default)]
    pub manifest_parsed: Option<ParsedTrackManifest>,
    pub album_replay_gain: f64,
    pub album_peak_amplitude: f64,
    pub track_replay_gain: f64,
    pub track_peak_amplitude: f64,
}

impl TrackPlaybackInfoResponse {
    /// Extracts all streaming URLs from the manifest
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playback = client.get_track_postpaywall_playback_info("123456789").await?;
    /// if let Some(urls) = playback.get_stream_urls() {
    ///     for url in urls {
    ///         println!("URL: {}", url);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_stream_urls(&self) -> Option<Vec<String>> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ParsedTrackManifest::Json(json_manifest) => json_manifest.urls.clone(),
            ParsedTrackManifest::Dash(dash_manifest) => dash_manifest.urls.clone(),
        })
    }

    /// Gets the primary/first streaming URL
    pub fn get_primary_url(&self) -> Option<String> {
        self.get_stream_urls()
            .and_then(|urls| urls.into_iter().next())
    }

    /// Gets the MIME type from the manifest
    pub fn get_mime_type(&self) -> Option<String> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ParsedTrackManifest::Json(json_manifest) => json_manifest.mime_type.clone(),
            ParsedTrackManifest::Dash(dash_manifest) => dash_manifest.mime_type.clone(),
        })
    }

    /// Gets the codec information from the manifest
    pub fn get_codecs(&self) -> Option<String> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ParsedTrackManifest::Json(json_manifest) => json_manifest.codecs.clone(),
            ParsedTrackManifest::Dash(dash_manifest) => dash_manifest.codecs.clone(),
        })
    }
}

/// JSON manifest containing track streaming information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTrackManifest {
    pub mime_type: String,
    pub codecs: String,
    pub encryption_type: String,
    pub urls: Vec<String>,
}

pub type ManifestType = ParsedTrackManifest;

/// Represents different types of streaming manifests
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ParsedTrackManifest {
    Json(JsonTrackManifest),
    Dash(DashManifest),
}

/// DASH (Dynamic Adaptive Streaming over HTTP) manifest for adaptive streaming
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DashManifest {
    pub mime_type: String,
    pub codecs: String,
    pub urls: Vec<String>,
    pub bitrate: Option<u32>,
    pub initialization_url: Option<String>,
    pub media_url_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timescale: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "startNumber"
    )]
    pub start_number: Option<u32>,
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
    ///
    /// # Example
    ///
    /// ```
    /// # use tidlers::client::models::track::DashManifest;
    /// # let manifest = DashManifest {
    /// #     mime_type: "audio/mp4".to_string(),
    /// #     codecs: "mp4a.40.2".to_string(),
    /// #     urls: vec![],
    /// #     bitrate: None,
    /// #     initialization_url: None,
    /// #     media_url_template: Some("segment_$Number$.m4s".to_string()),
    /// #     timescale: None,
    /// #     duration: None,
    /// #     start_number: None,
    /// # };
    /// if let Some(url) = manifest.get_segment_url(1) {
    ///     println!("Segment 1: {}", url); // "segment_1.m4s"
    /// }
    /// ```
    pub fn get_segment_url(&self, segment_number: u32) -> Option<String> {
        self.get_media_template()
            .map(|template| template.replace("$Number$", &segment_number.to_string()))
    }
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
