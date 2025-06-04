//! # Wit.ai Dictation API
//!
//! This module provides a comprehensive interface for performing speech-to-text transcription
//! using the Wit.ai dictation API. The dictation API converts audio data into text with
//! support for both streaming and batch processing modes.
//!
//! ## Overview
//!
//! The dictation API supports:
//! - **Multiple audio formats**: WAV, MP3, OGG, μ-law, and raw PCM audio
//! - **Streaming transcription**: Real-time processing of audio streams
//! - **Batch transcription**: Processing of complete audio files
//! - **Asynchronous and blocking modes**: Choose based on your application needs
//! - **Partial and final results**: Get intermediate results during long transcriptions
//!
//! ## Supported Audio Formats
//!
//! | Format | Description | Content-Type |
//! |--------|-------------|--------------|
//! | WAV    | Waveform Audio File Format | `audio/wav` |
//! | MP3    | MPEG Audio Layer III | `audio/mpeg3` |
//! | OGG    | Ogg Vorbis audio format | `audio/ogg` |
//! | μ-law  | µ-law algorithm (telephony) | `audio/ulaw` |
//! | Raw    | Raw PCM audio data | `audio/raw;encoding=...` |
//!
//! ## Getting Started
//!
//! ### Prerequisites
//!
//! 1. **Wit.ai Account**: Sign up at [wit.ai](https://wit.ai) and create an app
//! 2. **API Token**: Get your server access token from your app settings
//! 3. **Audio Data**: Prepare your audio files or streams
//!
//! ### Basic Setup
//!
//! ```rust
//! use wit_owo::prelude::*;
//!
//! // Initialize the client with your Wit.ai token
//! let client = WitClient::new("your_wit_ai_token_here");
//! ```
//!
//! ## Quick Start Examples
//!
//! ### Example 1: Simple File Transcription (Blocking)
//!
//! ```rust
//! # #[cfg(feature = "blocking")]
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use std::fs::File;
//! use std::io::Read;
//! use bytes::Bytes;
//!
//! // Initialize client
//! let client = WitClient::new("your_token_here");
//!
//! // Read audio file
//! let mut file = File::open("audio.wav")?;
//! let mut audio_data = Vec::new();
//! file.read_to_end(&mut audio_data)?;
//!
//! // Create dictation query
//! let query = DictationQuery::new(
//!     Encoding::Wav,
//!     AudioSource::Buffered(Bytes::from(audio_data))
//! );
//!
//! // Perform transcription
//! let results = client.post_blocking_dictation(query)?;
//!
//! // Process results
//! for dictation in results {
//!     println!("Transcription: {}", dictation.text);
//!     println!("Confidence: {:.2}", dictation.speech.confidence);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Example 2: Streaming Transcription (Async)
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use futures::stream::StreamExt;
//! use bytes::Bytes;
//!
//! let client = WitClient::new("your_token_here");
//!
//! // Load audio data
//! let audio_data = std::fs::read("audio.wav")?;
//! let query = DictationQuery::new(
//!     Encoding::Wav,
//!     AudioSource::Buffered(Bytes::from(audio_data))
//! );
//!
//! // Get streaming results
//! let mut stream = Box::pin(client.post_dictation(query).await);
//!
//! while let Some(result) = stream.next().await {
//!     match result {
//!         Ok(dictation) => {
//!             match dictation.speech_type {
//!                 SpeechType::PartialTranscription => {
//!                     println!("Partial: {}", dictation.text);
//!                 }
//!                 SpeechType::FinalTranscription => {
//!                     println!("Final: {}", dictation.text);
//!                 }
//!             }
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Example 3: Real-time Audio Stream Processing
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use futures::stream::{self, StreamExt};
//! use bytes::Bytes;
//!
//! let client = WitClient::new("your_token_here");
//!
//! // Create a stream of audio chunks (e.g., from microphone)
//! let audio_data = std::fs::read("audio.wav")?;
//! let chunk_size = 1024;
//!
//! let audio_stream = stream::iter(
//!     audio_data
//!         .chunks(chunk_size)
//!         .map(|chunk| Ok::<Bytes, reqwest::Error>(Bytes::copy_from_slice(chunk)))
//!         .collect::<Vec<_>>()
//! );
//!
//! let query = DictationQuery::new(
//!     Encoding::Wav,
//!     AudioSource::Stream(Box::pin(audio_stream))
//! );
//!
//! let mut stream = Box::pin(client.post_dictation(query).await);
//!
//! while let Some(result) = stream.next().await {
//!     match result {
//!         Ok(dictation) => println!("Live: {}", dictation.text),
//!         Err(e) => eprintln!("Stream error: {}", e),
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Example 4: Raw Audio Processing
//!
//! For raw audio data, you need to specify additional parameters:
//!
//! ```rust
//! # #[cfg(feature = "blocking")]
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//!
//! let client = WitClient::new("your_token_here");
//!
//! // Raw audio data (8kHz, 8-bit, mono, unsigned-integer)
//! let raw_audio = std::fs::read("audio.raw")?;
//!
//! let query = DictationQuery::new(
//!     Encoding::Raw,
//!     AudioSource::Buffered(Bytes::from(raw_audio))
//! )
//! .with_raw_encoding("unsigned-integer".to_string())
//! .with_bits(8)
//! .with_sample_rate(8000)
//! .with_endian(true); // little-endian
//!
//! let results = client.post_blocking_dictation(query)?;
//!
//! for dictation in results {
//!     println!("Raw audio transcription: {}", dictation.text);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Audio Format Guidelines
//!
//! ### Recommended Settings
//!
//! For best transcription quality:
//!
//! - **Sample Rate**: 16kHz or higher (8kHz minimum)
//! - **Bit Depth**: 16-bit or higher
//! - **Channels**: Mono preferred, stereo supported
//! - **Format**: WAV or FLAC for highest quality, MP3 for smaller files
//!
//! ### Format-Specific Notes
//!
//! #### WAV Files
//! - Most reliable format
//! - Supports various bit depths and sample rates
//! - No compression artifacts
//!
//! #### MP3 Files
//! - Good compression ratio
//! - Widely supported
//! - Some quality loss due to compression
//!
//! #### Raw PCM Audio
//! - Requires explicit format specification
//! - Highest quality when properly configured
//! - Most flexible but requires more setup
//!
//! ```rust
//! // Raw audio configuration example
//! # fn example() {
//! # use wit_owo::prelude::*;
//! # let audio_source = AudioSource::Buffered(bytes::Bytes::new());
//! let query = DictationQuery::new(Encoding::Raw, audio_source)
//!     .with_raw_encoding("signed-integer".to_string())  // or "unsigned-integer"
//!     .with_bits(16)                                    // 8, 16, 24, or 32
//!     .with_sample_rate(16000)                         // Hz
//!     .with_endian(true);                              // true=little, false=big
//! # }
//! ```
//!
//! ## Understanding Transcription Results
//!
//! ### Dictation Structure
//!
//! Each `Dictation` result contains:
//!
//! ```rust
//! # use wit_owo::prelude::*;
//! # fn example() {
//! # // These are just type demonstrations
//! # let _unused_speech = Speech { confidence: 0.0, tokens: vec![] };
//! # let _unused_text = String::new();
//! # let _unused_speech_type = SpeechType::PartialTranscription;
//! pub struct Dictation {
//!     pub speech: Speech,           // Detailed transcription info
//!     pub text: String,             // The transcribed text
//!     pub speech_type: SpeechType,  // Partial or Final
//! }
//!
//! pub struct Speech {
//!     pub confidence: f32,          // Confidence score (0.0-1.0)
//!     pub tokens: Vec<Token>,       // Individual word tokens
//! }
//!
//! pub struct Token {
//!     pub start: usize,             // Start position
//!     pub end: usize,               // End position
//!     pub token: String,            // The word/token text
//! }
//! # }
//! ```
//!
//! ### Result Types
//!
//! - **Partial Results**: Intermediate transcriptions that may change
//! - **Final Results**: Completed transcriptions that won't change
//!
//! ### Confidence Scoring
//!
//! - **0.0 - 0.5**: Low confidence, may need review
//! - **0.5 - 0.8**: Medium confidence, generally reliable
//! - **0.8 - 1.0**: High confidence, very reliable
//!
//! ## Error Handling
//!
//! ### Common Error Scenarios
//!
//! ```rust
//! # #[cfg(feature = "blocking")]
//! # async fn example() {
//! use wit_owo::prelude::*;
//! # let audio_source = AudioSource::Buffered(bytes::Bytes::new());
//!
//! let client = WitClient::new("your_token_here");
//! let query = DictationQuery::new(Encoding::Wav, audio_source);
//!
//! match client.post_blocking_dictation(query) {
//!     Ok(results) => {
//!         // Process successful results
//!         for dictation in results {
//!             println!("Success: {}", dictation.text);
//!         }
//!     }
//!     Err(ApiError::RequestError(e)) => {
//!         eprintln!("Network error: {}", e);
//!         // Handle network connectivity issues
//!     }
//!     Err(ApiError::WitError(wit_error)) => {
//!         eprintln!("Wit.ai API error: {}", wit_error);
//!         // Handle API-specific errors (invalid token, quota exceeded, etc.)
//!     }
//!     Err(ApiError::SerializationError(json_error)) => {
//!         eprintln!("JSON parsing error: {}", json_error);
//!         // Handle response parsing issues
//!     }
//!     Err(e) => {
//!         eprintln!("Other error: {}", e);
//!     }
//! }
//! # }
//! ```
//!
//! ## Performance Tips
//!
//! ### For Large Files
//! - Use streaming mode for files > 10MB
//! - Process in chunks of 1-5MB for optimal performance
//! - Consider audio compression before upload
//!
//! ### For Real-time Applications
//! - Use smaller chunk sizes (512-2048 bytes)
//! - Buffer partial results for smoother user experience
//! - Implement reconnection logic for network interruptions
//!
//! ### Memory Optimization
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # use wit_owo::prelude::*;
//! # fn create_audio_stream(file_path: &str) -> Result<AudioSource, Box<dyn std::error::Error>> {
//! #     let data = std::fs::read(file_path)?;
//! #     Ok(AudioSource::Buffered(bytes::Bytes::from(data)))
//! # }
//! // For large files, prefer streaming over buffering
//! let audio_stream = create_audio_stream("large_file.wav")?;
//! let query = DictationQuery::new(
//!     Encoding::Wav,
//!     audio_stream  // Uses less memory
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! The dictation API supports conditional compilation:
//!
//! - `async`: Enables asynchronous streaming functionality
//! - `blocking`: Enables synchronous blocking functionality
//!
//! Configure in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wit_owo = { version = "1.0", features = ["async", "blocking"] }
//! ```
//!
//! ## Troubleshooting
//!
//! ### Common Issues
//!
//! 1. **"Invalid audio format"**
//!    - Verify audio file isn't corrupted
//!    - Check if format matches specified encoding
//!    - Try converting to WAV format
//!
//! 2. **"Authentication failed"**
//!    - Verify your Wit.ai token is correct
//!    - Check token permissions in Wit.ai dashboard
//!    - Ensure token hasn't expired
//!
//! 3. **"No audio detected"**
//!    - Check audio file contains actual audio data
//!    - Verify volume levels aren't too low
//!    - Ensure audio duration is sufficient (>100ms)
//!
//! 4. **Poor transcription quality**
//!    - Use higher sample rates (16kHz+)
//!    - Reduce background noise
//!    - Ensure clear speech with good pronunciation
//!
//! ### Debug Mode
//!
//! Enable debug output to see request details:
//!
//! ```rust
//! // The library automatically prints debug information in debug builds
//! // Look for "Request" and "Received complete JSON" messages
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Audio Sources
//!
//! You can implement custom audio sources by converting your data to the appropriate format:
//!
//! ```rust
//! # fn example() {
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//! use futures::stream;
//!
//! # fn get_audio_from_somewhere() -> Vec<u8> { vec![] }
//! # let audio_chunks: Vec<Vec<u8>> = vec![];
//! // From a custom buffer
//! let custom_buffer: Vec<u8> = get_audio_from_somewhere();
//! let audio_source = AudioSource::Buffered(Bytes::from(custom_buffer));
//!
//! // From a custom stream
//! # #[cfg(feature = "async")]
//! # {
//! let custom_stream = stream::iter(
//!     audio_chunks.into_iter().map(|chunk| Ok(Bytes::from(chunk)))
//! );
//! let audio_source = AudioSource::Stream(Box::pin(custom_stream));
//! # }
//! # }
//! ```
//!
//! ### Integrating with Audio Libraries
//!
//! The dictation API works well with popular Rust audio libraries:
//!
//! ```rust
//! // With `rodio` for audio playback/recording
//! // With `cpal` for cross-platform audio I/O
//! // With `hound` for WAV file processing
//! ```
//!
//! For more examples and integration patterns, see the test cases in this module.

use crate::error::ApiError;
use crate::model::dictation::{Dictation, DictationQuery};
use crate::prelude::WitClient;
use crate::utils::json::extract_complete_json;
use crate::{error::WitError, prelude::BASE_URL};
use url::Url;

#[cfg(feature = "async")]
use futures::stream::{Stream, StreamExt};

impl WitClient {
  /// Performs speech-to-text dictation using the Wit.ai API.
  ///
  /// This method sends audio data to the Wit.ai dictation endpoint and returns a stream
  /// of partial and final transcription results as they become available.
  ///
  /// # Arguments
  ///
  /// * `params` - A `DictationQuery` containing the audio encoding format and audio data
  ///
  /// # Returns
  ///
  /// Returns a `Stream` that yields `Result<Dictation, ApiError>` items. Each item contains
  /// a transcription result, which may be partial (interim) or final depending on the
  /// response from the API.
  ///
  /// # Errors
  ///
  /// This method will return an error if:
  /// * The URL parsing fails
  /// * The HTTP request fails to send
  /// * The API returns a non-success status code
  /// * JSON deserialization of the response fails
  ///
  /// # Example
  ///
  /// ```no_run
  /// use wit_owo::prelude::*;
  /// use bytes::Bytes;
  /// use futures::stream::StreamExt;
  /// use std::fs::File;
  /// use std::io::Read;
  ///
  /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
  /// // Initialize the Wit.ai client with your token
  /// let client = WitClient::new("your_token_here");
  ///
  /// // Load audio data from a file (WAV format in this example)
  /// let mut file = File::open("path/to/audio.wav")?;
  /// let mut audio_bytes = Vec::new();
  /// file.read_to_end(&mut audio_bytes)?;
  /// let audio_data = Bytes::from(audio_bytes);
  ///
  /// // Create a dictation query with the appropriate encoding
  /// let params = DictationQuery::new(
  ///     Encoding::Wav,
  ///     AudioSource::Buffered(audio_data)
  /// );
  ///
  /// // Send the audio data to Wit.ai and process the streaming response
  /// let mut stream = Box::pin(client.post_dictation(params).await);
  ///
  /// // Iterate through the stream of transcription results
  /// while let Some(result) = stream.next().await {
  ///     match result {
  ///         Ok(dictation) => {
  ///             println!("Speech type: {:?}", dictation.speech_type);
  ///             println!("Transcription: {}", dictation.text);
  ///             println!("Confidence: {}", dictation.speech.confidence);
  ///         },
  ///         Err(e) => eprintln!("Error: {}", e)
  ///     }
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "async")]
  pub async fn post_dictation(
    &self,
    params: DictationQuery,
  ) -> impl Stream<Item = Result<Dictation, ApiError>> {
    use async_stream::try_stream;

    try_stream! {
      let content_type = params.to_string();
      let url = Url::parse(&format!("{BASE_URL}dictation"))?;

      let request = self
        .prepare_post_request(url)
        .header("Content-Type", content_type)
        .body(params.data);
      println!("Request {request:?}");

      let response = request.send().await?;

      if !response.status().is_success() {
        Err(serde_json::from_str::<WitError>(
          &response.text().await?,
        )?)?;
        return ;
      }

      let mut reader = response.bytes_stream();
      let mut buffer = String::new();

      while let Some(chunk) = reader.next().await {
        let chunk = chunk?;
        if chunk.is_empty() {
          continue;
        }

        // Convert chunk to string and append to buffer
        let chunk_str = String::from_utf8_lossy(&chunk);
        buffer.push_str(&chunk_str);

        // Process complete JSON objects from the buffer
        while let Some((json_str, remaining)) = extract_complete_json(&buffer) {
          // We print the JSON for debugging purposes
          println!("Received complete JSON: {json_str:?}");

          // Deserialize the complete JSON object
          let dictation: Dictation = serde_json::from_str(&json_str)?;
          yield dictation;

          // Update buffer with remaining data
          buffer = remaining;
        }
      }

    }
  }

  /// Performs blocking speech-to-text dictation using the Wit.ai API.
  ///
  /// This method sends audio data to the Wit.ai dictation endpoint and blocks until
  /// all transcription results are received. Unlike the async version which returns
  /// a stream, this method collects all results and returns them as a vector.
  ///
  /// # Arguments
  ///
  /// * `params` - A `DictationQuery` containing the audio encoding format and audio data
  ///
  /// # Returns
  ///
  /// Returns a `Result<Vec<Dictation>, ApiError>` containing all transcription results.
  /// Each `Dictation` item contains a transcription result, which may be partial (interim)
  /// or final depending on the response from the API.
  ///
  /// # Errors
  ///
  /// This method will return an error if:
  /// * The URL parsing fails
  /// * The HTTP request fails to send
  /// * The API returns a non-success status code
  /// * JSON deserialization of the response fails
  ///
  /// # Example
  ///
  /// ```no_run
  /// use wit_owo::prelude::*;
  /// use bytes::Bytes;
  /// use std::fs::File;
  /// use std::io::Read;
  ///
  /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
  /// // Initialize the Wit.ai client with your token
  /// let client = WitClient::new("your_token_here");
  ///
  /// // Load audio data from a file (WAV format in this example)
  /// let mut file = File::open("path/to/audio.wav")?;
  /// let mut audio_bytes = Vec::new();
  /// file.read_to_end(&mut audio_bytes)?;
  /// let audio_data = Bytes::from(audio_bytes);
  ///
  /// // Create a dictation query with the appropriate encoding
  /// let params = DictationQuery::new(
  ///     Encoding::Wav,
  ///     AudioSource::Buffered(audio_data)
  /// );
  ///
  /// // Send the audio data to Wit.ai and get all transcription results
  /// let results = client.post_blocking_dictation(params)?;
  ///
  /// // Process all transcription results
  /// for dictation in results {
  ///     println!("Speech type: {:?}", dictation.speech_type);
  ///     println!("Transcription: {}", dictation.text);
  ///     println!("Confidence: {}", dictation.speech.confidence);
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "blocking")]
  pub fn post_blocking_dictation(
    &self,
    params: DictationQuery,
  ) -> Result<Vec<Dictation>, ApiError> {
    use crate::error::WitError;

    let content_type = params.to_string();
    let url = Url::parse(&format!("{BASE_URL}dictation"))?;

    let request = self
      .prepare_post_blocking(url)
      .header("Content-Type", content_type)
      .body(params.data);

    println!("Request {request:?}");

    let response = request.send()?;

    if !response.status().is_success() {
      return Err(serde_json::from_str::<WitError>(&response.text()?)?.into());
    }

    let response_text = response.text()?;
    let mut buffer = response_text;
    let mut results = Vec::new();

    // Process complete JSON objects from the buffer
    while let Some((json_str, remaining)) = extract_complete_json(&buffer) {
      // We print the JSON for debugging purposes
      println!("Received complete JSON: {json_str:?}");

      // Deserialize the complete JSON object
      let dictation: Dictation = serde_json::from_str(&json_str)?;
      results.push(dictation);

      // Update buffer with remaining data
      buffer = remaining;
    }

    Ok(results)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::dictation::{AudioSource, Encoding};
  use crate::utils::tests::levenshtein_distance;
  use bytes::Bytes;
  use dotenvy::dotenv;

  #[cfg(feature = "async")]
  use futures::stream::StreamExt;
  use std::env;

  const EXPECTED_TEXT: &str = "the examination and testimony of the experts enabled the commission to conclude that five shots may have been fired";

  /// Helper function to test async dictation with buffered audio data
  #[cfg(feature = "async")]
  async fn test_async_dictation_buffered(
    encoding: Encoding,
    audio_data: Vec<u8>,
    format_name: &str,
  ) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let mut params = DictationQuery::new(encoding, AudioSource::Buffered(Bytes::from(audio_data)));

    if encoding == Encoding::Raw {
      // For RAW encoding, we need to specify additional parameters
      params = params
        .with_bits(8)
        .with_sample_rate(8000)
        .with_endian(true)
        .with_raw_encoding("unsigned-integer".to_string());
    }

    let mut stream = Box::pin(client.post_dictation(params).await);
    let mut received_results = false;
    let mut last_dictation: Option<Dictation> = None;

    while let Some(result) = stream.next().await {
      match result {
        Ok(dictation) => {
          assert!(
            !dictation.text.is_empty(),
            "Dictation text should not be empty for {format_name} format",
          );
          println!("{format_name} Transcription: {}", dictation.text);
          received_results = true;
          last_dictation = Some(dictation);
        }
        Err(e) => {
          panic!("Dictation failed with error for {format_name} format: {e:?}",);
        }
      }
    }

    assert!(
      received_results,
      "Should have received at least one dictation result for {format_name} format",
    );

    if let Some(dictation) = last_dictation {
      assert!(
        !dictation.text.is_empty(),
        "Last dictation text should not be empty for {format_name} format",
      );
      assert!(
        levenshtein_distance(dictation.text.to_ascii_lowercase().as_str(), EXPECTED_TEXT) < 5,
        "Last dictation text is not similar enough to expected text for {format_name} format",
      );
    } else {
      panic!("No dictation results were received for {format_name} format",);
    }
  }

  /// Helper function to test async dictation with streaming audio data
  #[cfg(feature = "async")]
  async fn test_async_dictation_streaming(
    encoding: Encoding,
    audio_data: Vec<u8>,
    format_name: &str,
  ) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let chunk_size = 1024;

    // Clone the data for the closure
    let data_clone = audio_data.clone();

    // Turn the static byte slice into a stream of `Bytes` chunks
    let audio_stream = futures::stream::iter(
      data_clone
        .chunks(chunk_size)
        .map(|chunk| Ok::<Bytes, reqwest::Error>(Bytes::copy_from_slice(chunk)))
        .collect::<Vec<_>>(),
    );

    // Build the dictation query using the streaming audio source
    let mut params = DictationQuery::new(encoding, AudioSource::Stream(Box::pin(audio_stream)));

    if encoding == Encoding::Raw {
      // For RAW encoding, we need to specify additional parameters
      params = params
        .with_bits(8)
        .with_sample_rate(8000)
        .with_endian(true)
        .with_raw_encoding("unsigned-integer".to_string());
    }

    // Send to Wit.ai and collect the streaming results
    let mut stream = Box::pin(client.post_dictation(params).await);
    let mut received = false;
    let mut last_dictation = None;

    while let Some(item) = stream.next().await {
      let dict = item
        .unwrap_or_else(|e| panic!("streaming dictation failed for {format_name} format: {e:?}",));
      assert!(
        !dict.text.is_empty(),
        "Dictation text should not be empty for {format_name} format"
      );
      received = true;
      last_dictation = Some(dict);
    }

    assert!(
      received,
      "Should have received at least one dictation result for {format_name} format"
    );

    let final_dict = last_dictation
      .unwrap_or_else(|| panic!("No dictation results were received for {format_name} format",));
    assert!(
      levenshtein_distance(final_dict.text.to_ascii_lowercase().as_str(), EXPECTED_TEXT) < 5,
      "Last dictation text is not similar enough to expected text for {format_name} format"
    );
  }

  // MP3 Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_mp3_buffered() {
    test_async_dictation_buffered(
      Encoding::Mp3,
      include_bytes!("../../assets/test.mp3").to_vec(),
      "MP3",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_mp3_streaming() {
    test_async_dictation_streaming(
      Encoding::Mp3,
      include_bytes!("../../assets/test.mp3").to_vec(),
      "MP3",
    )
    .await;
  }

  // OGG Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_ogg_buffered() {
    test_async_dictation_buffered(
      Encoding::Ogg,
      include_bytes!("../../assets/test.ogg").to_vec(),
      "OGG",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_ogg_streaming() {
    test_async_dictation_streaming(
      Encoding::Ogg,
      include_bytes!("../../assets/test.ogg").to_vec(),
      "OGG",
    )
    .await;
  }

  // WAV Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_wav_buffered() {
    test_async_dictation_buffered(
      Encoding::Wav,
      include_bytes!("../../assets/test.wav").to_vec(),
      "WAV",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_wav_streaming() {
    test_async_dictation_streaming(
      Encoding::Wav,
      include_bytes!("../../assets/test.wav").to_vec(),
      "WAV",
    )
    .await;
  }

  // RAW Tests (PCM 8kHz, u8)
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_raw_buffered() {
    test_async_dictation_buffered(
      Encoding::Raw, // 8kHz, 8-bit, mono
      include_bytes!("../../assets/test.raw").to_vec(),
      "RAW",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_dictation_raw_streaming() {
    test_async_dictation_streaming(
      Encoding::Raw, // 8kHz, 8-bit, mono
      include_bytes!("../../assets/test.raw").to_vec(),
      "RAW",
    )
    .await;
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_dictation() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let params = DictationQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    );

    let results = client
      .post_blocking_dictation(params)
      .expect("Failed to get dictation results");

    assert!(
      !results.is_empty(),
      "Should have received at least one dictation result"
    );

    for dictation in &results {
      assert!(
        !dictation.text.is_empty(),
        "Dictation text should not be empty"
      );
      println!("Transcription: {}", dictation.text);
    }

    let last_dictation = results.last().expect("No dictation results were received");
    const EXPECTED_TEXT: &str = "the examination and testimony of the experts enabled the commission to conclude that five shots may have been fired";
    assert!(
      levenshtein_distance(
        last_dictation.text.to_ascii_lowercase().as_str(),
        EXPECTED_TEXT
      ) < 5,
      "Last dictation text is not similar enough to expected text"
    );
  }
}
