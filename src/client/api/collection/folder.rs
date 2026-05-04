use tracing::debug;

use crate::{
    client::{
        TidalClient,
        models::collection::folder::{
            FolderCollectionEntry, FolderListResponse, FolderOrder, FolderOrderDirection,
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
    ) -> Result<FolderCollectionEntry, TidalError> {
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

    pub async fn remove_folder(&self, id: impl Into<String>) -> Result<(), TidalError> {
        let res = self
            .request(
                reqwest::Method::PUT,
                "/my-collection/playlists/folders/remove",
            )
            .with_country_code()
            .with_param("trns", format!("trn:folder:{}", id.into()))
            .with_base_url(API_V2_LOCATION)
            .send_raw()
            .await?;

        debug!("remove folder response: {}", res);

        Ok(())
    }

    pub async fn flattened_folders(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        order: Option<FolderOrder>,
        order_direction: Option<FolderOrderDirection>,
    ) -> Result<FolderListResponse, TidalError> {
        let order = match order {
            Some(FolderOrder::Date) => "DATE",
            None => "DATE",
        };

        let order_direction = match order_direction {
            Some(FolderOrderDirection::Ascending) => "ASC",
            Some(FolderOrderDirection::Descending) => "DESC",
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
