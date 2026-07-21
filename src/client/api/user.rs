use crate::{
    TidalClient, TidalError,
    client::models::user::{User, UserProfileResource, UserV1Response, UserV2Response},
    requests::TidalRequest,
    responses::ApiDataResponse,
    urls::{API_V2_LOCATION, OPEN_API_V2_LOCATION},
};

impl TidalClient {
    /// Gets information about a user by their user ID, this is not the same as the current logged
    /// in user, for that use `get_user_info`
    /// This only returns the first and last name of the user
    pub async fn get_user_v1(&self, user_id: String) -> Result<UserV1Response, TidalError> {
        self.request(reqwest::Method::GET, format!("/users/{user_id}"))
            .with_country_code()
            .send()
            .await
    }

    /// Gets information about a user by their user ID, this is not the same as the current logged
    /// in user, for that use `get_user_info`
    pub async fn get_user_v2(&self, user_id: String) -> Result<UserV2Response, TidalError> {
        self.request(reqwest::Method::GET, format!("/profiles/{user_id}"))
            .with_base_url(API_V2_LOCATION)
            .send()
            .await
    }

    /// Returns the current user's information, optionally forcing a refresh from the API
    pub async fn get_user_info(&mut self) -> Result<UserProfileResource, TidalError> {
        let json: ApiDataResponse<UserProfileResource> = self
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

    /// Logs the current user out of Tidal.
    pub async fn logout(&self) -> Result<(), TidalError> {
        let url = "/logout".to_string();

        let mut req = TidalRequest::new(reqwest::Method::POST, url.clone());
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let status = resp.status();

        if status != reqwest::StatusCode::NO_CONTENT {
            return Err(TidalError::Logout(status.as_str().to_string()));
        }

        Ok(())
    }
}
