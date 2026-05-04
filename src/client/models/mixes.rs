use serde::{Deserialize, Serialize};
use crate::client::models::responses::ApiLinks;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackMixResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mixes {
    pub track_mix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixResource {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

pub type ArrivalMixResourceLinks = ApiLinks;
