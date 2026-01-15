use crate::{
    client::{
        TidalClient,
        models::search::{
            SearchResultsFull, SearchSuggestionsFull,
            config::{SearchConfig, SearchSuggestionsConfig},
        },
    },
    error::TidalError,
};

impl TidalClient {
    /// Searches for content on Tidal with configurable options
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # use tidlers::client::models::search::config::{SearchConfig, SearchType};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let config = SearchConfig {
    ///     query: "Daft Punk".to_string(),
    ///     types: vec![SearchType::Artists, SearchType::Tracks],
    ///     limit: 10,
    ///     offset: 0,
    ///     ..Default::default()
    /// };
    ///
    /// let results = client.search(config).await?;
    /// if let Some(artists) = results.artists {
    ///     for artist in artists.items {
    ///         println!("Artist: {}", artist.name);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
            .with_param("offset", config.offset.to_string())
            .with_base_url(TidalClient::WEB_API_V2_LOCATION)
            .send()
            .await
    }

    /// Gets search suggestions for a query
    pub async fn search_suggestion(
        &self,
        config: SearchSuggestionsConfig,
    ) -> Result<SearchSuggestionsFull, TidalError> {
        self.request(reqwest::Method::GET, "/suggestions")
            .with_country_code()
            .with_param("query", config.query)
            .with_param("explicit", config.explicit.to_string())
            .with_param("hybrid", config.hybrid.to_string())
            .with_base_url(TidalClient::WEB_API_V2_LOCATION)
            .send()
            .await
    }
}
