use serde::{Deserialize, Serialize};

use crate::{client::TidalClient, error::TidalError, responses::TidalGenericResponse};

impl TidalClient {
    pub async fn get_arrival_mixes(
        &mut self,
    ) -> Result<TidalGenericResponse<Vec<ArrivalMixData>>, TidalError> {
        let url = format!(
            "/userRecommendations/{}/relationships/newArrivalMixes",
            self.user_info.as_ref().unwrap().user_id
        );

        self.request(reqwest::Method::GET, url)
            .with_base_url(Self::OPEN_API_V2_LOCATION)
            .send()
            .await
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
