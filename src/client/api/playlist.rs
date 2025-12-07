use std::collections::HashMap;

use crate::{
    client::{
        TidalClient,
        models::playlist::{PlaylistInfo, PlaylistItemsResponse},
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
