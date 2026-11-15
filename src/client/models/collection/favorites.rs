use serde::{Deserialize, Serialize};

use crate::client::models::collection::artist::CollectionArtistEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoritesResponse {
    #[serde(rename = "lastModifiedAt")]
    pub items: Vec<CollectionArtistEntry>,
}
