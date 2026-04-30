use crate::client::TidalClient;
use tracing::debug;

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
        debug!(
            payload_bytes = client_json.len(),
            "deserializing client session from JSON"
        );
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
    /// # use tidlers::{TidalClient, auth::TidalAuth};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = TidalAuth::with_oauth();
    /// # let client = TidalClient::new(&auth);
    /// let json = client.get_json();
    /// std::fs::write("session.json", json)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_json(&self) -> String {
        debug!("serializing client session to JSON");
        serde_json::to_string(&self).unwrap_or_else(|_| {
            panic!("failed to serialize TidalClient to JSON, something is seriously wrong here.")
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        TidalClient,
        auth::TidalAuth,
        client::models::playback::{AudioQuality, PlaybackMode},
    };

    #[test]
    fn json_roundtrip_preserves_session_state() {
        let auth = TidalAuth::with_access_token("token_123".to_string());
        let mut client = TidalClient::new(&auth);
        client.session.locale = "cs_CZ".to_string();
        client.set_time_offset("+02:00".to_string());
        client.set_audio_quality(AudioQuality::HiRes);
        client.set_playback_mode(PlaybackMode::Offline);

        let json = client.get_json();
        let restored = TidalClient::from_json(&json).expect("client json should deserialize");

        assert_eq!(
            restored.session.auth.access_token.as_deref(),
            Some("token_123")
        );
        assert_eq!(restored.session.locale, "cs_CZ");
        assert_eq!(restored.session.time_offset, "+02:00");
        assert!(matches!(
            restored.session.audio_quality,
            AudioQuality::HiRes
        ));
        assert!(matches!(
            restored.session.playback_mode,
            PlaybackMode::Offline
        ));
    }

    #[test]
    fn from_json_rejects_invalid_json() {
        let result = TidalClient::from_json("{this is invalid json");
        assert!(result.is_err());
    }
}
