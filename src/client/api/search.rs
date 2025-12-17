use crate::{
    client::{
        TidalClient,
        models::search::{SearchResultsFull, SearchSuggestionsFull, config::SearchConfig},
    },
    error::TidalError,
};

impl TidalClient {
    pub async fn search(&self, config: SearchConfig) -> Result<SearchResultsFull, TidalError> {
        let types_string = config
            .types
            .iter()
            .map(|t| t.to_api_params())
            .collect::<Vec<&str>>()
            .join(",");

        self.request(reqwest::Method::GET, "/search")
            .with_country_code()
            .with_locale()
            .with_param("query", config.query)
            .with_param(
                "includeContributors",
                config.include_contributors.to_string(),
            )
            .with_param("includeDidYouMean", config.include_did_you_mean.to_string())
            .with_param(
                "includeUserPlaylists",
                config.include_user_playlists.to_string(),
            )
            .with_param(
                "includeUserPlaylists",
                config.include_user_playlists.to_string(),
            )
            .with_param("supportsUserData", config.supports_user_data.to_string())
            .with_param("types", types_string)
            .with_param("limit", config.limit.to_string())
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
