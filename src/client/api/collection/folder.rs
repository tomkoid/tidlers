use crate::{
    client::{
        TidalClient,
        models::collection::folder::{
            FolderCollectionItem, FoldersFlattenedResponse, Order, OrderDirection,
        },
    },
    error::TidalError,
    urls::API_V2_LOCATION,
};

impl TidalClient {
    pub async fn create_folder(
        &self,
        name: impl Into<String>,
        parent_id: Option<String>,
    ) -> Result<FolderCollectionItem, TidalError> {
        self.request(
            reqwest::Method::PUT,
            "/my-collection/playlists/folders/create-folder",
        )
        .with_country_code()
        .with_param("name", name.into())
        .with_param("folderId", parent_id.unwrap_or("root".to_string()))
        .with_base_url(API_V2_LOCATION)
        .send()
        .await
    }

    pub async fn flattened_folders(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        order: Option<Order>,
        order_direction: Option<OrderDirection>,
    ) -> Result<FoldersFlattenedResponse, TidalError> {
        let order = match order {
            Some(Order::Date) => "DATE",
            None => "DATE",
        };

        let order_direction = match order_direction {
            Some(OrderDirection::Ascending) => "ASC",
            Some(OrderDirection::Descending) => "DESC",
            None => "DESC",
        };

        self.request(
            reqwest::Method::GET,
            "/my-collection/playlists/folders/flattened",
        )
        .with_country_code()
        .with_param("includeOnly", "FOLDER".to_string())
        .with_param("limit", limit.unwrap_or(50).to_string())
        .with_param("offset", offset.unwrap_or(0).to_string())
        .with_param("order", order)
        .with_param("orderDirection", order_direction)
        .with_base_url(API_V2_LOCATION)
        .send()
        .await
    }
}
