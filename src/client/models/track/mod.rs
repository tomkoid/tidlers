use std::collections::HashMap;

use crate::client::models::{album::Album, artist::Artist, media::MediaMetadata};

pub mod config;
pub mod playback;
pub mod user_uploads;

/// Represents a track
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub duration: u64,
    pub replay_gain: f64,
    pub peak: f32,
    pub allow_streaming: bool,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: Option<String>,
    pub premium_streaming_only: bool,
    pub track_number: u32,
    pub volume_number: u32,
    pub version: Option<String>,
    pub popularity: u32,
    pub copyright: Option<String>,
    pub bpm: Option<f32>,
    pub key: Option<String>,
    pub key_scale: Option<String>,
    pub url: String,
    pub isrc: Option<String>,
    pub editable: bool,
    pub explicit: bool,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub media_metadata: Option<MediaMetadata>,
    pub upload: bool,
    pub access_type: Option<String>,
    pub spotlighted: Option<bool>,
    pub date_added: Option<String>,
    pub index: Option<u64>,
    pub artist: Artist,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    pub mixes: Option<HashMap<String, String>>,
    pub item_uuid: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackRadioResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<Track>,
}

/// Lyrics text and provider metadata for a track.
///
/// Plain lyrics and provider metadata may be absent or null, including in
/// responses containing timed subtitles. `None` means absent, while an empty
/// string stays `Some("")`. Text and subtitles are preserved verbatim for the
/// caller to interpret; an omitted direction flag defaults to left-to-right.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResponse {
    pub track_id: u32,
    pub lyrics_provider: Option<String>,
    pub provider_commontrack_id: Option<String>,
    pub provider_lyrics_id: Option<String>,
    pub lyrics: Option<String>,
    pub subtitles: Option<String>,
    #[serde(rename = "isRightToLeft", default)]
    pub right_to_left: bool,
}

#[cfg(test)]
mod tests {
    use crate::client::models::track::playback::{
        DashManifest, JsonTrackManifest, ParsedTrackManifest, TrackPlaybackInfoResponse,
    };

    #[test]
    fn playback_info_helpers_work_for_json_manifest() {
        let manifest = JsonTrackManifest {
            mime_type: "audio/flac".to_string(),
            codecs: "flac".to_string(),
            encryption_type: "NONE".to_string(),
            urls: vec!["https://example.com/a.flac".to_string()],
        };

        let playback = TrackPlaybackInfoResponse {
            track_id: 1,
            asset_presentation: "FULL".to_string(),
            audio_mode: "STEREO".to_string(),
            audio_quality: "LOSSLESS".to_string(),
            manifest_mime_type: "application/json".to_string(),
            manifest_hash: "hash".to_string(),
            manifest: Some(manifest.clone()),
            manifest_parsed: Some(ParsedTrackManifest::Json(manifest)),
            manifest_raw: None,
            bit_depth: Some(16),
            sample_rate: Some(44100),
            album_replay_gain: 0.0,
            album_peak_amplitude: 0.0,
            track_replay_gain: 0.0,
            track_peak_amplitude: 0.0,
        };

        assert_eq!(
            playback.get_primary_url().as_deref(),
            Some("https://example.com/a.flac")
        );
        assert_eq!(playback.get_mime_type().as_deref(), Some("audio/flac"));
        assert_eq!(playback.get_codecs().as_deref(), Some("flac"));
    }

    #[test]
    fn dash_manifest_segment_url_uses_template() {
        let dash = DashManifest {
            mime_type: "audio/mp4".to_string(),
            codecs: "flac".to_string(),
            urls: vec![],
            bitrate: Some(1),
            initialization_url: Some("init.mp4".to_string()),
            media_url_template: Some("seg-$Number$.m4s".to_string()),
            timescale: Some(1),
            duration: Some(1),
            start_number: Some(1),
        };

        assert_eq!(dash.get_segment_url(42).as_deref(), Some("seg-42.m4s"));
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TrackOld {
//     pub id: i64,
//     pub title: String,
//     pub duration: i32,
//     pub replay_gain: f64,
//     pub peak: f64,
//     pub allow_streaming: bool,
//     pub stream_ready: bool,
//     pub pay_to_stream: bool,
//     pub ad_supported_stream_ready: bool,
//     pub dj_ready: bool,
//     pub stem_ready: bool,
//     pub stream_start_date: String,
//     pub premium_streaming_only: bool,
//     pub track_number: i32,
//     pub volume_number: i32,
//     pub version: Option<String>,
//     pub popularity: i32,
//     pub copyright: String,
//     pub bpm: Option<i32>,
//     pub description: Option<String>,
//     pub url: String,
//     pub isrc: String,
//     pub editable: bool,
//     pub explicit: bool,
//     pub audio_quality: String,
//     pub audio_modes: Vec<String>,
//     pub media_metadata: MediaMetadata,
//     pub upload: bool,
//     pub access_type: String,
//     pub spotlighted: bool,
//     pub artist: Artist,
//     pub artists: Vec<Artist>,
//     pub album: Album,
//     pub mixes: Option<Mixes>,
//     pub date_added: Option<String>,
//     pub index: Option<i64>,
//     pub item_uuid: Option<String>,
// }
//

#[cfg(test)]
mod lyrics_tests {
    use super::LyricsResponse;
    use serde_json::json;

    #[test]
    fn complete_lyrics_payload_round_trips() {
        let payload = json!({
            "trackId": 123, "lyricsProvider": "Provider",
            "providerCommontrackId": "common-1", "providerLyricsId": "lyrics-1",
            "lyrics": "First line\nSecond line", "subtitles": "[00:01.00]First line",
            "isRightToLeft": false
        });
        let lyrics: LyricsResponse = serde_json::from_value(payload.clone()).unwrap();
        assert_eq!(lyrics.lyrics_provider.as_deref(), Some("Provider"));
        assert_eq!(lyrics.provider_commontrack_id.as_deref(), Some("common-1"));
        assert_eq!(lyrics.provider_lyrics_id.as_deref(), Some("lyrics-1"));
        assert_eq!(serde_json::to_value(lyrics).unwrap(), payload);
    }

    #[test]
    fn subtitles_only_payload_keeps_timing_and_text() {
        let timed = "[offset:+20]\n[00:01.20]First line\n[00:03.45]Second line";
        let lyrics: LyricsResponse = serde_json::from_value(json!({
            "trackId": 123, "subtitles": timed
        }))
        .unwrap();
        assert_eq!(lyrics.track_id, 123);
        assert_eq!(lyrics.subtitles.as_deref(), Some(timed));
        assert_eq!(lyrics.subtitles.as_ref().unwrap().lines().count(), 3);
        assert!(lyrics.lyrics.is_none());
        assert!(lyrics.lyrics_provider.is_none());
        assert!(lyrics.provider_commontrack_id.is_none());
        assert!(lyrics.provider_lyrics_id.is_none());
        assert!(!lyrics.right_to_left);
    }

    #[test]
    fn null_provider_and_lyrics_fields_are_optional() {
        let lyrics: LyricsResponse = serde_json::from_value(json!({
            "trackId": 123, "lyricsProvider": null,
            "providerCommontrackId": null, "providerLyricsId": null,
            "lyrics": null, "subtitles": null
        }))
        .unwrap();
        assert!(lyrics.lyrics_provider.is_none());
        assert!(lyrics.provider_commontrack_id.is_none());
        assert!(lyrics.provider_lyrics_id.is_none());
        assert!(lyrics.lyrics.is_none());
        assert!(lyrics.subtitles.is_none());
        assert!(!lyrics.right_to_left);
        assert!(serde_json::to_value(lyrics).unwrap()["lyrics"].is_null());
    }

    #[test]
    fn empty_strings_remain_distinct_from_absent_metadata() {
        let lyrics: LyricsResponse = serde_json::from_value(json!({
            "trackId": 123, "lyricsProvider": "", "providerCommontrackId": "",
            "providerLyricsId": "", "lyrics": "", "subtitles": ""
        }))
        .unwrap();
        assert_eq!(lyrics.lyrics_provider.as_deref(), Some(""));
        assert_eq!(lyrics.provider_commontrack_id.as_deref(), Some(""));
        assert_eq!(lyrics.provider_lyrics_id.as_deref(), Some(""));
        assert_eq!(lyrics.lyrics.as_deref(), Some(""));
        assert_eq!(lyrics.subtitles.as_deref(), Some(""));
        assert_eq!(serde_json::to_value(lyrics).unwrap()["lyrics"], "");
    }

    #[test]
    fn plain_lyrics_and_explicit_right_to_left_are_preserved() {
        let text = "  مرحباً\nبالعالم  ";
        let lyrics: LyricsResponse = serde_json::from_value(json!({
            "trackId": 123, "lyrics": text, "isRightToLeft": true
        }))
        .unwrap();
        assert_eq!(lyrics.lyrics.as_deref(), Some(text));
        assert!(lyrics.subtitles.is_none());
        assert!(lyrics.right_to_left);
    }

    #[test]
    fn missing_identity_and_invalid_known_field_types_still_fail() {
        for payload in [
            json!({"status": 404, "userMessage": "Not found"}),
            json!({"lyrics": "No track id"}),
            json!({"trackId": 123, "lyrics": ["not", "a", "string"]}),
            json!({"trackId": 123, "isRightToLeft": "not a boolean"}),
        ] {
            assert!(serde_json::from_value::<LyricsResponse>(payload).is_err());
        }
    }
}
