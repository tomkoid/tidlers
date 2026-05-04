use serde::{Deserialize, Serialize};

use crate::client::models::track::Track;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFavoriteTracksResponse {
    pub items: Vec<CollectionFavoriteTrackEntry>,
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoriteTrackEntry {
    pub created: String,
    pub item: Track,
}
