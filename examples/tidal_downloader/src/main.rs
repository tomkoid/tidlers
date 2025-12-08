use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use tidlers::client::models::playback::AudioQuality;

mod args;
mod auth;
mod downloader;
mod types;

use auth::{authenticate, load_or_authenticate};
use downloader::Downloader;
use types::MediaType;

use crate::args::Cli;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum QualityArg {
    Low,
    High,
    Lossless,
    HiRes,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum MediaTypeArg {
    Auto,
    Track,
    Album,
    Playlist,
}

impl From<QualityArg> for AudioQuality {
    fn from(val: QualityArg) -> Self {
        match val {
            QualityArg::Low => AudioQuality::Low,
            QualityArg::High => AudioQuality::High,
            QualityArg::Lossless => AudioQuality::Lossless,
            QualityArg::HiRes => AudioQuality::HiRes,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("TIDAL Downloader\n");

    // authenticate
    let mut client = if cli.reauth {
        println!("forcing re-authentication...\n");
        authenticate(&cli.session_file).await?
    } else {
        load_or_authenticate(&cli.session_file).await?
    };

    // set audio quality
    client.set_audio_quality(cli.quality.into());
    println!("audio quality: {:?}\n", cli.quality);

    // determine media type
    let media_type = match cli.media_type {
        MediaTypeArg::Auto => detect_media_type(&cli.id),
        MediaTypeArg::Track => MediaType::Track,
        MediaTypeArg::Album => MediaType::Album,
        MediaTypeArg::Playlist => MediaType::Playlist,
    };

    println!("media type: {:?}", media_type);
    println!("output directory: {}\n", cli.output.display());

    // create output directory
    std::fs::create_dir_all(&cli.output).context("Failed to create output directory")?;

    // create downloader
    let downloader = Downloader::new(cli.output, cli.parallel);

    // download based on type
    match media_type {
        MediaType::Track => {
            println!("downloading track {}...\n", cli.id);
            downloader.download_track(&mut client, &cli.id).await?;
        }
        MediaType::Album => {
            println!("downloading album {}...\n", cli.id);
            downloader.download_album(&mut client, &cli.id).await?;
        }
        MediaType::Playlist => {
            println!("downloading playlist {}...\n", cli.id);
            downloader.download_playlist(&mut client, &cli.id).await?;
        }
    }

    println!("\ndownload complete!");

    Ok(())
}

fn detect_media_type(id: &str) -> MediaType {
    // TIDAL IDs are usually numeric
    // in a real scenario, you might want to try fetching and see what works
    // for now, we'll default to track for single numeric IDs
    if id.parse::<u64>().is_ok() {
        MediaType::Track
    } else {
        MediaType::Album
    }
}
