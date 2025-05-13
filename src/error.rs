//! Error types used across the Wit OWO client.

use serde::Deserialize;
use std::fmt::Display;
use thiserror::Error;

/// Represents a structured error returned by the Wit API.
///
/// This error is deserializable from the JSON payload returned by Wit.
/// It contains:
/// - `code`: a machine‐readable error code.
/// - `message`: a human‐readable description of what went wrong.
///
/// # Example
///
/// ```ignore
/// // Suppose the API returns:
/// // { "code": "invalid_request", "message": "Missing required parameter" }
/// let json = r#"{ "code": "invalid_request", "message": "Missing required parameter" }"#;
/// let wit_err: WitError = serde_json::from_str(json)?;
/// println!("{}", wit_err); // prints: Error code: invalid_request, message: Missing required parameter
/// ```
#[derive(Error, Debug, Deserialize)]
pub struct WitError {
  /// Machine-readable error code.
  pub code: String,
  /// Human-readable error message.
  pub message: String,
}

impl Display for WitError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error code: {}, message: {}", self.code, self.message)
  }
}

/// The unified error type for all operations in the Wit OWO client.
///
/// This enum aggregates:
/// - Errors from the Wit API (`WitError`).
/// - URL parsing failures (`url::ParseError`).
/// - HTTP request failures (`reqwest::Error`).
/// - JSON serialization/deserialization errors (`serde_json::Error`).
///
/// You can use the `?` operator to automatically convert into `ApiError` when
/// calling functions that return it.
#[derive(Error, Debug)]
pub enum ApiError {
  /// An error returned by the Wit API.
  #[error("API error: {0}")]
  WitError(#[from] WitError),

  /// Failed to parse a URL.
  #[error("URL error: {0}")]
  UrlError(#[from] url::ParseError),

  /// HTTP request failure.
  #[error("Request error: {0}")]
  RequestError(#[from] reqwest::Error),

  /// JSON (de)serialization failure.
  #[error("Serialization error: {0}")]
  SerializationError(#[from] serde_json::Error),
}
