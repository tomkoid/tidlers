#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSaveForLaterItemsResponse {
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}
