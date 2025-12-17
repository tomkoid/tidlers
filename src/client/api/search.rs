use crate::{
    client::{
        TidalClient,
        models::search::{SearchResultsFull, SearchSuggestionsFull},
    },
    error::TidalError,
};

impl TidalClient {
    pub async fn search(&self, query: impl Into<String>) -> Result<SearchResultsFull, TidalError> {
        self.request(reqwest::Method::GET, "/search")
            .with_country_code()
            .with_locale()
            .with_param("query", query.into())
            .with_param("includeContributors", "true")
            .with_param("includeDidYouMean", "true")
            .with_param("includeUserPlaylists", "true")
            .with_param("includeUserPlaylists", "true")
            .with_param("supportsUserData", "true")
            .with_param("types", "ARTISTS,ALBUMS,TRACKS,VIDEOS,PLAYLISTS,UPLOADS")
            .with_param("limit", "20")
            .with_base_url(TidalClient::WEB_API_V2_LOCATION)
            .send()
            .await
    }

    pub async fn search_suggestion(
        &self,
        query: impl Into<String>,
    ) -> Result<SearchSuggestionsFull, TidalError> {
        self.request(reqwest::Method::GET, "/suggestions")
            .with_country_code()
            .with_param("query", query.into())
            .with_param("explicit", "true")
            .with_param("hybrid", "true")
            .with_param("limit", "20")
            .with_base_url(TidalClient::WEB_API_V2_LOCATION)
            .send()
            .await
    }
}
