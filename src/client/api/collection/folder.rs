use crate::{
    client::{TidalClient, models::collection::FolderCollectionItem},
    error::TidalError,
    urls::API_V2_LOCATION,
};

impl TidalClient {
    pub async fn create_folder(
        &self,
        title: impl Into<String>,
        parent_id: Option<String>,
    ) -> Result<FolderCollectionItem, TidalError> {
        self.request(
            reqwest::Method::PUT,
            "/my-collection/playlists/folders/create-folder",
        )
        .with_country_code()
        .with_param("name", title.into())
        .with_param("folderId", parent_id.unwrap_or("root".to_string()))
        .with_base_url(API_V2_LOCATION)
        .send()
        .await
    }
}
