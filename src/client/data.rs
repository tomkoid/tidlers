use crate::client::TidalClient;

impl TidalClient {
    /// Deserializes a TidalClient from a JSON string
    ///
    /// Useful for persisting and restoring authenticated sessions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::TidalClient;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let json = std::fs::read_to_string("session.json")?;
    /// let client = TidalClient::from_json(&json)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_json(client_json: &str) -> Result<TidalClient, serde_json::Error> {
        let client_json: TidalClient = serde_json::from_str(client_json)?;

        Ok(TidalClient {
            user_info: client_json.user_info,
            session: client_json.session,
            ..Self::default()
        })
    }

    /// Serializes the TidalClient to a JSON string
    ///
    /// Useful for persisting authenticated sessions to disk.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use tidlers::{TidalClient, auth::init::TidalAuth};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let json = client.get_json();
    /// std::fs::write("session.json", json)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
