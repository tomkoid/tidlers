/// Response containing track playback information including manifest data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlaybackInfoResponse {
    pub video_id: u64,
    pub asset_presentation: String,
    pub stream_type: String,
    pub video_quality: String,
    pub manifest_mime_type: String,
    pub manifest_hash: String,
    #[serde(skip_deserializing)]
    pub manifest: Option<EmuVideoManifest>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmuVideoManifest {
    pub mime_type: String,
    pub urls: Vec<String>,
}
