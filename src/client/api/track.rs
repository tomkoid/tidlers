use base64::{Engine, engine::general_purpose};
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::{
    client::{
        TidalClient,
        models::{
            mixes::TrackMixInfo,
            playback::AssetPresentation,
            track::{
                DashManifest, ManifestType, Track, TrackManifest,
                TrackPlaybackInfoPostPaywallResponse,
            },
        },
    },
    error::TidalError,
    ids::TrackId,
};

impl TidalClient {
    /// Retrieves track information by track ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let track = client.get_track("123456789").await?;
    /// println!("Track: {} by {}", track.title, track.artist.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_track(&self, track_id: impl Into<TrackId>) -> Result<Track, TidalError> {
        let track_id = track_id.into();
        self.request(reqwest::Method::GET, format!("/tracks/{}/", track_id))
            .with_country_code()
            .send()
            .await
    }

    /// Parses a DASH XML manifest into a structured format
    fn parse_dash_manifest(xml: &str) -> Result<DashManifest, TidalError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut urls = Vec::new();
        let mut mime_type = String::new();
        let mut codecs = String::new();
        let mut bitrate = None;
        let mut buf = Vec::new();
        let mut init_url = None;
        let mut media_url = None;
        let mut timescale = None;
        let mut duration = None;
        let mut start_number = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(e)) | Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"AdaptationSet" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"mimeType" {
                                mime_type = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    b"Representation" => {
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"codecs" => {
                                    codecs = String::from_utf8_lossy(&attr.value).to_string();
                                }
                                b"bandwidth" => {
                                    bitrate =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>().ok();
                                }
                                _ => {}
                            }
                        }
                    }
                    b"SegmentTemplate" => {
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"initialization" => {
                                    init_url =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                                b"media" => {
                                    media_url =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                                b"timescale" => {
                                    timescale =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>().ok();
                                }
                                b"duration" => {
                                    duration =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>().ok();
                                }
                                b"startNumber" => {
                                    start_number =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>().ok();
                                }
                                _ => {}
                            }
                        }
                    }
                    b"BaseURL" => {
                        if let Ok(Event::Text(e)) = reader.read_event_into(&mut buf) {
                            let url = String::from_utf8_lossy(e.as_ref()).to_string();
                            if !url.is_empty() {
                                urls.push(url);
                            }
                        }
                    }
                    _ => {}
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(TidalError::Other(format!("XML parsing error: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        let initialization_url = init_url.clone();
        let media_url_template = media_url.clone();

        if let Some(init) = init_url {
            urls.push(init);
        }
        if let Some(media) = media_url {
            urls.push(media);
        }

        if urls.is_empty() {
            return Err(TidalError::Other(
                "No URLs found in DASH manifest".to_string(),
            ));
        }

        Ok(DashManifest {
            mime_type,
            codecs,
            urls,
            bitrate,
            initialization_url,
            media_url_template,
            timescale,
            duration,
            start_number,
        })
    }

    /// Gets track playback information including streaming URLs and manifest
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let playback = client.get_track_postpaywall_playback_info("123456789").await?;
    /// if let Some(url) = playback.get_primary_url() {
    ///     println!("Stream URL: {}", url);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_track_postpaywall_playback_info(
        &self,
        track_id: impl Into<TrackId>,
    ) -> Result<TrackPlaybackInfoPostPaywallResponse, TidalError> {
        let track_id = track_id.into();
        let audio_quality = self.session.audio_quality.to_string();
        let playback_mode = self.session.playback_mode.to_string();

        let body: String = self
            .request(
                reqwest::Method::GET,
                format!("/tracks/{}/playbackinfopostpaywall", track_id),
            )
            .with_country_code()
            .with_param("audioquality", audio_quality)
            .with_param("playbackmode", playback_mode)
            .with_param("assetpresentation", AssetPresentation::Full.to_string())
            .send_raw()
            .await?;

        let parsed = serde_json::from_str::<serde_json::Value>(&body)?;

        let manifest_decoded =
            general_purpose::STANDARD.decode(parsed["manifest"].as_str().unwrap())?;
        let manifest_decoded_str = String::from_utf8(manifest_decoded)?;

        let mut response: TrackPlaybackInfoPostPaywallResponse =
            serde_json::from_str::<TrackPlaybackInfoPostPaywallResponse>(&body)?;

        // Try to parse as JSON first (for LOW, HIGH, LOSSLESS)
        if let Ok(json_manifest) = serde_json::from_str::<TrackManifest>(&manifest_decoded_str) {
            response.manifest = Some(json_manifest.clone());
            response.manifest_parsed = Some(ManifestType::Json(json_manifest));
        } else {
            // If JSON parsing fails, try DASH XML (for HiRes)
            match Self::parse_dash_manifest(&manifest_decoded_str) {
                Ok(dash_manifest) => {
                    response.manifest_parsed = Some(ManifestType::Dash(dash_manifest));
                }
                Err(e) => {
                    return Err(TidalError::Other(format!(
                        "Failed to parse manifest as JSON or DASH: {}",
                        e
                    )));
                }
            }
        }

        Ok(response)
    }

    pub async fn get_track_mix(
        &self,
        track_id: impl Into<TrackId>,
    ) -> Result<TrackMixInfo, TidalError> {
        let track_id = track_id.into();
        self.request(reqwest::Method::GET, format!("/tracks/{}/mix", track_id))
            .with_country_code()
            .send()
            .await
    }
}
