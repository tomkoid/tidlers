use crate::{
    client::{
        TidalClient,
        models::search::{
            SearchResultsResponse, SearchSuggestionsResponse,
            config::{SearchConfig, SearchSuggestionsConfig},
        },
    },
    error::TidalError,
    urls::WEB_API_V2_LOCATION,
};

impl TidalClient {
    /// Searches for content on Tidal with configurable options
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::TidalClient;
    /// # use tidlers::auth::TidalAuth;
    /// # use tidlers::client::models::search::config::{SearchConfig, SearchType};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let config = SearchConfig {
    ///     query: "Daft Punk".to_string(),
    ///     types: vec![SearchType::Artists, SearchType::Tracks],
    ///     limit: 10,
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
    pub async fn search(&self, config: SearchConfig) -> Result<SearchResultsResponse, TidalError> {
        let types_string = config
            .types
            .iter()
            .map(|t| t.to_api_params())
            .collect::<Vec<&str>>()
            .join(",");

        if config.limit > 300 {
            return Err(TidalError::InvalidArgument(
                "search limit cannot be greater than 300".to_string(),
            ));
        }

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
            .with_base_url(WEB_API_V2_LOCATION)
            .send()
            .await
    }

    /// Gets search suggestions for a query
    pub async fn search_suggestion(
        &self,
        config: SearchSuggestionsConfig,
    ) -> Result<SearchSuggestionsResponse, TidalError> {
        self.request(reqwest::Method::GET, "/suggestions")
            .with_country_code()
            .with_param("query", config.query)
            .with_param("explicit", config.explicit.to_string())
            .with_param("hybrid", config.hybrid.to_string())
            .with_base_url(WEB_API_V2_LOCATION)
            .send()
            .await
    }
}
