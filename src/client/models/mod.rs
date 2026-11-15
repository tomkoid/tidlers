use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod activity;
pub mod album;
pub mod artist;
pub mod collection;
pub mod events;
pub mod feed;
pub mod home;
pub mod media;
pub mod mix;
pub mod page;
pub mod playback;
pub mod playlist;
pub mod responses;
pub mod search;
pub mod subscription;
pub mod track;
pub mod user;
pub mod video;

/// Collection models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Tracks,
    Albums,
    Artists,
    Playlists,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceType::Tracks => "tracks",
            ResourceType::Albums => "albums",
            ResourceType::Artists => "artists",
            ResourceType::Playlists => "playlists",
        };
        write!(f, "{}", s)
    }
}

// trait for converting a struct into a HashMap of parameters for API requests
pub trait IntoParamMap {
    fn into_param_map(self) -> std::collections::HashMap<String, String>;
}

// backwards compat
pub mod mixes {
    pub use super::mix::*;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistNameId {
    pub name: String,
    pub id: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderDirection::Ascending => write!(f, "ASC"),
            OrderDirection::Descending => write!(f, "DESC"),
        }
    }
}
