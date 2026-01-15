use base64::{Engine, engine::general_purpose};

/// Retrieves the default client credentials by decoding a base64 encoded string
pub fn get_client_credentials() -> (String, String) {
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
