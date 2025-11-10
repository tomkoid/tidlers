use color_eyre::eyre::Result;
use tidlers::client::{models::playback::AudioQuality, tidal::TidalClient};

use crate::{auth::handle_auth, save::save_session_data, streamer::DashStreamer};

mod auth;
mod oauth_handler;
mod save;
mod streamer;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    println!("=== Tidlers HiRes Audio Streamer ===\n");

    // handle authentication and create Tidal client
    let mut tidal = if let Some(auth) = handle_auth().await? {
        TidalClient::new(&auth)
    } else {
        let saved_session_data = save::get_session_data().unwrap();
        let mut cl = TidalClient::from_serialized(&saved_session_data)?;

        let refreshed = cl.refresh_access_token(false).await?;
        if refreshed {
            println!("token refreshed from saved session data");
        } else {
            println!("using saved session data");
        }

        cl
    };

    // if waiting for oauth login, handle oauth flow
    if tidal.waiting_for_oauth_login() {
        println!("Handling OAuth flow...");
        auth::handle_oauth_flow(&mut tidal).await?;
        println!("OAuth flow complete\n");
    }

    println!("Logged in successfully!");

    // get and save user info
    tidal.refresh_user_info().await?;
    save_session_data(&tidal.get_json());

    println!("User: {}", tidal.user_info.as_ref().unwrap().username);

    // get subscription info
    let subscription = tidal.subscription().await?;
    println!(
        "Subscription: {}\n",
        subscription.subscription.subscription_type
    );

    // Track ID to stream
    print!("\nTrack ID to stream: ");
    io::stdout().flush()?;

    let mut track_id_input = String::new();
    io::stdin().read_line(&mut track_id_input)?;
    let track_id = track_id_input.trim();

    println!("Fetching track information...");
    let track_info = tidal.get_track(track_id.to_string()).await?;
    println!("\n=== Track Info ===");
    println!("Title: {}", track_info.title);
    println!("Artist: {}", track_info.artist.name);
    println!("Album: {}", track_info.album.title);
    println!("Duration: {}s", track_info.duration);
    println!("==================\n");

    // Prompt user for audio quality
    println!("Select audio quality:");
    println!("1. Low (96 kbps)");
    println!("2. High (320 kbps)");
    println!("3. Lossless (1411 kbps FLAC)");
    println!("4. HiRes (up to 9216 kbps FLAC)");
    print!("\nEnter choice (1-4, default 4): ");

    use std::io::{self, Write};
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let audio_quality = match choice {
        "1" => AudioQuality::Low,
        "2" => AudioQuality::High,
        "3" => AudioQuality::Lossless,
        "" | "4" => AudioQuality::HiRes,
        _ => {
            println!("Invalid choice, using HiRes");
            AudioQuality::HiRes
        }
    };

    println!("\nSetting audio quality to: {:?}", audio_quality);
    tidal.set_audio_quality(audio_quality.clone());

    // get playback information
    println!("Fetching playback information...");
    let playback_info = tidal
        .get_track_postpaywall_playback_info(track_id.to_string())
        .await?;

    println!("Audio Mode: {}", playback_info.audio_mode);
    println!("Audio Quality: {}", playback_info.audio_quality);
    println!("Manifest MIME Type: {}", playback_info.manifest_mime_type);

    // Prompt for number of segments (for HiRes/DASH only)
    let max_segments = if matches!(audio_quality, AudioQuality::HiRes) {
        print!(
            "\nHow many segments to download? (Enter for all, or a number like 10 for preview): "
        );
        io::stdout().flush()?;

        let mut segments_input = String::new();
        io::stdin().read_line(&mut segments_input)?;
        let segments_input = segments_input.trim();

        if segments_input.is_empty() {
            None
        } else {
            segments_input.parse::<u32>().ok()
        }
    } else {
        None
    };

    // Create streamer and play the track
    let streamer = DashStreamer::new();

    println!("\n=== Starting Playback ===");
    streamer.stream_track(&playback_info, max_segments).await?;

    println!("\n=== Stream Complete ===");

    Ok(())
}
