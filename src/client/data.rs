use crate::client::TidalClient;

impl TidalClient {
    /// Deserializes a TidalClient from a JSON string
    pub fn from_json(client_json: &str) -> Result<TidalClient, serde_json::Error> {
        let client_json: TidalClient = serde_json::from_str(client_json)?;

        Ok(TidalClient {
            user_info: client_json.user_info,
            session: client_json.session,
            ..Self::default()
        })
    }

    /// Serializes the TidalClient to a JSON string
    pub fn get_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
