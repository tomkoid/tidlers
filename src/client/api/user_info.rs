use crate::{
    client::{
        TidalClient,
        models::user::{User, UserData},
    },
    error::TidalError,
    responses::TidalGenericResponse,
    urls::OPEN_API_V2_LOCATION,
};

impl TidalClient {
    /// Returns the current user's information, optionally forcing a refresh from the API
    pub async fn get_user_info(&mut self) -> Result<UserData, TidalError> {
        let json: TidalGenericResponse<UserData> = self
            .request(reqwest::Method::GET, "/users/me")
            .with_base_url(OPEN_API_V2_LOCATION)
            .send()
            .await?;

        Ok(json.data)
    }

    /// Refreshes the internally stored user information by fetching it from the API, this is needed
    /// after login or if you want to make sure you have the latest info that is then used for some
    /// other functionality
    pub async fn refresh_user_info(&mut self) -> Result<(), TidalError> {
        let ui = self.get_user_info().await?;

        self.user_info = Some(User {
            user_id: ui.id.parse()?,
            username: ui.attributes.username,
            email: ui.attributes.email,
            country_code: ui.attributes.country,
            email_verified: ui.attributes.email_verified,

            ..self.user_info.clone().unwrap()
        });

        Ok(())
    }
}
