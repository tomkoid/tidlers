use base64::{Engine, engine::general_purpose};

#[derive(Debug, Clone)]
pub struct TidalAuth {
    pub client_id: String,
    pub client_secret: String,

    pub access_token: Option<String>,
    pub user_id: Option<u64>,

    api_token_auth: bool,
}

/// retrieves the client credentials by decoding a base64 encoded string
fn get_client_credentials() -> (String, String) {
    let encoded = String::from(
        "ZlgySnhkbW50WldLMGl4VDsxTm45QWZEQWp4cmdKRkpiS05XTGVBeUtHVkdtSU51WFBQTEhWWEF2eEFnPQ==",
    );

    let decoded_bytes = general_purpose::STANDARD
        .decode(encoded)
        .expect("Failed to decode Base64");

    let (client_id, client_secret) = String::from_utf8(decoded_bytes)
        .expect("failed to convert bytes to String in get client credentials")
        .split_once(';')
        .map(|(id, secret)| (id.to_string(), secret.to_string()))
        .unwrap();

    (client_id, client_secret)
}

impl TidalAuth {
    pub fn new() -> Self {
        Self {
            access_token: None,
            api_token_auth: false,
            ..Default::default()
        }
    }

    pub fn with_access_token(access_token: String) -> Self {
        Self {
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
        let c_creds = get_client_credentials();
        Self {
            client_id: c_creds.0,
            client_secret: c_creds.1,
            access_token: None,
            user_id: None,
            api_token_auth: false,
        }
    }
}
