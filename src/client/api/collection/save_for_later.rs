use crate::{
    TidalClient, TidalError,
    client::models::{
        IntoParamMap, ResourceType,
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
            "/userCollectionSaveForLaters/me/relationships/items",
        )
        ._with_params(config.into_param_map())
        .with_country_code()
        .with_base_url(OPEN_API_V2_LOCATION)
        .send()
        .await
    }

    /// Adds an item to the user's "Save for Later" collection.
    ///
    /// id: The ID of the item to add (track, album, playlist, etc.)
    /// resource_type: The type of the resource (track, album, playlist, etc.)
    ///
    /// Returns a `GetSaveForLaterResponse` containing the added item in the "Save for
    /// Later" collection.
    pub async fn add_save_for_later_items(
        &self,
        id: impl Into<String>,
        resource_type: ResourceType,
    ) -> Result<GetSaveForLaterResponse, TidalError> {
        let body = serde_json::json!({
            "data": [
                {
                    "id": id.into(),
                    "type": resource_type.to_string()
                }
            ]
        });

        self.request(
            reqwest::Method::POST,
            "/userCollectionSaveForLaters/me/relationships/items",
        )
        .with_body(body)
        .with_country_code()
        .with_base_url(OPEN_API_V2_LOCATION)
        .send()
        .await
    }
}
