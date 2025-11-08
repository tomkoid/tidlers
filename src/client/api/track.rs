use std::collections::HashMap;

use base64::{Engine, engine::general_purpose};

use crate::{
    client::{
        models::{
            mixes::TrackMixInfo,
            playback::AssetPresentation,
            track::{Track, TrackManifest, TrackPlaybackInfoPostPaywallResponse},
        },
        tidal::TidalClient,
    },
    error::TidalError,
    requests::TidalRequest,
};

impl TidalClient {
    pub async fn get_track(&mut self, track_id: String) -> Result<Track, TidalError> {
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

    pub async fn get_track_postpaywall_playback_info(
        &mut self,
        track_id: String,
    ) -> Result<TrackPlaybackInfoPostPaywallResponse, TidalError> {
        let url = format!("/tracks/{}/playbackinfopostpaywall", track_id);

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        params.insert(
            "audioquality".to_string(),
            self.session.audio_quality.to_string(),
        );

        params.insert(
            "playbackmode".to_string(),
            self.session.playback_mode.to_string(),
        );

        params.insert(
            "assetpresentation".to_string(),
            AssetPresentation::Full.to_string(),
        );
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;
        let parsed = serde_json::from_str::<serde_json::Value>(&body)?;

        let manifest_decoded =
            general_purpose::STANDARD.decode(parsed["manifest"].as_str().unwrap())?;
        let manifest_decoded_str = String::from_utf8(manifest_decoded)?;

        let mut response: TrackPlaybackInfoPostPaywallResponse =
            serde_json::from_str::<TrackPlaybackInfoPostPaywallResponse>(&body)?;
        response.manifest = Some(serde_json::from_str::<TrackManifest>(
            &manifest_decoded_str,
        )?);

        Ok(response)
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
