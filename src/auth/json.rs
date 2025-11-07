use crate::auth::init::TidalAuth;

impl TidalAuth {
    pub fn from_serialized(auth_json: &str) -> Result<TidalAuth, serde_json::Error> {
        let auth_json: TidalAuth = serde_json::from_str(auth_json)?;

        Ok(Self {
            client_id: auth_json.client_id,
            client_secret: auth_json.client_secret,
            access_token: auth_json.access_token,
            user_id: auth_json.user_id,
            api_token_auth: auth_json.api_token_auth,
            rq: auth_json.rq,
        })
    }

    pub fn get_auth_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
