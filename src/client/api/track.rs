use std::collections::HashMap;

use crate::{
    client::{
        models::{mixes::TrackMixInfo, track::Track},
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
