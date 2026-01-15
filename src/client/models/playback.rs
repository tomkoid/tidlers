use std::fmt;

/// Audio quality levels available for streaming
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AudioQuality {
    Low,
    High,
    Lossless,
    HiRes,
}

/// Playback mode for tracks
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlaybackMode {
    Stream,
    Offline,
}

/// Asset presentation type for media
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AssetPresentation {
    Full,
    Preview,
}

impl fmt::Display for AudioQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "LOW"),
            Self::High => write!(f, "HIGH"),
            Self::Lossless => write!(f, "LOSSLESS"),
            Self::HiRes => write!(f, "HI_RES"),
        }
    }
}

impl fmt::Display for PlaybackMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stream => write!(f, "STREAM"),
            Self::Offline => write!(f, "OFFLINE"),
        }
    }
}
impl fmt::Display for AssetPresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(f, "FULL"),
            Self::Preview => write!(f, "PREVIEW"),
        }
    }
}
