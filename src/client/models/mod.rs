use std::fmt::Display;

pub mod activity;
pub mod album;
pub mod artist;
pub mod collection;
pub mod feed;
pub mod home;
pub mod media;
pub mod mix;
pub mod playback;
pub mod playlist;
pub mod responses;
pub mod search;
pub mod subscription;
pub mod track;
pub mod user;

// backwards compat
pub mod mixes {
    pub use super::mix::*;
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
