use serde::{Deserialize, Serialize};

use crate::client::models::album::Album;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFavoriteAlbumsResponse {
    pub items: Vec<CollectionFavoriteAlbumEntry>,
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoriteAlbumEntry {
    pub created: String,
    pub item: Album,
}
