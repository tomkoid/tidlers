use crate::client::models::video::config::VideoPlaybackInfoConfig;
use crate::client::models::video::playback::EmuVideoManifest;
use crate::client::models::video::playback::VideoPlaybackInfoResponse;

use base64::{Engine, engine::general_purpose};

use crate::{
    TidalClient, TidalError,
    client::models::{playback::AssetPresentation, video::Video},
    ids::VideoId,
};

impl TidalClient {
    /// Retrieves video information by video ID
    pub async fn get_video(&self, video_id: impl Into<VideoId>) -> Result<Video, TidalError> {
        let video_id = video_id.into();
        self.request(reqwest::Method::GET, format!("/videos/{}/", video_id))
            .with_country_code()
            .send()
            .await
    }

    /// Gets video playback information including streaming URLs and manifest
    // TODO: implement opts for track and video (quality and playback mode), if not found, set to
    // default
    pub async fn get_video_postpaywall_playback_info(
        &self,
        video_id: impl Into<VideoId>,
        config: Option<VideoPlaybackInfoConfig>,
    ) -> Result<VideoPlaybackInfoResponse, TidalError> {
        let video_id = video_id.into();

        let config = config.unwrap_or_default();

        let video_quality = config
            .video_quality
            .unwrap_or(self.session.video_quality.clone());
        let playback_mode = config
            .playback_mode
            .unwrap_or(self.session.playback_mode.clone());
        let asset_presentation = config.asset_presentation.unwrap_or(AssetPresentation::Full);

        let body: String = self
            .request(
                reqwest::Method::GET,
                format!("/videos/{}/playbackinfopostpaywall", video_id),
            )
            .with_country_code()
            .with_param("videoquality", video_quality.to_string())
            .with_param("playbackmode", playback_mode.to_string())
            .with_param("assetpresentation", asset_presentation.to_string())
            .send_raw()
            .await?;

        let parsed = serde_json::from_str::<serde_json::Value>(&body)?;

        let manifest_decoded =
            general_purpose::STANDARD.decode(parsed["manifest"].as_str().unwrap())?;
        let manifest_decoded_str = String::from_utf8(manifest_decoded)?;

        let mut response: VideoPlaybackInfoResponse = serde_json::from_str(&body)?;

        let manifest: EmuVideoManifest = serde_json::from_str(&manifest_decoded_str)?;
        response.manifest = Some(manifest);

        Ok(response)
    }
}
