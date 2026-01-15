# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Breaking:** Changed internal functions to `pub(crate)` visibility to prevent usage in external code:
  - `BasicAuth::new()`
  - `TidalRequest::new()`
  - `RequestClient::new()`
  - `TidalSession::new()`
  - `TidalAuth::is_token_auth()`
  - `TidalAuth::is_token_expired()`
  - `auth_default_request_client()`
  - `get_client_credentials()`
  - `debug_json()`
  - `debug_json_str()`
  - `SearchType::to_api_params()`
  - `ApiRequestBuilder` and all its methods (new, with_country_code, with_locale, with_base_url, with_param, with_params, with_optional_param, with_headers, send, send_raw)
  - `TidalClient::request()`

### Fixed
- **Breaking:** Changed `activity`, `artists` and `disclaimers` fields to be optional in TopArtistsResponse to prevent crashes

## [0.1.0] - 2025-01-15

### Added
- Initial release of Tidlers
- OAuth2 device flow authentication support
- Support for all TIDAL audio quality levels:
  - Low (96 kbps AAC)
  - High (320 kbps AAC)
  - Lossless (44.1 KHz FLAC)
  - HiRes (192 KHz FLAC with DASH streaming)
- DASH manifest parsing for HiRes audio streaming
- API:
  - Track information and playback
  - Album and playlist management
  - Artist information
  - User profile and subscription details
  - Search functionality
  - Mixes and recommendations
- Session persistence with JSON
- Type-safe API with custom ID types
- Complete documentation with code examples
- Example projects:
  - `testing_client` - Basic API usage examples
  - `login_save` - Session persistence example
  - `hires_streamer` - Audio streaming with playback

### Notes
- Minimum Rust version: 1.75 (edition 2021)
- Requires active TIDAL with subscription
- Some code and documentation written with AI assistance
