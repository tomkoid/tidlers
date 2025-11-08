#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MediaMetadata {
    pub tags: Vec<String>,
}
