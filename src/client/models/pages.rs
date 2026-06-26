use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    pub self_link: Option<String>,
    pub id: String,
    pub title: String,
    pub rows: Vec<PageRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRow {
    pub modules: Vec<PageModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModule {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub title: String,
    pub description: String,
    pub layout: Option<String>,

    pub show_more: Option<PageModuleShowMore>,
    pub paged_list: PageModulePagedList,
    pub pre_title: Option<String>,
    pub list_format: Option<String>,

    /// The width of the module on official Tidal clients
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModuleShowMore {
    pub title: String,
    pub api_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModulePagedList {
    pub items: Vec<serde_json::Value>,
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub lines: Option<u32>,
    pub data_api_path: Option<String>,
}
