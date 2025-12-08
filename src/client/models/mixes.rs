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

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixData {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}
