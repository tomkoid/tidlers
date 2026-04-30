use serde::{Deserialize, Serialize};

use crate::client::models::collection::artist::CollectionArtistMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoritesResponse {
    #[serde(rename = "lastModifiedAt")]
    pub items: Vec<CollectionArtistMetadata>,
}
