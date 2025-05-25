# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1] - 2025-05-25

### Added

- **Dictation API Support**: Implemented POST `/dictation` endpoint for audio transcription
  - Support for streaming and blocking dictation requests
  - Audio source handling (buffered and streamed)
  - Multiple encoding format support (WAV, etc.)
  - Comprehensive test coverage with real audio file testing
- **Community Section**: Added Mastodon contact information to README for project updates and support

### Changed

- Updated README.md to reflect current API implementation status
- Fixed dependency name in README from `wit_api` to `wit_owo`
- Updated API status indicators:
  - GET `/message`: Now marked as ✔️ (fully supported and tested)
  - POST `/dictation`: Now marked as ⚠️ (supported but being improved)

### Technical

- Enhanced error handling for dictation API
- Added streaming support for real-time dictation processing
- Improved JSON parsing utilities for streaming responses
- Added Levenshtein distance utility for test validation

## [1.1.0] - Previous Release

### Added to 1.1.0

- Core message API implementation
- GET `/message` endpoint support with comprehensive query parameters
- **WitClient**: Main client for interacting with Wit.AI API
- **MessageQuery**: Builder pattern for message requests with:
  - Text queries up to 280 characters
  - Tag support for app versioning
  - Intent limit control (up to 8 intents)
  - Entity filtering
- **Robust Type System**:
  - Strong typing for all API responses
  - Enums for entities, intents, and traits
  - Confidence score handling with proper floating-point precision
- Async and blocking client support
- Feature flags for `async` and `blocking` modes
- Comprehensive error handling with `ApiError`
- Serde-based JSON serialization/deserialization
- URL encoding and parameter handling

### Technical changes to 1.1.0

- Multi-platform Support: CI/CD testing on Ubuntu, Windows, and macOS
- Comprehensive Testing: Unit tests with real API integration
- Full rustdoc documentation with examples

## [1.0.0] - Initial Release

### Added to 1.0.0

- Initial project structure
- Basic Wit.AI API client foundation
- Cargo workspace configuration
- CI/CD pipeline with GitHub Actions
- Licence files (MIT and Apache-2.0)
- Basic README with project description
- Security policy documentation
- Core dependencies' setup:
  - `reqwest` for HTTP client
  - `serde` and `serde_json` for serialization
  - `tokio` for async runtime
  - `chrono` for date/time handling
  - `thiserror` for error handling

---

## Version History Summary

- **1.1.1**: Added dictation API support and community information
- **1.1.0**: Core message API implementation with full feature set
- **1.0.0**: Initial project foundation and setup

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Licence

This project is licensed under either of

- Apache Licence, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT licence ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

At your option.
