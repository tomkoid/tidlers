use serde::{Deserialize, Serialize};

pub enum FolderOrder {
    Date,
}

/// Order direction for sorting folders (backwards compatibility, reusing OrderDirection)
pub type FolderOrderDirection = crate::client::models::OrderDirection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderListResponse {
    pub items: Vec<FolderCollectionEntry>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderCollectionEntry {
    pub added_at: String,
    pub data: FolderDetails,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub trn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderDetails {
    pub created_at: String,
    pub id: String,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub total_number_of_items: i64,
    pub trn: String,
}
