use crate::{
    TidalClient, TidalError,
    client::models::{
        IntoParamMap, ResourceType,
        collection::save_for_later::{
            config::GetSaveForLaterItemsConfig, list::GetSaveForLaterResponse,
            remove::RemoveSaveForLaterItemsResponse,
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

    /// Adds items to the user's "Save for Later" collection.
    ///
    /// `items` is a vector of tuples where each tuple contains the ID of the item and its
    /// corresponding `ResourceType`.
    ///
    /// Returns a `GetSaveForLaterResponse` containing the added item in the "Save for
    /// Later" collection.
    pub async fn add_save_for_later_items(
        &self,
        items: Vec<(String, ResourceType)>,
    ) -> Result<GetSaveForLaterResponse, TidalError> {
        let mut body = serde_json::json!({
            "data": []
        });

        for (id, resource_type) in items {
            body["data"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({
                    "id": id,
                    "type": resource_type.to_string()
                }));
        }

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

    /// Removes items to the user's "Save for Later" collection.
    ///
    /// `items` is a vector of tuples where each tuple contains the ID of the item and its
    /// corresponding `ResourceType`.
    ///
    /// Returns a `GetSaveForLaterResponse` containing the added item in the "Save for
    /// Later" collection.
    pub async fn remove_save_for_later_items(
        &self,
        items: Vec<(String, ResourceType)>,
    ) -> Result<RemoveSaveForLaterItemsResponse, TidalError> {
        let mut body = serde_json::json!({
            "data": []
        });

        for (id, resource_type) in items {
            body["data"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({
                    "id": id,
                    "type": resource_type.to_string()
                }));
        }

        self.request(
            reqwest::Method::DELETE,
            "/userCollectionSaveForLaters/me/relationships/items",
        )
        .with_body(body)
        .with_country_code()
        .with_base_url(OPEN_API_V2_LOCATION)
        .send()
        .await
    }
}
