#[derive(Debug, Clone)]
pub struct TidalAuth {
    pub client_id: String,
    pub client_secret: String,

    pub access_token: Option<String>,
    pub user_id: Option<u64>,

    api_token_auth: bool,
}

impl TidalAuth {
    const CLIENT_ID: &str = "zU4XHVVkc2tDPo4t";
    const CLIENT_SECRET: &str = "VJKhDFqJPqvsPVNBV6ukXTJmwlvbttP7wlMlrc72se4=";

    pub fn new() -> Self {
        Self {
            client_id: Self::CLIENT_ID.to_string(),
            client_secret: Self::CLIENT_SECRET.to_string(),
            access_token: None,
            api_token_auth: false,
            ..Default::default()
        }
    }

    pub fn with_access_token(access_token: String) -> Self {
        Self {
            client_id: Self::CLIENT_ID.to_string(),
            client_secret: Self::CLIENT_SECRET.to_string(),
            access_token: Some(access_token),
            api_token_auth: false,
            ..Default::default()
        }
    }

    pub fn with_api_token(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token: None,
            api_token_auth: true,
            ..Default::default()
        }
    }

    pub fn is_token_auth(&self) -> bool {
        self.api_token_auth
    }
}

impl Default for TidalAuth {
    fn default() -> Self {
        Self {
            client_id: Self::CLIENT_ID.to_string(),
            client_secret: Self::CLIENT_SECRET.to_string(),
            access_token: None,
            user_id: None,
            api_token_auth: false,
        }
    }
}
