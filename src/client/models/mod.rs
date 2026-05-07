pub mod activity;
pub mod album;
pub mod artist;
pub mod collection;
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
