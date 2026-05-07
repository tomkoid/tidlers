use crate::{
    client::{
        TidalClient,
        models::{mix::MixItemsResponse, mixes::ArrivalMixResource, responses::ApiDataResponse},
    },
    error::TidalError,
    urls::OPEN_API_V2_LOCATION,
};

impl TidalClient {
    /// Gets the tracks in a mix
    pub async fn get_mix_tracks(
        &self,
        mix_id: String,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<MixItemsResponse, TidalError> {
        self.request(reqwest::Method::GET, format!("/mixes/{mix_id}/items"))
            .with_country_code()
            .with_param("limit", limit.unwrap_or(100).to_string())
            .with_param("offset", offset.unwrap_or(0).to_string())
            .send()
            .await
    }

    pub async fn get_arrival_mixes(
        &self,
    ) -> Result<ApiDataResponse<Vec<ArrivalMixResource>>, TidalError> {
        let url = format!(
            "/userRecommendations/{}/relationships/newArrivalMixes",
            self.user_id()?
        );

        self.request(reqwest::Method::GET, url)
            .with_base_url(OPEN_API_V2_LOCATION)
            .send()
            .await
    }
}
