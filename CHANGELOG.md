# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.4] - 2025-06-01

### Changed for 1.1.4

- **Dependency Updates**: Refreshed all dependencies to their latest compatible versions
  - Updated `reqwest` to 0.12.18 for improved HTTP client performance and security
  - Updated `hyper-util` to 0.1.13 with enhanced HTTP utilities and additional features
  - Updated `openssl` to 0.10.73 for latest security patches and TLS improvements
  - Updated `parking_lot` to 0.12.4 for better synchronization primitives
  - Updated `cc`, `lock_api`, `socket2`, and other transitive dependencies
- **Build System**: Cargo.lock refreshed to ensure reproducible builds with latest dependency versions

### Technical changes for 1.1.4

- No API changes or breaking modifications in this patch release
- Maintains full backward compatibility with existing Wit.ai API integrations
- Improved build times and security posture through dependency updates

## [1.1.3] - 2025-05-25

### Added for 1.1.3

- **Deno-based CI Automation**: Comprehensive test runner with feature flag combinations
  - Automatic detection of Cargo.toml features and dependency validation
  - Support for both Clippy linting and cargo test execution
  - Configurable test modes (library tests, doc tests, or both)
  - Automated testing across all valid feature flag combinations
- **Enhanced Audio Format Support**: Extended dictation API with comprehensive audio format testing
  - Added test assets for MP3, OGG, WAV, and RAW audio formats
  - Comprehensive test coverage for both buffered and streaming dictation
  - Proper PCM parameters for raw audio (8kHz, 8-bit, unsigned-integer)
  - Levenshtein distance validation for transcription accuracy

### Changed for 1.1.3

- **Feature Gating Improvements**: Proper conditional compilation for async/blocking functionality
  - Gated futures and streaming functionality behind async feature flag
  - Reduced binary size and compilation time for users with specific feature requirements
  - Fixed README example code blocks to prevent documentation test issues
- **Dictation API Enhancements**:
  - Refactored URL generation with dedicated `to_url()` method in `DictationQuery`
  - Added builder pattern methods for audio parameter configuration
  - Improved query parameter handling for raw audio configurations

### Fixed for 1.1.3

- **Raw Audio Content-Type**: Fixed raw audio Content-Type header format for proper Wit.ai API compatibility
- **Project Governance**:
  - Updated CODEOWNERS with correct GitHub username and improved pattern matching
  - Corrected security contact email in SECURITY.md

### Technical changes to 1.1.3

- Added builder pattern methods for audio configuration:
  - `with_raw_encoding()` for raw audio encoding type specification
  - `with_bits()` for audio bit depth configuration
  - `with_sample_rate()` for sample rate specification
  - `with_endian()` for endianness configuration
- Improved code organization by moving URL generation logic to dedicated methods
- Enhanced conditional compilation with proper feature gating
- Added comprehensive audio format test suite with real audio files

## [1.1.1] - 2025-05-25

### Added for 1.1.1

- **Dictation API Support**: Implemented POST `/dictation` endpoint for audio transcription
  - Support for streaming and blocking dictation requests
  - Audio source handling (buffered and streamed)
  - Multiple encoding format support (WAV, etc.)
  - Comprehensive test coverage with real audio file testing
- **Community Section**: Added Mastodon contact information to README for project updates and support

### Changed for 1.1.1

- Updated README.md to reflect current API implementation status
- Fixed dependency name in README from `wit_api` to `wit_owo`
- Updated API status indicators:
  - GET `/message`: Now marked as ✔️ (fully supported and tested)
  - POST `/dictation`: Now marked as ⚠️ (supported but being improved)

### Technical changes to 1.1.1

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

## [1.0.1] - Initial Release

### Added to 1.0.1

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

- **1.1.3**: Enhanced feature gating, comprehensive audio format support, and CI automation improvements
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
