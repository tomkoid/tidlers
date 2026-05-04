use crate::{
    client::{
        TidalClient,
        models::{mixes::ArrivalMixResource, responses::ApiDataResponse},
    },
    error::TidalError,
    urls::OPEN_API_V2_LOCATION,
};

impl TidalClient {
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
