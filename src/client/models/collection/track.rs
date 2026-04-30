use serde::{Deserialize, Serialize};

use crate::client::models::track::Track;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionTracksResponse {
    pub items: Vec<CollectionTrackItem>,
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionTrackItem {
    pub created: String,
    pub item: Track,
}
