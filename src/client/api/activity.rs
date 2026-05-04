use crate::{
    client::{
        TidalClient,
        models::activity::{ActivityTimelineResponse, TopArtistsMonthlyResponse},
    },
    error::TidalError,
    urls::API_V2_LOCATION,
};

impl TidalClient {
    pub async fn get_activity_timeline(&self) -> Result<ActivityTimelineResponse, TidalError> {
        self.request(reqwest::Method::GET, "/my-activity/timeline")
            .with_country_code()
            .with_locale()
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }

    pub async fn get_activity_top_artists(
        &self,
        year: i32,
        month: u32,
    ) -> Result<TopArtistsMonthlyResponse, TidalError> {
        self.request(reqwest::Method::GET, "/my-activity/top-artists")
            .with_country_code()
            .with_locale()
            .with_param("year", year.to_string())
            .with_param("month", month.to_string())
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }
}
