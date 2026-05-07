use serde::{Deserialize, Serialize};

use crate::client::models::collection::artist::CollectionArtistEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FavoriteResourceType {
    Tracks,
    Albums,
}

impl ToString for FavoriteResourceType {
    fn to_string(&self) -> String {
        match self {
            FavoriteResourceType::Tracks => "tracks".to_string(),
            FavoriteResourceType::Albums => "albums".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoritesResponse {
    #[serde(rename = "lastModifiedAt")]
    pub items: Vec<CollectionArtistEntry>,
}
