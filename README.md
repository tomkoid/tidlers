# Tidlers

A Rust library for interacting with the TIDAL music streaming API.

## Features

- Multiple auth flows:
  - OAuth2 device-code flow (`TidalAuth::with_oauth()`)
  - OAuth2 PKCE flow for HiRes streaming (`TidalAuth::with_pkce()`)
  - Client-credentials flow (`TidalAuth::with_api_token(...)`)
  - Direct access token (`TidalAuth::with_access_token(...)`)
- Access-token refresh support
- Session persistence (`get_json()` / `from_json()`)
- Audio quality support: Low, High, Lossless, HiRes
- DASH manifest parsing for HiRes playback
- API support for tracks, albums, artists, playlists, collection, mixes, search, user, and subscription
- `tracing` for auth/session/request flows

## Projects using Tidlers

- [yadal](https://codeberg.org/tomkoid/yadal) - Command-line downloader with parallel downloads and all quality support
- [Maré Player](https://github.com/glima/mare-player) - COSMIC TIDAL applet/standalone app

## Installation

```sh
cargo add tidlers
```

Or use the latest git version:

```toml
[dependencies]
tidlers = { git = "https://codeberg.org/tomkoid/tidlers.git" }
```

## Quick Start (OAuth device-code flow)

```rust
use tidlers::{auth::TidalAuth, TidalClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = TidalAuth::with_oauth();
    let mut client = TidalClient::new(&auth);

    let oauth = client.get_oauth_link().await?;
    println!("Visit: {}", oauth.verification_uri_complete);

    client
        .wait_for_oauth(
            &oauth.device_code,
            oauth.expires_in,
            oauth.interval,
            None,
        )
        .await?;

    let me = client.refresh_user_info().await?;
    println!("Logged in as: {}", me.username);

    Ok(())
}
```

## PKCE Quick Start

```rust
use tidlers::{auth::TidalAuth, TidalClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = TidalAuth::with_pkce();
    let mut client = TidalClient::new(&auth);

    let login_url = client.initiate_pkce_login()?;
    println!("Visit: {}", login_url);

    // After browser redirect, paste the full redirect URL:
    let mut redirect_url = String::new();
    std::io::stdin().read_line(&mut redirect_url)?;
    client.finish_pkce_login(redirect_url.trim()).await?;

    client.refresh_user_info().await?;
    Ok(())
}
```

## Session Persistence

```rust
let session_json = client.get_json();
std::fs::write("session.json", session_json)?;

let session_data = std::fs::read_to_string("session.json")?;
let mut client = TidalClient::from_json(&session_data)?;
client.refresh_access_token(false).await?;
client.refresh_user_info().await?;
```

## API Examples

### Get track info + playback info

```rust
use tidlers::client::models::playback::AudioQuality;

let track = client.get_track("66035607").await?;
println!("{} - {}", track.artist.name, track.title);

client.set_audio_quality(AudioQuality::HiRes);
let playback = client.get_track_postpaywall_playback_info("66035607").await?;
if let Some(urls) = playback.get_stream_urls() {
    println!("Stream URLs: {urls:?}");
}
```

### Album + items

```rust
let album = client.get_album("251380836").await?;
println!("Album: {}", album.title);

let items = client
    .get_album_items("251380836", Some(50), Some(0))
    .await?;
println!("Album items: {}", items.items.len());
```

### Playlist + paginated items

```rust
use tidlers::client::models::playlist::{OrderDirection, PlaylistItemsOrder};

let playlist = client.get_playlist("YOUR_PLAYLIST_UUID").await?;
println!("Playlist: {}", playlist.title);

let playlist_items = client
    .get_playlist_items(
        "YOUR_PLAYLIST_UUID",
        Some(100),
        Some(0),
        PlaylistItemsOrder::Index,
        OrderDirection::Ascending,
    )
    .await?;
println!("Playlist items: {}", playlist_items.items.len());
```

### Search

```rust
use tidlers::client::models::search::config::SearchType;

let results = client
    .search_direct("daft punk", Some(vec![SearchType::Tracks]), Some(10), Some(0))
    .await?;

println!("Found {} top hits", results.top_hits.items.len());
```

### Subscription + mixes

```rust
let subscription = client.subscription().await?;
println!(
    "Subscription type: {:?}",
    subscription.subscription.subscription_type
);

let mix = client.get_track_mix("66035607", Some(20), Some(0)).await?;
println!("Mix items: {}", mix.items.len());
```

## Examples

### `testing-client`

General CLI for testing endpoints and auth flows.

```bash
# Device-code OAuth
cargo run -p testing-client -- user-info

# PKCE auth (optional via CLI flag)
cargo run -p testing-client -- --pkce user-info
```

### `pkce-login`

Minimal PKCE login example.

```bash
cargo run -p pkce-login
```

### `login-save`

Shows login + session persistence usage.

```bash
cargo run -p login-save
```

### `hires-streamer`

HiRes streaming and DASH manifest usage.

```bash
cargo run -p hires-streamer
```

## Tracing

Tidlers emits logs via `tracing`. Example subscriber:

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn init_tracing() {
    let _ = fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("tidlers=debug")),
        )
        .try_init();
}
```

Then run with:

```bash
RUST_LOG=tidlers=debug cargo run -p testing-client -- user-info
```

## Development

```bash
cargo build
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

## Notes

- OAuth device-code and PKCE flows require browser/user interaction
- Many endpoints are country-scoped; after restoring a session, call `refresh_user_info()` before country-scoped requests
- Rate limiting is not implemented
- Some parts of code and documentation are written using AI

## License

This project is for educational and personal use. Ensure compliance with [TIDAL's Terms of Service](https://tidal.com/terms).

## Disclaimer

This is an unofficial library and is not affiliated with or endorsed by TIDAL. Use at your own risk.
