/// Response containing resources added to saved for later collection
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSaveForLaterResponse {
    pub data: Vec<serde_json::Value>,
    #[serde(default)]
    pub included: Vec<SaveForLaterItemsIncludedResource>,
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsIncludedResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<SaveForLaterItemsIncludedResourceAttributes>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsIncludedResourceAttributes {
    #[serde(default)]
    pub access_type: Option<String>,
    pub bounded: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_links: Vec<SaveForLaterItemsExternalLink>,
    #[serde(default)]
    pub last_modified_at: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    #[serde(default)]
    pub number_of_items: Option<u32>,
    #[serde(default)]
    pub number_of_track_items: Option<u32>,
    #[serde(default)]
    pub number_of_video_items: Option<u32>,
    #[serde(default)]
    pub playlist_type: Option<String>,
}

/// Link information including URL and metadata
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsExternalLink {
    pub href: String,
    pub meta: SaveForLaterItemsExternalLinkMeta,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsExternalLinkMeta {
    #[serde(default, rename = "type")]
    pub link_type: String,
}
