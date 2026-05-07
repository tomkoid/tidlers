use base64::{Engine, engine::general_purpose};
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::{
    client::{
        TidalClient,
        models::{
            mixes::TrackMixResponse,
            playback::AssetPresentation,
            track::{
                DashManifest, JsonTrackManifest, ParsedTrackManifest, Track,
                TrackPlaybackInfoResponse, TrackRadioResponse,
            },
        },
    },
    error::TidalError,
    ids::TrackId,
    urls::OPEN_API_V2_LOCATION,
};

use crate::client::models::track::config::UserUploadsIncludeOptions;
use crate::client::models::track::user_uploads::UserUploadsResponse;

impl TidalClient {
    /// Retrieves track information by track ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
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
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> Result<TrackPlaybackInfoResponse, TidalError> {
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

        let mut response: TrackPlaybackInfoResponse =
            serde_json::from_str::<TrackPlaybackInfoResponse>(&body)?;

        // Try to parse as JSON first (for LOW, HIGH, LOSSLESS)
        if let Ok(json_manifest) = serde_json::from_str::<JsonTrackManifest>(&manifest_decoded_str)
        {
            response.manifest = Some(json_manifest.clone());
            response.manifest_parsed = Some(ParsedTrackManifest::Json(json_manifest));
        } else {
            // If JSON parsing fails, try DASH XML (for HiRes)
            match Self::parse_dash_manifest(&manifest_decoded_str) {
                Ok(dash_manifest) => {
                    response.manifest_parsed = Some(ParsedTrackManifest::Dash(dash_manifest));
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
    ) -> Result<TrackMixResponse, TidalError> {
        let track_id = track_id.into();
        self.request(reqwest::Method::GET, format!("/tracks/{}/mix", track_id))
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_track_radio(
        &self,
        track_id: impl Into<TrackId>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<TrackRadioResponse, TidalError> {
        let track_id = track_id.into();
        self.request(reqwest::Method::GET, format!("/tracks/{}/radio", track_id))
            .with_param("limit", limit.unwrap_or(100).to_string())
            .with_param("offset", offset.unwrap_or(0).to_string())
            .with_country_code()
            .send()
            .await
    }

    pub async fn get_user_uploads(
        &self,
        include: UserUploadsIncludeOptions,
        next_cursor: Option<String>,
    ) -> Result<UserUploadsResponse, TidalError> {
        if self.session.auth.user_id.is_none() {
            return Err(TidalError::NotAuthenticated);
        }

        let includes = include.to_api_params();

        let user_id = if let Some(id) = self.session.auth.user_id {
            id
        } else {
            return Err(TidalError::NotAuthenticated);
        };

        self.request(reqwest::Method::GET, "/tracks")
            .with_country_code()
            .with_param("filter[owners.id]", user_id.to_string())
            // .with_param("limit", 1.to_string())
            .with_param("include", includes)
            .with_optional_param("page_cursor", next_cursor)
            .with_base_url(OPEN_API_V2_LOCATION)
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::TidalClient;

    #[test]
    fn parse_dash_manifest_extracts_expected_fields() {
        let xml = r#"
            <MPD>
              <Period>
                <AdaptationSet mimeType="audio/mp4">
                  <Representation codecs="flac" bandwidth="9216000">
                    <BaseURL>https://audio.example.com/</BaseURL>
                    <SegmentTemplate
                      initialization="init.mp4"
                      media="chunk-$Number$.m4s"
                      timescale="48000"
                      duration="96000"
                      startNumber="1"
                    />
                  </Representation>
                </AdaptationSet>
              </Period>
            </MPD>
        "#;

        let parsed = TidalClient::parse_dash_manifest(xml).expect("manifest should parse");
        assert_eq!(parsed.mime_type, "audio/mp4");
        assert_eq!(parsed.codecs, "flac");
        assert_eq!(parsed.bitrate, Some(9_216_000));
        assert_eq!(parsed.initialization_url.as_deref(), Some("init.mp4"));
        assert_eq!(
            parsed.media_url_template.as_deref(),
            Some("chunk-$Number$.m4s")
        );
        assert_eq!(parsed.timescale, Some(48_000));
        assert_eq!(parsed.duration, Some(96_000));
        assert_eq!(parsed.start_number, Some(1));
        assert!(
            parsed
                .urls
                .contains(&"https://audio.example.com/".to_string())
        );
        assert!(parsed.urls.contains(&"init.mp4".to_string()));
        assert!(parsed.urls.contains(&"chunk-$Number$.m4s".to_string()));
    }

    #[test]
    fn parse_dash_manifest_errors_when_no_urls_found() {
        let xml = r#"
            <MPD>
              <Period>
                <AdaptationSet mimeType="audio/mp4">
                  <Representation codecs="flac" bandwidth="9216000" />
                </AdaptationSet>
              </Period>
            </MPD>
        "#;

        let result = TidalClient::parse_dash_manifest(xml);
        assert!(result.is_err());
    }
}
