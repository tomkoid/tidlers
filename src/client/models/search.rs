use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsResponse {
    pub data: SearchResultData,
    pub links: SearchResultLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultData {
    pub id: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub attributes: SearchResultAttributes,
    pub relationships: SearchResultRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultAttributes {
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultRelationships {
    pub albums: Option<SearchRelationship>,
    pub artists: Option<SearchRelationship>,
    pub playlists: Option<SearchRelationship>,
    #[serde(rename = "topHits")]
    pub top_hits: Option<SearchRelationship>,
    pub tracks: Option<SearchRelationship>,
    pub videos: Option<SearchRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRelationship {
    pub links: SearchRelationshipLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRelationshipLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTypeResultsResponse {
    pub data: Vec<SearchTypeItemRef>,
    pub links: SearchTypeLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTypeItemRef {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTypeLinks {
    #[serde(rename = "self")]
    pub self_link: String,
    pub next: Option<String>,
    pub meta: Option<SearchTypeMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTypeMeta {
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionsResponse {
    pub data: SearchSuggestionData,
    pub links: SearchResultLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionData {
    pub id: String,
    #[serde(rename = "type")]
    pub suggestion_type: String,
    pub attributes: SearchSuggestionAttributes,
    pub relationships: SearchSuggestionRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionAttributes {
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
    pub history: Vec<SearchSuggestionEntry>,
    pub suggestions: Vec<SearchSuggestionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionEntry {
    pub query: String,
    pub highlights: Vec<SearchHighlight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHighlight {
    pub start: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionRelationships {
    #[serde(rename = "directHits")]
    pub direct_hits: Option<SearchRelationship>,
}

#[derive(Clone, Debug)]
pub enum SearchType {
    Albums,
    Artists,
    Playlists,
    TopHits,
    Tracks,
    Videos,
}

impl FromStr for SearchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "albums" => Ok(SearchType::Albums),
            "artists" => Ok(SearchType::Artists),
            "playlists" => Ok(SearchType::Playlists),
            "topHits" | "tophits" | "top_hits" => Ok(SearchType::TopHits),
            "tracks" => Ok(SearchType::Tracks),
            "videos" => Ok(SearchType::Videos),
            _ => Err(format!("unknown search type: {s}")),
        }
    }
}

impl ToString for SearchType {
    fn to_string(&self) -> String {
        match self {
            SearchType::Albums => "albums",
            SearchType::Artists => "artists",
            SearchType::Playlists => "playlists",
            SearchType::TopHits => "topHits",
            SearchType::Tracks => "tracks",
            SearchType::Videos => "videos",
        }
        .to_string()
    }
}
