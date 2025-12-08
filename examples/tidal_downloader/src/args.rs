use std::path::PathBuf;

use clap::Parser;

use crate::{MediaTypeArg, QualityArg};

#[derive(Parser)]
#[command(name = "tidal-downloader")]
#[command(author, version, about = "Download music from TIDAL", long_about = None)]
pub struct Cli {
    /// Media ID (track, album, or playlist)
    #[arg(value_name = "ID")]
    pub id: String,

    /// Type of media to download
    #[arg(short, long, value_enum, default_value = "auto")]
    pub media_type: MediaTypeArg,

    /// Audio quality
    #[arg(short, long, value_enum, default_value = "hires")]
    pub quality: QualityArg,

    /// Output directory
    #[arg(short, long, default_value = "downloads")]
    pub output: PathBuf,

    /// Maximum parallel downloads
    #[arg(short, long, default_value = "5")]
    pub parallel: usize,

    /// Force re-authentication
    #[arg(long)]
    pub reauth: bool,

    /// Session file path
    #[arg(long, default_value = "session.json")]
    pub session_file: PathBuf,
}
