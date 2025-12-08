use crate::{
    client::{
        TidalClient,
        models::user::{User, UserData},
    },
    error::TidalError,
    responses::TidalGenericResponse,
};

impl TidalClient {
    pub async fn refresh_user_info(&mut self) -> Result<(), TidalError> {
        let json: TidalGenericResponse<UserData> = self
            .request(reqwest::Method::GET, "/users/me")
            .with_base_url(Self::OPEN_API_V2_LOCATION)
            .send()
            .await?;

        self.user_info = Some(User {
            user_id: json.data.id.parse()?,
            username: json.data.attributes.username,
            email: json.data.attributes.email,
            country_code: json.data.attributes.country,
            email_verified: json.data.attributes.email_verified,

            ..self.user_info.clone().unwrap()
        });

        Ok(())
    }
}
