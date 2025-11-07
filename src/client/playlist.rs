use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    client::{
        tidal::TidalClient,
        track::{Artist, MediaMetadata},
    },
    error::TidalError,
    requests::TidalRequest,
};

impl TidalClient {
    pub async fn get_playlist(
        &mut self,
        playlist_uuid: String,
    ) -> Result<PlaylistInfo, TidalError> {
        let url = format!("/playlists/{}/", playlist_uuid);

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
    pub async fn get_playlist_items(
        &mut self,
        playlist_uuid: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<PlaylistItemsResponse, TidalError> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        if limit > 100 {
            return Err(TidalError::InvalidArgument(
                "limit cannot be greater than 100".to_string(),
            ));
        }

        let url = format!("/playlists/{}/items", playlist_uuid);

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert("limit".to_string(), limit.to_string());
        params.insert("offset".to_string(), offset.to_string());
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsResponse {
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: String,
    pub cut: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub duration: i32,
    pub replay_gain: f64,
    pub peak: f64,
    pub allow_streaming: bool,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub premium_streaming_only: bool,
    pub track_number: i32,
    pub volume_number: i32,
    pub version: Option<String>,
    pub popularity: i32,
    pub copyright: String,
    pub bpm: Option<i32>,
    pub description: Option<String>,
    pub url: String,
    pub isrc: String,
    pub editable: bool,
    pub explicit: bool,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub media_metadata: MediaMetadata,
    pub upload: bool,
    pub access_type: String,
    pub spotlighted: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub mixes: Option<Mixes>,
    pub date_added: String,
    pub index: i64,
    pub item_uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mixes {
    pub track_mix: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistInfo {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: u64,
    #[serde(rename = "numberOfVideos")]
    pub number_of_videos: u64,
    pub creator: PlaylistCreator,
    pub description: String,
    pub duration: u64,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub created: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    #[serde(rename = "publicPlaylist")]
    pub public_playlist: bool,
    pub url: String,
    pub image: String,
    pub popularity: u64,
    #[serde(rename = "squareImage")]
    pub square_image: String,
    #[serde(rename = "customImageUrl")]
    pub custom_image_url: Option<String>,
    #[serde(rename = "promotedArtists")]
    pub promoted_artists: Vec<String>,
    #[serde(rename = "lastItemAddedAt")]
    pub last_item_added_at: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistCreator {
    pub id: u64,
}
