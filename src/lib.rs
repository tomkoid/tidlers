//! # Tidlers - TIDAL API Client
//!
//! An unofficial Rust library for interacting with the TIDAL music streaming API.
//!
//! This library has API support for tracks, albums, artists, videos, playlists, collection, mixes, search, user, subscription information and much more...
//!
//! ## Features
//!
//! - Audio quality support: Low, High, Lossless, HiRes (only with PKCE auth)
//! - Multiple auth flows:
//!   - OAuth2 device-code flow (`TidalAuth::with_oauth()`)
//!   - OAuth2 PKCE flow for HiRes streaming (`TidalAuth::with_pkce()`)
//!   - Client-credentials flow (`TidalAuth::with_api_token(...)`)
//!   - Direct access token (`TidalAuth::with_access_token(...)`)
//! - DASH manifest parsing for HiRes playback
//! - Session persistence (`get_json()` / `from_json()`)
//! - `tracing` for auth/session/request flows
//!
//! ## Example
//!
//! ```no_run
//! use tidlers::{TidalClient, auth::TidalAuth};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let auth = TidalAuth::with_oauth();
//! let mut client = TidalClient::new(&auth);
//!     
//! // Get OAuth link and wait for user authentication
//! let oauth_link = client.get_oauth_link().await?;
//! println!("Visit: {}", oauth_link.verification_uri_complete);
//!     
//! client.wait_for_oauth(
//!     &oauth_link.device_code,
//!     oauth_link.expires_in,
//!     oauth_link.interval,
//!     None
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! For more examples, check the [examples directory](https://codeberg.org/tomkoid/tidlers/src/branch/main/examples) in Tidlers

pub mod auth;
pub mod client;
pub mod error;
pub mod ids;
pub mod requests;
pub mod resources;
pub mod session;
pub mod urls;
pub mod utils;
pub use client::models::responses;

// Re-export main types for convenience
pub use client::TidalClient;
pub use error::TidalError;
pub use session::TidalSession;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
