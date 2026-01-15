//! # Tidlers - Tidal API Client
//!
//! A Rust library for interacting with the Tidal music streaming API.
//!
//! ## Features
//!
//! - OAuth2 authentication flow
//! - Track, album, artist, and playlist information retrieval
//! - Search functionality
//! - Audio quality configuration
//! - Session management and serialization
//!
//! ## Example
//!
//! ```no_run
//! use tidlers::{TidalClient, auth::init::TidalAuth};
//!
//! #[tokio::main]
//! async fn main() {
//!     let auth = TidalAuth::with_oauth();
//!     let mut client = TidalClient::new(&auth);
//!     
//!     // Get OAuth link and wait for user authentication
//!     let oauth_link = client.get_oauth_link().await.unwrap();
//!     println!("Visit: {}", oauth_link.verification_uri_complete);
//!     
//!     client.wait_for_oauth(
//!         &oauth_link.device_code,
//!         oauth_link.expires_in,
//!         oauth_link.interval,
//!         None
//!     ).await.unwrap();
//! }
//! ```

pub mod auth;
pub mod client;
pub mod error;
pub mod ids;
pub mod requests;
pub mod responses;
pub mod session;
pub mod utils;

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
