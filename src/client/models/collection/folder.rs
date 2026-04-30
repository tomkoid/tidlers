use serde::{Deserialize, Serialize};

pub enum Order {
    Date,
}

pub enum OrderDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoldersFlattenedResponse {
    pub items: Vec<FolderCollectionItem>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderCollectionItem {
    pub added_at: String,
    pub data: FolderData,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub trn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderData {
    pub created_at: String,
    pub id: String,
    pub item_type: String,
    pub last_modified_at: String,
    pub name: String,
    pub total_number_of_items: i64,
    pub trn: String,
}
