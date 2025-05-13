use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug, Deserialize)]
pub struct WitError {
  pub code: String,
  pub message: String,
}

impl Display for WitError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error code: {}, message: {}", self.code, self.message)
  }
}

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("API error: {0}")]
  WitError(#[from] WitError),
  #[error("Url error: {0}")]
  UrlError(#[from] url::ParseError),
  #[error("Request error: {0}")]
  RequestError(#[from] reqwest::Error),
  #[error("Serialization error: {0}")]
  SerializationError(#[from] serde_json::Error),
}
