use crate::{
    TidalClient, TidalError,
    client::models::{
        IntoParamMap,
        collection::save_for_later::{
            config::GetSaveForLaterItemsConfig, list_items::GetSaveForLaterResponse,
        },
    },
    urls::OPEN_API_V2_LOCATION,
};

impl TidalClient {
    pub async fn get_save_for_later_items(
        &self,
        config: GetSaveForLaterItemsConfig,
    ) -> Result<GetSaveForLaterResponse, TidalError> {
        self.request(
            reqwest::Method::GET,
            format!("/userCollectionSaveForLaters/me/relationships/items"),
        )
        ._with_params(config.into_param_map())
        .with_country_code()
        .with_base_url(OPEN_API_V2_LOCATION)
        .send()
        .await
    }
}
