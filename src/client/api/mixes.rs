use crate::{
    client::{TidalClient, models::mixes::ArrivalMixData},
    error::TidalError,
    responses::TidalGenericResponse,
};

impl TidalClient {
    pub async fn get_arrival_mixes(
        &self,
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
