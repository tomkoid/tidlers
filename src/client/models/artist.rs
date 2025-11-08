#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: String,
    pub picture: Option<String>,
}
