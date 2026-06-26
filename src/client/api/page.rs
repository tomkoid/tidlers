use crate::{
    client::{TidalClient, models::page::PageResponse},
    error::TidalError,
};

impl TidalClient {
    /// Retrieves a page by its slug
    pub async fn get_page(&self, slug: impl Into<String>) -> Result<PageResponse, TidalError> {
        self.request(reqwest::Method::GET, format!("/pages/{}", slug.into()))
            .with_country_code()
            .with_locale()
            .with_web_stuff()
            .send()
            .await
    }

    /// Retrieves the explore page
    pub async fn get_explore_page(&self) -> Result<PageResponse, TidalError> {
        self.get_page("explore").await
    }

    /// Removes the leading `/`, `v1/`, and `pages/` from a page slug, if present
    pub fn normalize_page_slug(&self, slug: impl Into<String>) -> String {
        slug.into()
            .trim_start_matches('/')
            .trim_start_matches("v1/")
            .trim_start_matches("pages/")
            .to_string()
    }
}
