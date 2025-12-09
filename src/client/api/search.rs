use crate::{
    client::{
        TidalClient,
        models::search::{SearchResultsResponse, SearchSuggestionsResponse},
    },
    error::TidalError,
};

impl TidalClient {
    pub async fn search(
        &self,
        query: impl Into<String>,
    ) -> Result<SearchResultsResponse, TidalError> {
        self.request(
            reqwest::Method::GET,
            format!("/searchResults/{}", query.into()),
        )
        .with_country_code()
        .with_base_url(TidalClient::OPEN_API_V2_LOCATION)
        .send()
        .await
    }

    pub async fn search_suggestion(
        &self,
        query: impl Into<String>,
    ) -> Result<SearchSuggestionsResponse, TidalError> {
        self.request(
            reqwest::Method::GET,
            format!("/searchSuggestions/{}", query.into()),
        )
        .with_country_code()
        .with_base_url(TidalClient::OPEN_API_V2_LOCATION)
        .send()
        .await
    }
}
