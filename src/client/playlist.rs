use std::collections::HashMap;

use crate::{client::tidal::TidalClient, error::TidalError, requests::TidalRequest};

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
