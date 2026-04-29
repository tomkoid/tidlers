use base64::{Engine, engine::general_purpose};

use crate::auth::TidalAuth;

impl TidalAuth {
    /// Sets the client ID for API token authentication
    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    /// Sets the client secret for API token authentication
    pub fn set_client_secret(&mut self, client_secret: String) {
        self.client_secret = client_secret;
    }
}

/// Retrieves the default client credentials by decoding a base64 encoded string
pub(crate) fn get_default_client_credentials() -> (String, String) {
    let encoded = String::from(
        "NE4zbjZRMXg5NUxMNUs3cDtvS09YZkpXMzcxY1g2eGFaMFB5aGdHTkJkTkxsQlpkNEFLS1lvdWdNamlrPQ==",
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
