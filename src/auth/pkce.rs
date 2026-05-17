use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkceConfig {
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub client_unique_key: String,
    pub code_challenge: String,
    pub code_verifier: String,
}
