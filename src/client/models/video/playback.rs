/// Response containing video playback information including manifest data
///
/// Same tolerance as [`TrackPlaybackInfoResponse`](crate::client::models::track::playback::TrackPlaybackInfoResponse):
/// only `video_id` is required, so an omitted descriptive field costs the
/// caller a default rather than the whole response.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlaybackInfoResponse {
    pub video_id: u64,
    #[serde(default)]
    pub asset_presentation: String,
    #[serde(default)]
    pub stream_type: String,
    #[serde(default)]
    pub video_quality: String,
    #[serde(default)]
    pub manifest_mime_type: String,
    #[serde(default)]
    pub manifest_hash: String,
    #[serde(skip_deserializing)]
    pub manifest: Option<EmuVideoManifest>,
    #[serde(skip_deserializing, default)]
    pub manifest_raw: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmuVideoManifest {
    pub mime_type: String,
    pub urls: Vec<String>,
}
