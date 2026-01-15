# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
