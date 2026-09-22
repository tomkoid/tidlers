/// The tiers a release is available in, as TIDAL advertises them:
/// `LOSSLESS`, `HIRES_LOSSLESS`, `DOLBY_ATMOS` and so on.
///
/// `Default` is the empty list, which is what an absent `mediaMetadata` means:
/// nothing advertised, rather than nothing known.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MediaMetadata {
    #[serde(default)]
    pub tags: Vec<String>,
}
