use serde::{Deserialize, Serialize};

use crate::{
    client::tidal::TidalClient, error::TidalError, requests::TidalRequest,
    responses::TidalGenericResponse,
};

impl TidalClient {
    pub async fn get_arrival_mixes(
        &mut self,
    ) -> Result<TidalGenericResponse<Vec<ArrivalMixData>>, TidalError> {
        let url = format!(
            "/userRecommendations/{}/relationships/newArrivalMixes",
            self.user_info.as_ref().unwrap().user_id
        );

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        req.access_token = self.session.auth.access_token.clone();
        req.base_url = Some(Self::OPEN_API_V2_LOCATION.to_string());
        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: TidalGenericResponse<Vec<ArrivalMixData>> = serde_json::from_str(&body)?;

        Ok(json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixData {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}
