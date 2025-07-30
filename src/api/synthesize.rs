//! # Wit.ai Text-to-Speech Synthesis API
//!
//! This module provides a comprehensive interface for interacting with the Wit.ai Synthesis API endpoint.
//! The Synthesis API enables you to convert text into natural-sounding speech using various voices
//! and customization options.
//!
//! ## Overview
//!
//! The Wit.ai Synthesis API allows you to:
//! - Convert text to speech using multiple voice options
//! - Choose from different audio formats (MP3, WAV, PCM)
//! - Customize voice characteristics (style, speed, pitch)
//! - Stream audio data for real-time playback
//! - Support for SSML (Speech Synthesis Markup Language) tags
//!
//! ## Quick Start
//!
//! ### Basic Usage (Async with Streaming)
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     synthesize::{SynthesizeQuery, SynthesizeCodec},
//! };
//! use futures::StreamExt;
//! # use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new(&token);
//!
//!     // Create a synthesis request
//!     let query = SynthesizeQuery::new(
//!         "Hello, welcome to Wit.ai text-to-speech!".to_string(),
//!         "wit$Rebecca".to_string()
//!     );
//!
//!     // Stream MP3 audio data
//!     let mut stream = Box::pin(client.post_synthesize(&query, &SynthesizeCodec::Mp3).await);
//!     let mut audio_data = Vec::new();
//!
//!     while let Some(chunk) = stream.next().await {
//!         let bytes = chunk?;
//!         audio_data.extend_from_slice(&bytes);
//!     }
//!
//!     // Save or play the audio_data...
//!     println!("Generated {} bytes of audio", audio_data.len());
//!
//!     Ok(())
//! }
//!
//! # main().unwrap();
//! # }
//! ```
//!
//! ### Basic Usage (Blocking)
//!
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     synthesize::{SynthesizeQuery, SynthesizeCodec},
//! };
//! # use std::env;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new(&token);
//!
//!     let query = SynthesizeQuery::new(
//!         "Hello from the blocking API!".to_string(),
//!         "wit$Rebecca".to_string()
//!     );
//!
//!     // Get all audio data at once
//!     let audio_data = client.post_blocking_synthesize(&query, &SynthesizeCodec::Wav)?;
//!
//!     // Save to file
//!     std::fs::write("output.wav", &audio_data)?;
//!     println!("Saved {} bytes to output.wav", audio_data.len());
//!
//!     Ok(())
//! }
//!
//! # main().unwrap();
//! # }
//! ```
//!
//! ## Advanced Features
//!
//! ### Voice Customization
//!
//! Customize the voice output with style, speed, and pitch parameters:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     synthesize::{SynthesizeQuery, SynthesizeCodec},
//! };
//! # use std::env;
//! # use futures::StreamExt;
//!
//! async fn customize_voice(client: &WitClient) -> Result<(), Box<dyn std::error::Error>> {
//!     let query = SynthesizeQuery::new(
//!         "This is a customized voice".to_string(),
//!         "wit$Rebecca".to_string()
//!     )
//!     .with_style("formal".to_string())  // Use formal speaking style
//!     .with_speed(120)                   // 20% faster than normal
//!     .with_pitch(90);                   // Slightly lower pitch
//!
//!     let mut stream = Box::pin(client.post_synthesize(&query, &SynthesizeCodec::Mp3).await);
//!     
//!     // Process the audio stream...
//!     # while let Some(_) = stream.next().await {}
//!
//!     Ok(())
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! #     let client = WitClient::new("test_token");
//! #     let _ = customize_voice(&client).await;
//! # });
//! # }
//! ```
//!
//! ### Audio Format Selection
//!
//! Choose the appropriate audio format for your use case:
//!
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     synthesize::{SynthesizeQuery, SynthesizeCodec},
//! };
//! # use std::env;
//!
//! fn different_formats(client: &WitClient) -> Result<(), Box<dyn std::error::Error>> {
//!     let query = SynthesizeQuery::new(
//!         "Testing different audio formats".to_string(),
//!         "wit$Rebecca".to_string()
//!     );
//!
//!     // MP3 - Compressed, good for web/mobile
//!     let mp3_data = client.post_blocking_synthesize(&query, &SynthesizeCodec::Mp3)?;
//!     
//!     // WAV - Uncompressed, includes headers
//!     let wav_data = client.post_blocking_synthesize(&query, &SynthesizeCodec::Wav)?;
//!     
//!     // PCM - Raw audio data (16-bit, 16kHz, mono)
//!     let pcm_data = client.post_blocking_synthesize(&query, &SynthesizeCodec::Pcm)?;
//!
//!     println!("MP3 size: {} bytes", mp3_data.len());
//!     println!("WAV size: {} bytes", wav_data.len());
//!     println!("PCM size: {} bytes", pcm_data.len());
//!
//!     Ok(())
//! }
//!
//! # dotenvy::dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//! # let client = WitClient::new(&token);
//! # let _ = different_formats(&client);
//! # }
//! ```
//!
//! ### Working with Available Voices
//!
//! First, retrieve available voices, then use them for synthesis:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     synthesize::{SynthesizeQuery, SynthesizeCodec},
//! };
//! use futures::StreamExt;
//! # use std::env;
//!
//! async fn use_available_voices(client: &WitClient) -> Result<(), Box<dyn std::error::Error>> {
//!     // Get all available voices
//!     let voices = client.get_voices().await?;
//!     
//!     // Find a voice with specific characteristics
//!     let female_voice = voices.iter()
//!         .find(|v| v.gender == "female" && v.locale == "en_US")
//!         .ok_or("No suitable voice found")?;
//!
//!     // Use the voice for synthesis
//!     let query = SynthesizeQuery::new(
//!         "Using a dynamically selected voice".to_string(),
//!         female_voice.name.clone()
//!     );
//!
//!     // Check if voice supports specific style
//!     if female_voice.styles.contains(&"soft".to_string()) {
//!         let query = query.with_style("soft".to_string());
//!         let mut stream = Box::pin(client.post_synthesize(&query, &SynthesizeCodec::Mp3).await);
//!         // Process audio...
//!         # while let Some(_) = stream.next().await {}
//!     }
//!
//!     Ok(())
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! #     let client = WitClient::new("test_token");
//! #     let _ = use_available_voices(&client).await;
//! # });
//! # }
//! ```
//!
//! ### Error Handling
//!
//! The API provides detailed error information for various failure scenarios:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::{
//!     model::{
//!         client::WitClient,
//!         synthesize::{SynthesizeQuery, SynthesizeCodec},
//!     },
//!     error::ApiError,
//! };
//! use futures::StreamExt;
//! # use std::env;
//!
//! async fn handle_synthesis_errors() {
//!     # dotenvy::dotenv().ok();
//!     # let _token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new("INVALID_TOKEN");
//!
//!     let query = SynthesizeQuery::new(
//!         "Test message".to_string(),
//!         "invalid_voice".to_string()
//!     );
//!
//!     let mut stream = Box::pin(client.post_synthesize(&query, &SynthesizeCodec::Mp3).await);
//!     
//!     match stream.next().await {
//!         Some(Ok(chunk)) => {
//!             println!("Received {} bytes", chunk.len());
//!         }
//!         Some(Err(ApiError::WitError(wit_error))) => {
//!             println!("Wit.ai error: {}", wit_error.error);
//!             println!("Error code: {}", wit_error.code);
//!         }
//!         Some(Err(e)) => {
//!             println!("Other error: {}", e);
//!         }
//!         None => {
//!             println!("Stream ended unexpectedly");
//!         }
//!     }
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(handle_synthesis_errors());
//! # }
//! ```
//!
//! ## Best Practices
//!
//! ### 1. **Voice Selection**
//! Always verify that a voice supports your desired features before using it:
//!
//! ```no_run
//! # use wit_owo::model::voice::Voice;
//! fn select_voice(voices: &[Voice]) -> Option<&Voice> {
//!     voices.iter().find(|v| {
//!         v.supported_features.contains(&"style".to_string()) &&
//!         v.supported_features.contains(&"pitch".to_string()) &&
//!         v.locale == "en_US"
//!     })
//! }
//! ```
//!
//! ### 2. **Text Length Considerations**
//! Keep synthesis requests within reasonable limits for optimal performance:
//!
//! ```no_run
//! # use wit_owo::model::synthesize::SynthesizeQuery;
//! fn chunk_text(text: &str, max_chunk_size: usize) -> Vec<String> {
//!     // Split text into manageable chunks at sentence boundaries
//!     text.split(". ")
//!         .map(|s| s.to_string() + ".")
//!         .collect()
//! }
//! ```
//!
//! ### 3. **Stream Processing**
//! For async operations, process audio data as it arrives rather than buffering everything:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! # use wit_owo::model::{client::WitClient, synthesize::{SynthesizeQuery, SynthesizeCodec}};
//! # use futures::StreamExt;
//! # async fn example(client: &WitClient, query: &SynthesizeQuery) -> Result<(), Box<dyn std::error::Error>> {
//! let mut stream = Box::pin(client.post_synthesize(&query, &SynthesizeCodec::Mp3).await);
//!
//! // Process chunks as they arrive
//! while let Some(chunk) = stream.next().await {
//!     let audio_chunk = chunk?;
//!     // Send to audio player, write to file incrementally, etc.
//!     # let _ = audio_chunk;
//! }
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### 4. **Parameter Validation**
//! The library validates parameters, but understanding the limits helps:
//!
//! - **Speed**: 10-400 (100 = normal)
//! - **Pitch**: 25-400 (100 = normal)
//! - **Text**: Must not be empty
//! - **Voice**: Must be a valid voice identifier
//!
//! ## Feature Flags
//!
//! This module's functionality depends on cargo features:
//!
//! - **`async`** (default): Enables `post_synthesize()` for streaming operations
//! - **`blocking`**: Enables `post_blocking_synthesize()` for synchronous operations
//!
//! ## Audio Format Details
//!
//! ### PCM (Raw Audio)
//! - 16-bit signed integers
//! - 16kHz sample rate
//! - Mono channel
//! - No headers or metadata
//!
//! ### WAV
//! - Standard WAV container
//! - Contains PCM data with proper headers
//! - Compatible with most audio players
//!
//! ### MP3
//! - Compressed format
//! - Smaller file size
//! - Widely supported
//!
//! ## Examples
//!
//! See the `tests` module in this file for comprehensive examples of all functionality.
//!
//! ## Related Documentation
//!
//! - [`WitClient`]: The main client struct
//! - [`SynthesizeQuery`]: Request parameters for synthesis
//! - [`SynthesizeCodec`]: Available audio formats
//! - [`Voice`]: Voice information and capabilities
//! - [`ApiError`]: Error types returned by the API
//!
//! [`WitClient`]: crate::model::client::WitClient
//! [`SynthesizeQuery`]: crate::model::synthesize::SynthesizeQuery
//! [`SynthesizeCodec`]: crate::model::synthesize::SynthesizeCodec
//! [`Voice`]: crate::model::voice::Voice
//! [`ApiError`]: crate::error::ApiError

use crate::error::WitError;
use crate::{
  error::ApiError,
  model::synthesize::{SynthesizeCodec, SynthesizeQuery},
  prelude::WitClient,
};
use bytes::Bytes;
#[cfg(feature = "async")]
use futures::{Stream, StreamExt as _};

impl WitClient {
  /// Asynchronously synthesizes speech from text using the Wit.ai API.
  ///
  /// This method sends a `POST` request to the `/synthesize` endpoint with the
  /// specified text and voice parameters, and returns a stream of audio data.
  ///
  /// # Arguments
  ///
  /// * `tts` - A `SynthesizeQuery` containing the text, voice, and other synthesis options.
  /// * `codec` - The desired audio codec for the output stream.
  ///
  /// # Returns
  ///
  /// A stream of `Result<Bytes, ApiError>`, where each `Ok` variant contains a chunk of
  /// audio data.
  ///
  /// # Errors
  ///
  /// This method will return an error if:
  /// * The URL parsing fails.
  /// * The HTTP request fails to send.
  /// * The API returns a non-success status code.
  #[cfg(feature = "async")]
  pub async fn post_synthesize(
    &self,
    tts: &SynthesizeQuery,
    codec: &SynthesizeCodec,
  ) -> impl Stream<Item = Result<Bytes, ApiError>> {
    use async_stream::try_stream;

    try_stream! {
      let url = tts.to_url()?;
      let request = self
         .prepare_post_request(url)
         .header("Content-Type", "application/json")
         .header("Accept", codec.to_string())
         .json(tts);

      let response = request.send().await?;

      if !response.status().is_success() {
        let error_text = response.text().await?;
        let wit_error: WitError = serde_json::from_str(&error_text)
           .unwrap_or_else(|_| WitError {
             error: format!("Failed to synthesize speech: {error_text}"),
             code: "synthesis_failed".to_string(),
           });
        Err(ApiError::WitError(wit_error))?;
        return ;
      }

      let mut stream = response.bytes_stream();
      while let Some(chunk) = stream.next().await {
        yield chunk?;
      }
    }
  }

  /// Synchronously synthesizes speech from text using the Wit.ai API.
  ///
  /// This method sends a `POST` request to the `/synthesize` endpoint with the
  /// specified text and voice parameters, and returns the entire audio data at once.
  ///
  /// # Arguments
  ///
  /// * `tts` - A `SynthesizeQuery` containing the text, voice, and other synthesis options.
  /// * `codec` - The desired audio codec for the output data.
  ///
  /// # Returns
  ///
  /// A `Result<Bytes, ApiError>`, where the `Ok` variant contains the entire audio data.
  ///
  /// # Errors
  ///
  /// This method will return an error if:
  /// * The URL parsing fails.
  /// * The HTTP request fails to send.
  /// * The API returns a non-success status code.
  #[cfg(feature = "blocking")]
  pub fn post_blocking_synthesize(
    &self,
    tts: &SynthesizeQuery,
    codec: &SynthesizeCodec,
  ) -> Result<Bytes, ApiError> {
    let url = tts.to_url()?;
    let request = self
      .prepare_post_blocking(url)
      .header("Content-Type", "application/json")
      .header("Accept", codec.to_string())
      .json(tts);

    let response = request.send()?;

    if !response.status().is_success() {
      let error_text = response.text()?;
      let wit_error: WitError = serde_json::from_str(&error_text).unwrap_or_else(|_| WitError {
        error: format!("Failed to synthesize speech: {error_text}"),
        code: "synthesis_failed".to_string(),
      });
      return Err(ApiError::WitError(wit_error))?;
    }

    Ok(response.bytes()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::{
    client::WitClient,
    synthesize::{SynthesizeCodec, SynthesizeQuery},
    voice::Voice,
  };
  use dotenvy::dotenv;
  use std::env;

  #[cfg(feature = "async")]
  use futures::stream::StreamExt;

  /// Helper function to test async synthesis with buffered audio data
  #[cfg(feature = "async")]
  async fn test_async_synthesize(text: &str, voice: &str, codec: SynthesizeCodec, test_name: &str) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let tts = SynthesizeQuery::new(text.to_string(), voice.to_string());
    let mut stream = Box::pin(client.post_synthesize(&tts, &codec).await);
    let mut audio_data = Vec::new();

    while let Some(result) = stream.next().await {
      let chunk = result.unwrap_or_else(|e| panic!("Synthesis failed for {test_name}: {e:?}"));
      audio_data.extend_from_slice(&chunk);
    }

    assert!(
      !audio_data.is_empty(),
      "Should have received audio data for {test_name}"
    );
    assert!(
      audio_data.len() > 1000,
      "Audio data too small for {test_name}"
    );
  }

  /// Helper function to test blocking synthesis
  #[cfg(feature = "blocking")]
  fn test_blocking_synthesize(text: &str, voice: &str, codec: SynthesizeCodec, test_name: &str) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let tts = SynthesizeQuery::new(text.to_string(), voice.to_string());
    let audio_data = client
      .post_blocking_synthesize(&tts, &codec)
      .unwrap_or_else(|_| panic!("Synthesis failed for {test_name}"));

    assert!(
      !audio_data.is_empty(),
      "Should have received audio data for {test_name}"
    );
    assert!(
      audio_data.len() > 1000,
      "Audio data too small for {test_name}",
    );
  }

  /// Helper to get a valid voice for testing
  #[cfg(feature = "async")]
  async fn get_test_voice(client: &WitClient) -> Voice {
    let voices = client.get_voices().await.expect("Failed to get voices");
    voices.first().cloned().expect("No voices available")
  }

  /// Helper to get a valid voice for blocking tests
  #[cfg(feature = "blocking")]
  fn get_test_voice_blocking(client: &WitClient) -> Voice {
    let voices = client.get_voices_blocking().expect("Failed to get voices");
    voices.first().cloned().expect("No voices available")
  }

  // MP3 Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_synthesize_mp3() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice(&client).await;

    test_async_synthesize(
      "The quick brown fox jumps over the lazy dog",
      &voice.name,
      SynthesizeCodec::Mp3,
      "MP3",
    )
    .await;
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_synthesize_mp3() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice_blocking(&client);

    test_blocking_synthesize(
      "The quick brown fox jumps over the lazy dog",
      &voice.name,
      SynthesizeCodec::Mp3,
      "MP3",
    );
  }

  // WAV Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_synthesize_wav() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice(&client).await;

    test_async_synthesize(
      "How razorback-jumping frogs can level six piqued gymnasts",
      &voice.name,
      SynthesizeCodec::Wav,
      "WAV",
    )
    .await;
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_synthesize_wav() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice_blocking(&client);

    test_blocking_synthesize(
      "How razorback-jumping frogs can level six piqued gymnasts",
      &voice.name,
      SynthesizeCodec::Wav,
      "WAV",
    );
  }

  // RAW Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_synthesize_raw() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice(&client).await;

    test_async_synthesize(
      "Crazy Fredrick bought many very exquisite opal jewels",
      &voice.name,
      SynthesizeCodec::Pcm,
      "RAW",
    )
    .await;
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_synthesize_raw() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice_blocking(&client);

    test_blocking_synthesize(
      "Crazy Fredrick bought many very exquisite opal jewels",
      &voice.name,
      SynthesizeCodec::Pcm,
      "RAW",
    );
  }

  // Test with style parameter
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_synthesize_with_style() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);
    let voice = get_test_voice(&client).await;

    let mut tts = SynthesizeQuery::new(
      "This is a formal announcement".to_string(),
      voice.name.clone(),
    );
    tts.style = Some("formal".to_string());

    let codec = SynthesizeCodec::Mp3;
    let mut stream = Box::pin(client.post_synthesize(&tts, &codec).await);
    let mut audio_data = Vec::new();

    while let Some(result) = stream.next().await {
      let chunk = result.expect("Synthesis failed with style parameter");
      audio_data.extend_from_slice(&chunk);
    }

    assert!(!audio_data.is_empty(), "Should have received audio data");
  }

  // Test error handling
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_synthesize_invalid_voice() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let tts = SynthesizeQuery::new(
      "This should fail".to_string(),
      "invalid_voice_123".to_string(),
    );
    let codec = SynthesizeCodec::Mp3;
    let mut stream = Box::pin(client.post_synthesize(&tts, &codec).await);

    let result = stream.next().await;
    assert!(result.is_some());
    let err = result.unwrap();
    assert!(matches!(err, Err(ApiError::WitError(_))));
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_synthesize_invalid_voice() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let tts = SynthesizeQuery::new(
      "This should fail".to_string(),
      "invalid_voice_123".to_string(),
    );
    let codec = SynthesizeCodec::Mp3;
    let result = client.post_blocking_synthesize(&tts, &codec);

    assert!(matches!(result, Err(ApiError::WitError(_))));
  }
}
