/// Response containing track playback information including manifest data
///
/// Every field but `track_id` defaults when TIDAL omits it: the descriptive
/// scalars vary by tier, account and track, and the manifest is what the
/// caller came for — losing the whole response over an absent `audioMode` or
/// replay gain serves nobody. A missing gain defaults to 0.0, which is the
/// no-adjustment value a client would apply anyway.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackPlaybackInfoResponse {
    pub track_id: u64,
    #[serde(default)]
    pub asset_presentation: String,
    #[serde(default)]
    pub audio_mode: String,
    #[serde(default)]
    pub audio_quality: String,
    #[serde(default)]
    pub manifest_mime_type: String,
    #[serde(default)]
    pub manifest_hash: String,
    #[serde(skip_deserializing, default)]
    pub manifest: Option<JsonTrackManifest>,
    #[serde(skip_deserializing, default)]
    pub manifest_parsed: Option<ParsedTrackManifest>,
    /// The manifest as TIDAL sent it, base64-decoded but otherwise untouched.
    ///
    /// For the FLAC tiers that manifest is a DASH MPD, and a player that hands
    /// it to a demuxer (GStreamer's `dashdemux` via a `data:application/dash+xml`
    /// URI, say) needs the XML verbatim — [`DashManifest`] describes the
    /// manifest but cannot reproduce it.
    #[serde(skip_deserializing, default)]
    pub manifest_raw: Option<String>,
    pub bit_depth: Option<u32>,
    pub sample_rate: Option<u32>,
    #[serde(default)]
    pub album_replay_gain: f64,
    #[serde(default)]
    pub album_peak_amplitude: f64,
    #[serde(default)]
    pub track_replay_gain: f64,
    #[serde(default)]
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
    /// let playback = client.get_track_postpaywall_playback_info("123456789", None).await?;
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
    /// # use tidlers::client::models::track::playback::DashManifest;
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
    use super::TrackPlaybackInfoResponse;

    #[test]
    fn parses_a_response_that_carries_only_the_essentials() {
        // Everything TIDAL can leave out, left out.
        let json = r#"{ "trackId": 123456789, "audioQuality": "HI_RES_LOSSLESS" }"#;

        let response: TrackPlaybackInfoResponse =
            serde_json::from_str(json).expect("response should parse");

        assert_eq!(response.track_id, 123_456_789);
        assert_eq!(response.audio_quality, "HI_RES_LOSSLESS");
        assert_eq!(response.audio_mode, "");
        assert_eq!(response.album_replay_gain, 0.0);
        assert_eq!(response.bit_depth, None);
    }
}
