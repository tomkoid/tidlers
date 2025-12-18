pub enum SearchType {
    Artists,
    Albums,
    Tracks,
    Videos,
    Playlists,
    Uploads,
}

pub struct SearchConfig {
    pub query: String,
    pub include_contributors: bool,
    pub include_did_you_mean: bool,
    pub include_user_playlists: bool,
    pub supports_user_data: bool,
    pub types: Vec<SearchType>,
    pub limit: u32,
    pub offset: u32,
}

pub struct SearchSuggestionsConfig {
    pub query: String,
    pub explicit: bool,
    pub hybrid: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            query: String::new(),
            include_contributors: true,
            include_did_you_mean: true,
            include_user_playlists: true,
            supports_user_data: true,
            types: vec![
                SearchType::Artists,
                SearchType::Albums,
                SearchType::Tracks,
                SearchType::Uploads,
                SearchType::Videos,
                SearchType::Playlists,
            ],
            limit: 20,
            offset: 0,
        }
    }
}

impl Default for SearchSuggestionsConfig {
    fn default() -> Self {
        SearchSuggestionsConfig {
            query: String::new(),
            explicit: true,
            hybrid: true,
        }
    }
}

impl SearchType {
    pub fn to_api_params(&self) -> &str {
        match self {
            SearchType::Artists => "ARTISTS",
            SearchType::Albums => "ALBUMS",
            SearchType::Tracks => "TRACKS",
            SearchType::Videos => "VIDEOS",
            SearchType::Playlists => "PLAYLISTS",
            SearchType::Uploads => "UPLOADS",
        }
    }
}
