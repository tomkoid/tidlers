use std::collections::HashMap;

use crate::client::models::{album::Album, artist::Artist, media::MediaMetadata};

/// Response containing user uploaded tracks
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploads {
    pub data: Vec<serde_json::Value>,
    #[serde(default)]
    pub included: Vec<UserUploadResource>,
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// Represents a user upload source file resource
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<UserUploadAttributes>,
    #[serde(default)]
    pub relationships: Option<UserUploadRelationships>,
}

/// Attributes of a user upload source file
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadAttributes {
    #[serde(default)]
    pub md5_hash: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub upload_link: Option<UploadLink>,
    #[serde(default)]
    pub status: Option<UploadStatus>,
}

/// Upload link information including URL and metadata
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLink {
    pub href: String,
    pub meta: UploadMeta,
}

/// Metadata for upload link
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadMeta {
    pub method: String,
    pub headers: UploadHeaders,
}

/// Headers required for upload
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UploadHeaders {
    pub content_length: String,
    pub content_md5: String,
}

/// Status information for uploaded file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadStatus {
    pub technical_file_status: String,
    pub moderation_file_status: String,
}

/// Relationships for user uploads
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadRelationships {
    #[serde(default)]
    pub owners: Option<OwnerRelationship>,
}

/// Owner relationship information
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerRelationship {
    pub links: RelationshipLinks,
}

/// Links for relationships
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

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

/// Response containing track playback information including manifest data
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
    /// Extracts all streaming URLs from the manifest
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
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
            ManifestType::Json(json_manifest) => json_manifest.urls.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.urls.clone(),
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
            ManifestType::Json(json_manifest) => json_manifest.mime_type.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.mime_type.clone(),
        })
    }

    /// Gets the codec information from the manifest
    pub fn get_codecs(&self) -> Option<String> {
        self.manifest_parsed.as_ref().map(|m| match m {
            ManifestType::Json(json_manifest) => json_manifest.codecs.clone(),
            ManifestType::Dash(dash_manifest) => dash_manifest.codecs.clone(),
        })
    }
}

/// JSON manifest containing track streaming information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackManifest {
    pub mime_type: String,
    pub codecs: String,
    pub encryption_type: String,
    pub urls: Vec<String>,
}

/// Represents different types of streaming manifests
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ManifestType {
    Json(TrackManifest),
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
    #[serde(skip_serializing_if = "Option::is_none", default, rename = "startNumber")]
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
