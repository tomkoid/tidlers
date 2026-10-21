//! Playback reporting (TIDAL's event producer).
//!
//! Official clients report every listen to `POST {EVENTS_LOCATION}/event-batch`.
//! Those reports put tracks in the user's Recently Played and listening
//! history, count towards play statistics and inform recommendations.
//!
//! Nothing is reported unless the app asks: call
//! [`TidalClient::report_playback_session`] for each listen, or start a
//! [`PlaybackReporter`] and queue listens on it.
//!
//! The request is AWS SQS `SendMessageBatch` form encoding: one entry whose
//! body is a JSON `playback_session` event (group `play_log`, version 2) and
//! whose `Headers` message attribute carries the client's identity and the
//! raw access token. The user, numeric client and session ids come from the
//! access token's claims.

use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine, engine::general_purpose};
use tokio::sync::mpsc;
use tracing::{debug, warn};

use crate::{
    client::{
        TidalClient,
        models::events::{EventClient, PlaybackSession},
    },
    error::TidalError,
    urls::EVENTS_LOCATION,
};

const EVENT_NAME: &str = "playback_session";
const EVENT_GROUP: &str = "play_log";
const EVENT_VERSION: u32 = 2;
/// Playback reports are needed for the service to work as expected.
const CONSENT_CATEGORY: &str = "NECESSARY";

impl TidalClient {
    /// Reports one finished listen to TIDAL, as `app`, with this client's
    /// current access token.
    ///
    /// `app` has to describe the client the access token was issued to; see
    /// [`EventClient`].
    pub async fn report_playback_session(
        &self,
        session: &PlaybackSession,
        app: &EventClient,
    ) -> Result<(), TidalError> {
        let token = self
            .session
            .auth
            .access_token
            .as_deref()
            .ok_or(TidalError::NotAuthenticated)?;
        let claims = TokenClaims::decode(token);
        let user_id = claims
            .user_id
            .clone()
            .or_else(|| self.session.auth.user_id.map(|id| id.to_string()));
        let now = now_ms();
        let body = message_body(session, &claims, user_id.as_deref(), app, now, &uuid_v4());
        let headers = headers_attribute(token, claims.client_id.as_deref(), app, now);

        let mut request = self
            .request(reqwest::Method::POST, "/event-batch")
            .with_base_url(EVENTS_LOCATION);
        for (key, value) in sqs_batch_entry(&uuid_v4(), &body, &headers) {
            request = request.with_form_param(key, value);
        }
        request.send_raw().await?;
        Ok(())
    }
}

/// Reports listens in the background, so playback never waits on the
/// network.
///
/// Created with [`PlaybackReporter::spawn`], which starts a worker on the
/// current Tokio runtime; it stops when the reporter is dropped. Each
/// [`report`](Self::report) takes the client to send with, so reports use the
/// app's current access token even after it has been refreshed.
#[derive(Debug, Clone)]
pub struct PlaybackReporter {
    queue: mpsc::UnboundedSender<(TidalClient, PlaybackSession)>,
}

impl PlaybackReporter {
    /// Starts the reporting worker, which reports as `app`.
    ///
    /// # Panics
    ///
    /// Outside a Tokio runtime.
    pub fn spawn(app: EventClient) -> Self {
        let (queue, mut listens) = mpsc::unbounded_channel::<(TidalClient, PlaybackSession)>();
        tokio::spawn(async move {
            while let Some((client, session)) = listens.recv().await {
                match client.report_playback_session(&session, &app).await {
                    Ok(()) => debug!(track_id = %session.track_id, "reported playback session"),
                    Err(error) => warn!(
                        track_id = %session.track_id,
                        %error,
                        "failed to report playback session"
                    ),
                }
            }
        });
        Self { queue }
    }

    /// Queues a finished listen, to be reported with `client`'s access token.
    /// Never blocks; failures are logged.
    pub fn report(&self, client: &TidalClient, session: PlaybackSession) {
        if self.queue.send((client.clone(), session)).is_err() {
            warn!("playback reporter has stopped; listen not reported");
        }
    }
}

/// The access token claims a report carries. TIDAL attributes the play from
/// the user id (`uid`) and the numeric client id (`cid`); `sid` is echoed as
/// the session id mobile clients send.
#[derive(Debug, Default, PartialEq)]
struct TokenClaims {
    user_id: Option<String>,
    client_id: Option<String>,
    session_id: Option<String>,
}

impl TokenClaims {
    /// Best-effort: an undecodable token yields no claims, and the report is
    /// still sent.
    fn decode(token: &str) -> Self {
        let Some(payload) = token.split('.').nth(1) else {
            return Self::default();
        };
        let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(payload.trim_end_matches('='))
        else {
            return Self::default();
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            return Self::default();
        };
        // TIDAL sends these as numbers; accept strings too.
        let claim = |name: &str| match value.get(name) {
            Some(serde_json::Value::String(s)) => Some(s.clone()),
            Some(serde_json::Value::Number(n)) => Some(n.to_string()),
            _ => None,
        };
        Self {
            user_id: claim("uid").or_else(|| claim("sub")),
            client_id: claim("cid"),
            session_id: claim("sid"),
        }
    }
}

fn message_body(
    session: &PlaybackSession,
    claims: &TokenClaims,
    user_id: Option<&str>,
    app: &EventClient,
    now_ms: u64,
    event_id: &str,
) -> String {
    let number = |id: Option<&str>| id.and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    let client_id = number(claims.client_id.as_deref());
    serde_json::json!({
        "group": EVENT_GROUP,
        "version": EVENT_VERSION,
        "ts": now_ms,
        "uuid": event_id,
        "user": {
            "id": number(user_id),
            "clientId": client_id,
            "sessionId": claims.session_id.as_deref().unwrap_or(""),
        },
        "client": {
            "token": client_id.to_string(),
            "deviceType": app.device_type,
            "version": app.app_version,
            "platform": app.platform,
        },
        "payload": {
            "playbackSessionId": session.playback_session_id,
            "productType": "TRACK",
            "actualProductId": session.track_id,
            "requestedProductId": session.track_id,
            "actualAssetPresentation": "FULL",
            "actualAudioMode": "STEREO",
            "actualQuality": session.quality,
            // Unset is an empty string: TIDAL drops events whose source is null.
            "sourceType": session.source_type.as_deref().unwrap_or(""),
            "sourceId": session.source_id.as_deref().unwrap_or(""),
            "startTimestamp": session.start_timestamp_ms,
            "endTimestamp": session.end_timestamp_ms,
            "startAssetPosition": session.start_position_s,
            "endAssetPosition": session.end_position_s,
            "isPostPaywall": true,
            "actions": [
                {
                    "actionType": "PLAYBACK_START",
                    "assetPosition": session.start_position_s,
                    "timestamp": session.start_timestamp_ms,
                },
                {
                    "actionType": "PLAYBACK_STOP",
                    "assetPosition": session.end_position_s,
                    "timestamp": session.end_timestamp_ms,
                },
            ],
        },
        "extras": null,
    })
    .to_string()
}

/// The `Headers` message attribute. `authorization` is the raw token, without
/// the `Bearer ` prefix the HTTP header has.
fn headers_attribute(
    token: &str,
    client_id: Option<&str>,
    app: &EventClient,
    now_ms: u64,
) -> String {
    serde_json::json!({
        "app-name": app.app_name,
        "app-version": app.app_version,
        "client-id": client_id.unwrap_or("unknown"),
        "consent-category": CONSENT_CATEGORY,
        "os-name": app.platform,
        "requested-sent-timestamp": now_ms,
        "authorization": token,
    })
    .to_string()
}

/// One `SendMessageBatch` entry (entries are numbered from 1) with the event
/// name and headers as message attributes.
fn sqs_batch_entry(entry_id: &str, body: &str, headers: &str) -> Vec<(String, String)> {
    const ENTRY: &str = "SendMessageBatchRequestEntry.1";
    [
        ("Id", entry_id),
        ("MessageBody", body),
        ("MessageAttribute.1.Name", "Name"),
        ("MessageAttribute.1.Value.StringValue", EVENT_NAME),
        ("MessageAttribute.1.Value.DataType", "String"),
        ("MessageAttribute.2.Name", "Headers"),
        ("MessageAttribute.2.Value.StringValue", headers),
        ("MessageAttribute.2.Value.DataType", "String"),
    ]
    .into_iter()
    .map(|(key, value)| (format!("{ENTRY}.{key}"), value.to_string()))
    .collect()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// A random (version 4) UUID.
fn uuid_v4() -> String {
    let mut b: [u8; 16] = rand::random();
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let hex: String = b.iter().map(|byte| format!("{byte:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(payload: &str) -> String {
        let encode = |s: &str| general_purpose::URL_SAFE_NO_PAD.encode(s);
        format!("{}.{}.", encode(r#"{"alg":"none"}"#), encode(payload))
    }

    fn session(source: Option<(&str, &str)>) -> PlaybackSession {
        PlaybackSession {
            playback_session_id: "listen-1".to_string(),
            track_id: "12345".to_string(),
            quality: "LOSSLESS".to_string(),
            source_type: source.map(|(kind, _)| kind.to_string()),
            source_id: source.map(|(_, id)| id.to_string()),
            start_timestamp_ms: 1_700_000_000_000,
            end_timestamp_ms: 1_700_000_180_000,
            start_position_s: 0.0,
            end_position_s: 180.0,
        }
    }

    #[test]
    fn token_claims_are_read_from_numbers_or_strings() {
        let claims = TokenClaims::decode(&token(r#"{"uid":12345,"cid":8017,"sid":"sess-abc"}"#));
        assert_eq!(
            claims,
            TokenClaims {
                user_id: Some("12345".to_string()),
                client_id: Some("8017".to_string()),
                session_id: Some("sess-abc".to_string()),
            }
        );
        let sub = TokenClaims::decode(&token(r#"{"sub":"42"}"#));
        assert_eq!(sub.user_id.as_deref(), Some("42"));
        assert_eq!(TokenClaims::decode("not a token"), TokenClaims::default());
    }

    #[test]
    fn message_body_describes_the_listen_and_the_client() {
        let claims = TokenClaims::decode(&token(r#"{"uid":42,"cid":8017,"sid":"sess-abc"}"#));
        let body = message_body(
            &session(Some(("ALBUM", "999"))),
            &claims,
            claims.user_id.as_deref(),
            &EventClient::tidal_android(),
            7,
            "event-1",
        );
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["group"], "play_log");
        assert_eq!(v["version"], 2);
        assert_eq!(v["uuid"], "event-1");
        assert_eq!(v["user"]["id"], 42);
        assert_eq!(v["user"]["clientId"], 8017);
        assert_eq!(v["user"]["sessionId"], "sess-abc");
        assert_eq!(v["client"]["platform"], "android");
        assert_eq!(v["client"]["deviceType"], "phone");
        assert_eq!(v["payload"]["playbackSessionId"], "listen-1");
        assert_eq!(v["payload"]["actualProductId"], "12345");
        assert_eq!(v["payload"]["actualQuality"], "LOSSLESS");
        assert_eq!(v["payload"]["sourceType"], "ALBUM");
        assert_eq!(v["payload"]["sourceId"], "999");
        let actions = v["payload"]["actions"].as_array().unwrap();
        assert_eq!(actions[0]["actionType"], "PLAYBACK_START");
        assert_eq!(actions[1]["actionType"], "PLAYBACK_STOP");
        assert_eq!(actions[1]["assetPosition"], 180.0);
    }

    #[test]
    fn a_listen_without_a_source_sends_empty_strings_not_null() {
        let body = message_body(
            &session(None),
            &TokenClaims::default(),
            None,
            &EventClient::tidal_android(),
            0,
            "event-1",
        );
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["payload"]["sourceType"], "");
        assert_eq!(v["payload"]["sourceId"], "");
        assert_eq!(v["user"]["id"], 0);
    }

    #[test]
    fn headers_attribute_carries_the_raw_token() {
        let attr = headers_attribute("raw-token", Some("8017"), &EventClient::tidal_android(), 7);
        let v: serde_json::Value = serde_json::from_str(&attr).unwrap();
        assert_eq!(v["authorization"], "raw-token");
        assert_eq!(v["client-id"], "8017");
        assert_eq!(v["app-name"], "TIDAL");
        assert_eq!(v["app-version"], "2.87.0");
        assert_eq!(v["os-name"], "android");
        assert_eq!(v["consent-category"], "NECESSARY");
    }

    #[test]
    fn batch_has_one_entry_with_name_and_headers_attributes() {
        let entry: std::collections::HashMap<_, _> =
            sqs_batch_entry("msg-7", "the-body", "the-headers")
                .into_iter()
                .collect();
        let field = |key: &str| entry[&format!("SendMessageBatchRequestEntry.1.{key}")].as_str();
        assert_eq!(entry.len(), 8);
        assert_eq!(field("Id"), "msg-7");
        assert_eq!(field("MessageBody"), "the-body");
        assert_eq!(field("MessageAttribute.1.Name"), "Name");
        assert_eq!(
            field("MessageAttribute.1.Value.StringValue"),
            "playback_session"
        );
        assert_eq!(field("MessageAttribute.2.Name"), "Headers");
        assert_eq!(field("MessageAttribute.2.Value.StringValue"), "the-headers");
        assert_eq!(field("MessageAttribute.2.Value.DataType"), "String");
    }

    #[test]
    fn uuids_are_random_version_4() {
        let (a, b) = (uuid_v4(), uuid_v4());
        assert_ne!(a, b);
        assert_eq!(a.len(), 36);
        assert_eq!(&a[14..15], "4");
        assert!(matches!(&a[19..20], "8" | "9" | "a" | "b"));
    }

    #[tokio::test]
    async fn reports_without_a_session_fail_cleanly() {
        let client = TidalClient::new(&crate::auth::TidalAuth::default());
        let result = client
            .report_playback_session(&session(None), &EventClient::tidal_android())
            .await;
        assert!(matches!(result, Err(TidalError::NotAuthenticated)));
    }
}
