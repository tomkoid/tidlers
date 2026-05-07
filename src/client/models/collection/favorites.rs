use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::client::models::collection::artist::CollectionArtistEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FavoriteResourceType {
    Tracks,
    Albums,
    Artists,
}

impl Display for FavoriteResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FavoriteResourceType::Tracks => "tracks",
            FavoriteResourceType::Albums => "albums",
            FavoriteResourceType::Artists => "artists",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoritesResponse {
    #[serde(rename = "lastModifiedAt")]
    pub items: Vec<CollectionArtistEntry>,
}
