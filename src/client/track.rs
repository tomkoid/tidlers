use std::collections::HashMap;

use crate::{client::tidal::TidalClient, error::TidalError, requests::TidalRequest};

impl TidalClient {
    pub async fn get_track(&mut self, track_id: String) -> Result<TrackInfo, TidalError> {
        let url = format!("/tracks/{}/", track_id);

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_track_mix(&mut self, track_id: String) -> Result<TrackMixInfo, TidalError> {
        let url = format!("/tracks/{}/mix", track_id);

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackInfo {
    pub id: u64,
    pub title: String,
    pub duration: u64,
    #[serde(rename = "replayGain")]
    pub replay_gain: f32,
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
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub mixes: HashMap<String, String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackMixInfo {
    pub id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MediaMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: String,
    pub picture: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub cover: String,
    #[serde(rename = "vibrantColor")]
    pub vibrant_color: String,
    #[serde(rename = "videoCover")]
    pub video_cover: Option<String>,
}
