//! Playback reporting: telling TIDAL that a track was listened to.
//!
//! See [`TidalClient::report_playback_session`](crate::client::TidalClient::report_playback_session)
//! and [`PlaybackReporter`](crate::client::api::events::PlaybackReporter).

use serde::{Deserialize, Serialize};

/// One finished listen, as reported to TIDAL.
///
/// Timestamps are Unix epoch milliseconds; positions are seconds into the
/// track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaybackSession {
    /// A new, unique id for this listen (a UUID).
    pub playback_session_id: String,
    /// The track that was played.
    pub track_id: String,
    /// The quality that was played, as TIDAL names it (e.g. `LOSSLESS`).
    pub quality: String,
    /// What the track was played from: `ALBUM`, `PLAYLIST`, `MIX`, `ARTIST`,
    /// ... Plays without one count towards play statistics, but TIDAL only
    /// lists plays with a source in Recently Played.
    pub source_type: Option<String>,
    /// The id of that album, playlist, mix or artist.
    pub source_id: Option<String>,
    /// When playback started.
    pub start_timestamp_ms: u64,
    /// When playback stopped.
    pub end_timestamp_ms: u64,
    /// Where in the track playback started.
    pub start_position_s: f64,
    /// Where in the track playback stopped.
    pub end_position_s: f64,
}

/// The client app a playback report describes.
///
/// It has to match the client the access token was issued to: TIDAL treats
/// reports differently depending on it. For example, only plays reported by
/// mobile clients appear in Recently Played.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventClient {
    /// The app's name, e.g. `TIDAL`.
    pub app_name: String,
    /// The app's version, e.g. `2.87.0`.
    pub app_version: String,
    /// The platform, e.g. `android`.
    pub platform: String,
    /// The device type, e.g. `phone`.
    pub device_type: String,
}

impl EventClient {
    /// TIDAL's Android phone app, the client that tidlers' default PKCE
    /// credentials belong to.
    pub fn tidal_android() -> Self {
        Self {
            app_name: "TIDAL".to_string(),
            app_version: "2.87.0".to_string(),
            platform: "android".to_string(),
            device_type: "phone".to_string(),
        }
    }
}
