#[derive(Debug, Clone, Default)]
pub struct TidalCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,

    api_token_auth: bool,
}

impl TidalCredentials {
    const CLIENT_ID: &str = "zU4XHVVkc2tDPo4t";
    const CLIENT_SECRET: &str = "VJKhDFqJPqvsPVNBV6ukXTJmwlvbttP7wlMlrc72se4=";

    pub fn new() -> Self {
        Self {
            client_id: Self::CLIENT_ID.to_string(),
            client_secret: Self::CLIENT_SECRET.to_string(),
            access_token: None,
            api_token_auth: false,
        }
    }

    pub fn with_access_token(access_token: String) -> Self {
        Self {
            client_id: Self::CLIENT_ID.to_string(),
            client_secret: Self::CLIENT_SECRET.to_string(),
            access_token: Some(access_token),
            api_token_auth: false,
        }
    }

    pub fn with_api_token(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token: None,
            api_token_auth: true,
        }
    }

    pub fn is_token_auth(&self) -> bool {
        self.api_token_auth
    }
}
