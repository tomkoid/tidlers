use serde::{Deserialize, Serialize};

pub mod album;
pub mod artist;
pub mod favorites;
pub mod folder;
pub mod playlist;
pub mod track;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionCreator {
    pub id: i64,
    pub name: Option<String>,
    pub picture: Option<String>,
    #[serde(rename = "type")]
    pub creator_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SharingLevel {
    Public,
    Private,
}
