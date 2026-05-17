use base64::{DecodeError, Engine, engine::general_purpose};
use rand::{Rng, RngExt};
use sha2::{Digest, Sha256};

use crate::{
    auth::{TidalAuth, pkce::PkceConfig},
    urls::PKCE_URI_REDIRECT,
};

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

fn decode_base64(encoded: impl AsRef<[u8]>) -> Result<String, DecodeError> {
    let decoded_bytes = general_purpose::STANDARD.decode(encoded)?;

    String::from_utf8(decoded_bytes)
        .map_err(|e| DecodeError::InvalidLength(e.utf8_error().valid_up_to()))
}

/// Retrieves the default OAuth2 client credentials by decoding a base64 encoded string
pub(crate) fn get_default_client_credentials() -> (String, String) {
    let encoded = String::from(
        "NE4zbjZRMXg5NUxMNUs3cDtvS09YZkpXMzcxY1g2eGFaMFB5aGdHTkJkTkxsQlpkNEFLS1lvdWdNamlrPQ==",
    );

    let decoded = decode_base64(encoded).expect("failed to decode default client credentials");
    let (client_id, client_secret) = decoded
        .split_once(';')
        .expect("failed to parse default client credentials");

    (client_id.to_string(), client_secret.to_string())
}

impl PkceConfig {
    pub fn try_default() -> Result<Self, DecodeError> {
        let mut rng = rand::rng();

        let client_id = decode_base64(format!(
            "{}{}",
            decode_base64("TmtKRVUxSmtjRXM=")?,
            decode_base64("NWFIRkZRbFJuVlE9PQ==")?
        ))?;

        let client_secret = decode_base64(format!(
            "{}{}",
            decode_base64("ZUdWMVVHMVpOMjVpY0ZvNVNVbGlURUZqVVQ=")?,
            decode_base64("a3pjMmhyWVRGV1RtaGxWVUZ4VGpaSlkzTjZhbFJIT0QwPQ==")?
        ))?;

        let bits: u64 = rng.random();
        let client_unique_key = format!("{:02x}", bits);

        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);
        let code_verifier = general_purpose::URL_SAFE_NO_PAD.encode(random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let hash_result = hasher.finalize();
        let code_challenge = general_purpose::URL_SAFE_NO_PAD.encode(hash_result);

        Ok(Self {
            redirect_uri: PKCE_URI_REDIRECT.to_string(),
            client_id,
            client_secret,
            client_unique_key,
            code_challenge,
            code_verifier,
        })
    }
}
