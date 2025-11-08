use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackMixInfo {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mixes {
    pub track_mix: String,
}
