# Tidlers

A Rust library for interacting with the TIDAL music streaming API.

## Features

- OAuth 2.0 authentication
- Access token management and refresh
- Support for all TIDAL audio quality levels (Low, High, Lossless, HiRes)
- DASH manifest parsing for HiRes audio streaming
- API coverage:
  - Track information and playback
  - Album and playlist management
  - Artist information
  - User profile and subscription details
  - Mixes and recommendations
- Session persistence for seamless re-authentication
- Type-safe API with serde-based deserialization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tidlers = { git = "https://codeberg.org/tomkoid/tidlers.git" }
```

## Quick Start

```rust
use tidlers::{
    auth::init::TidalAuth,
    client::tidal::TidalClient,
    client::models::playback::AudioQuality,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with OAuth authentication
    let auth = TidalAuth::with_oauth();
    let mut client = TidalClient::new(&auth);
    
    // Get OAuth link for user to authorize
    let oauth = client.get_oauth_link().await?;
    println!("Visit: https://{}", oauth.verification_uri_complete);
    
    // Wait for authorization
    client.wait_for_oauth(
        &oauth.device_code,
        oauth.expires_in,
        oauth.interval,
        None,
    ).await?;
    
    // Get user information
    client.refresh_user_info().await?;
    println!("Logged in as: {}", client.user_info.unwrap().username);
    
    // Get track information
    let track = client.get_track("66035607".to_string()).await?;
    println!("Track: {} by {}", track.title, track.artist.name);
    
    // Get playback info for HiRes audio
    client.set_audio_quality(AudioQuality::HiRes);
    let playback = client.get_track_postpaywall_playback_info("66035607".to_string()).await?;
    
    // Access stream URLs
    if let Some(urls) = playback.get_stream_urls() {
        println!("Stream URLs: {:?}", urls);
    }
    
    Ok(())
}
```

## Audio Quality Support

Tidlers supports all TIDAL audio quality tiers:

- **Low**: 96 kbps AAC
- **High**: 320 kbps AAC
- **Lossless**: 44.1 KHz FLAC (CD quality)
- **HiRes**: 192 KHz FLAC (High-Res)

HiRes audio uses DASH (Dynamic Adaptive Streaming over HTTP) manifests, which are automatically parsed by the library.

## Session Persistence

Save and restore sessions to avoid re-authentication:

```rust
// Save session
let session_json = client.get_json();
std::fs::write("session.json", session_json)?;

// Load session
let session_data = std::fs::read_to_string("session.json")?;
let mut client = TidalClient::from_serialized(&session_data)?;

// Refresh token if needed
client.refresh_access_token().await?;
```

## API Examples

### Get Track Information

```rust
let track = client.get_track("track_id".to_string()).await?;
println!("Title: {}", track.title);
println!("Artist: {}", track.artist.name);
println!("Duration: {}s", track.duration);
```

### Get Album Details

```rust
let album = client.get_album("album_id".to_string()).await?;

// Takes in the album id with optional limit and offset for pagination
let items = client.get_album_items("album_id".to_string(), Some(50), Some(0)).await?;
```

### Get Playlist

```rust
let playlist = client.get_playlist("playlist_uuid".to_string()).await?;

// Takes in the playlist uuid with optional limit and offset for pagination
let items = client.get_playlist_items("playlist_uuid".to_string(), Some(50), Some(0)).await?;
```

### Get Track Mixes (Recommendations)

```rust
let mix = client.get_track_mix("track_id".to_string()).await?;
```

### Check Subscription

```rust
let subscription = client.subscription().await?;
println!("Type: {:?}", subscription.subscription.subscription_type);
```

## DASH Manifest Support

For HiRes audio, TIDAL uses DASH streaming. Tidlers parses these manifests automatically:

```rust
let playback_info = client.get_track_postpaywall_playback_info(track_id).await?;

match &playback_info.manifest_parsed {
    Some(ManifestType::Dash(dash)) => {
        println!("Codec: {}", dash.codecs);
        println!("Bitrate: {} bps", dash.bitrate.unwrap_or(0));
        
        // Get initialization segment
        if let Some(init_url) = dash.get_init_url() {
            // Download init segment
        }
        
        // Get individual segments
        for i in 1..=10 {
            if let Some(segment_url) = dash.get_segment_url(i) {
                // Download segment i
            }
        }
    }
    Some(ManifestType::Json(json)) => {
        // Standard JSON manifest for non-HiRes
        println!("Direct URL: {}", json.urls[0]);
    }
    None => {}
}
```

## Examples

The repository includes several examples demonstrating different use cases:

### testing_client

Basic example showing authentication, API calls, and data retrieval:

```bash
cargo run -p testing-client
```

This example is also where I test new features during development.

### hires_streamer

Complete audio streaming example with playback support:

```bash
cargo run -p hires-streamer
```

## Error Handling

The library uses the `TidalError` enum for error handling:

```rust
use tidlers::error::TidalError;

match client.get_track(track_id).await {
    Ok(track) => println!("Got track: {}", track.title),
    Err(TidalError::NotAuthenticated) => println!("Please login first"),
    Err(TidalError::Request(e)) => println!("Network error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Requirements

- Rust 2024 edition
- TIDAL account with active subscription
- Network connection for API access (obviously)

## Dependencies

Core dependencies:
- `reqwest` - HTTP client
- `serde` / `serde_json` - Serialization
- `tokio` - Async runtime
- `base64` - Base64 encoding/decoding
- `quick-xml` - XML parsing for DASH manifests
- `thiserror` - Error handling

## Development

Build the library:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run an example:

```bash
cargo run -p testing-client
cargo run -p hires-streamer
```

## Notes

- OAuth device code flow requires user interaction via browser
- DASH segments for HiRes audio are MP4 fragments that need to be combined
- Rate limiting is not implemented; use responsibly
- Some parts of code and documentation are written using AI

## License

This project is for educational and personal use. Ensure compliance with TIDAL's Terms of Service.

## Disclaimer

This is an unofficial library and is not affiliated with or endorsed by TIDAL. Use at your own risk.
