//! # Text-to-Speech Synthesis Models
//!
//! This module provides data structures for the Wit.ai text-to-speech synthesis API.
//! It includes types for specifying audio codecs, synthesis parameters, and request
//! validation.
//!
//! ## Key Components
//!
//! - [`SynthesizeCodec`] - Audio format options for synthesized speech
//! - [`SynthesizeQuery`] - Request parameters for text-to-speech synthesis
//!
//! ## Usage Example
//!
//! ```
//! # use wit_owo::model::synthesize::{SynthesizeQuery, SynthesizeCodec};
//! // Create a synthesis query with custom parameters
//! let query = SynthesizeQuery::new(
//!     "Hello, how are you today?".to_string(),
//!     "wit$Rebecca".to_string()
//! )
//! .with_style("formal".to_string())
//! .with_speed(120)  // 20% faster
//! .with_pitch(90);  // Slightly lower pitch
//!
//! // Choose an audio format
//! let codec = SynthesizeCodec::Mp3;
//! ```

use serde::Serialize;
use url::Url;

use crate::{error::ApiError, prelude::BASE_URL};

/// Audio codec options for text-to-speech synthesis.
///
/// This enum specifies the desired output format for synthesized audio.
/// Each variant corresponds to a specific MIME type that will be sent
/// in the `Accept` header of the synthesis request.
#[derive(Debug, Clone, Serialize)]
pub enum SynthesizeCodec {
  /// Raw PCM audio format (16-bit, mono, 16kHz).
  /// Serializes to "audio/pcm16" MIME type.
  #[serde(rename = "audio/pcm16")]
  Pcm,
  /// MP3 compressed audio format.
  /// Serializes to "audio/mpeg" MIME type.
  #[serde(rename = "audio/mpeg")]
  Mp3,
  /// WAV container format with PCM audio.
  /// Serializes to "audio/wav" MIME type.
  #[serde(rename = "audio/wav")]
  Wav,
}

impl std::fmt::Display for SynthesizeCodec {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let codec_str = match self {
      SynthesizeCodec::Pcm => "audio/pcm16",
      SynthesizeCodec::Mp3 => "audio/mpeg",
      SynthesizeCodec::Wav => "audio/wav",
    };
    write!(f, "{codec_str}")
  }
}

/// Parameters for text-to-speech synthesis requests.
///
/// This struct contains all the configurable options for synthesizing speech
/// from text using the Wit.ai API. It supports customization of voice characteristics
/// including style, speed, and pitch.
#[derive(Debug, Default, Serialize)]
pub struct SynthesizeQuery {
  /// The text to be synthesized into speech.
  /// Must not be empty or longer than `MAX_TEXT_LENGTH` characters.
  pub q: String,
  /// The voice identifier to use for synthesis.
  /// Must be a valid voice name from the available voices list.
  pub voice: String,
  /// Optional style modifier for the voice.
  /// Available styles depend on the selected voice (e.g., "default", "soft", "formal").
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<String>,
  /// Optional speed modifier for speech rate.
  /// Valid range is 10-400, where 100 is normal speed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub speed: Option<u16>,
  /// Optional pitch modifier for voice pitch.
  /// Valid range is 25-400, where 100 is normal pitch.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pitch: Option<i16>,
}

impl SynthesizeQuery {
  /// Creates a new synthesis query with the specified text and voice.
  ///
  /// # Arguments
  ///
  /// * `q` - The text to be synthesized into speech.
  /// * `voice` - The voice identifier to use for synthesis.
  ///
  /// # Panics
  ///
  /// Panics if the query is invalid (empty text, empty voice, or invalid parameters).
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::synthesize::SynthesizeQuery;
  /// let query = SynthesizeQuery::new(
  ///     "Hello, world!".to_string(),
  ///     "wit$Rebecca".to_string()
  /// );
  /// ```
  pub fn new(q: String, voice: String) -> Self {
    let ret = Self {
      q,
      voice,
      ..Default::default()
    };

    if !is_valid_query(&ret) {
      panic!("Thy synthesize request is not valid.");
    }

    ret
  }

  /// Calculates the actual character length of the text, excluding SSML tags.
  ///
  /// This method counts only the characters that will be spoken, ignoring
  /// any SSML markup tags enclosed in angle brackets.
  ///
  /// # Returns
  ///
  /// The number of speakable characters in the text.
  #[allow(dead_code)]
  pub(crate) fn len(&self) -> usize {
    let mut count = 0;
    let mut in_tag = false;
    for c in self.q.chars() {
      if c == '<' {
        in_tag = true;
      } else if c == '>' {
        in_tag = false;
      } else if !in_tag {
        count += 1;
      }
    }

    count
  }

  /// Sets the style for the synthesis query.
  ///
  /// # Arguments
  ///
  /// * `style` - The style to apply (must be supported by the selected voice).
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::synthesize::SynthesizeQuery;
  /// let query = SynthesizeQuery::new("Hello".to_string(), "wit$Rebecca".to_string())
  ///     .with_style("formal".to_string());
  /// ```
  pub fn with_style(mut self, style: String) -> Self {
    self.style = Some(style);
    self
  }

  /// Sets the speech speed for the synthesis query.
  ///
  /// # Arguments
  ///
  /// * `speed` - The speed value (10-400, where 100 is normal).
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::synthesize::SynthesizeQuery;
  /// let query = SynthesizeQuery::new("Hello".to_string(), "wit$Rebecca".to_string())
  ///     .with_speed(150); // 50% faster than normal
  /// ```
  pub fn with_speed(mut self, speed: u16) -> Self {
    self.speed = Some(speed);
    self
  }

  /// Sets the voice pitch for the synthesis query.
  ///
  /// # Arguments
  ///
  /// * `pitch` - The pitch value (25-400, where 100 is normal).
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::synthesize::SynthesizeQuery;
  /// let query = SynthesizeQuery::new("Hello".to_string(), "wit$Rebecca".to_string())
  ///     .with_pitch(80); // Lower pitch than normal
  /// ```
  pub fn with_pitch(mut self, pitch: i16) -> Self {
    self.pitch = Some(pitch);
    self
  }

  /// Converts the synthesis query into a URL for the API request.
  ///
  /// # Returns
  ///
  /// A `Result` containing the constructed URL or an error if URL parsing fails.
  pub(crate) fn to_url(&self) -> Result<Url, ApiError> {
    Url::parse(&format!("{BASE_URL}synthesize")).map_err(|e| e.into())
  }
}

/// Validates the `SynthesizeQuery` to ensure it meets the requirements for synthesis.
///
/// # Arguments
///
/// * `query` - The synthesis query to validate.
///
/// # Returns
///
/// `true` if the query is valid, `false` otherwise.
///
/// # Validation Rules
///
/// - Text (`q`) must not be empty
/// - Voice identifier must not be empty
/// - Speed, if specified, must be between 10 and 400 (inclusive)
/// - Pitch, if specified, must be between 25 and 400 (inclusive)
pub(crate) fn is_valid_query(query: &SynthesizeQuery) -> bool {
  if query.q.is_empty() || query.voice.is_empty() {
    return false;
  }
  match query.speed {
    Some(speed) if !(10..=400).contains(&speed) => return false,
    _ => {}
  }
  match query.pitch {
    Some(pitch) if !(25..=400).contains(&pitch) => return false,
    _ => {}
  }

  true
}
