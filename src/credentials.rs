#[derive(Debug, Clone, Default)]
pub struct TidalCredentials {
    pub client_id: String,
    pub client_secret: String,

    token_auth: bool,
}

impl TidalCredentials {
    pub fn new() -> Self {
        Self {
            client_id: "zU4XHVVkc2tDPo4t".to_string(),
            client_secret: "VJKhDFqJPqvsPVNBV6ukXTJmwlvbttP7wlMlrc72se4=".to_string(),
            token_auth: false,
        }
    }

    pub fn with_token(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            token_auth: true,
        }
    }

    pub fn is_token_auth(&self) -> bool {
        self.token_auth
    }
}
