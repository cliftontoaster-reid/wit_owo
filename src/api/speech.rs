//! # Wit.ai Speech API
//!
//! This module provides a comprehensive interface for performing speech-to-text transcription
//! and natural language understanding (intent and entity recognition) using the Wit.ai Speech API.
//! Unlike the dictation endpoint which only provides transcription results, the Speech API
//! returns rich understanding data (intents and entities) along with interim and final
//! transcription outputs. Both asynchronous streaming and blocking batch modes are supported.
//!
//! ## Overview
//!
//! - **Streaming (Async)**: Real-time processing of audio streams with partial and final updates
//! - **Blocking (Sync)**: Send a complete audio payload and receive all results at once
//! - **Natural Language Understanding**: Automatic intent and entity extraction
//! - **Multiple Formats**: WAV, MP3, OGG, μ-law, and raw PCM
//!
//! ## Supported Audio Formats
//!
//! | Format | Description                      | Content-Type                |
//! |--------|----------------------------------|-----------------------------|
//! | WAV    | Waveform Audio File Format       | `audio/wav`                 |
//! | MP3    | MPEG Audio Layer III             | `audio/mpeg3`               |
//! | OGG    | Ogg Vorbis audio format          | `audio/ogg`                 |
//! | μ-law  | µ-law algorithm (telephony)      | `audio/ulaw`                |
//! | Raw    | Raw PCM audio data               | `audio/raw; encoding=...`   |
//!
//! ## Getting Started
//!
//! 1. **Create a Wit.ai App**: Sign up at [wit.ai](https://wit.ai) and create an application.
//! 2. **Obtain API Token**: Copy the server access token from your app settings.
//! 3. **Enable Features**: In `Cargo.toml`, enable `async` and/or `blocking` under `features`.
//! 4. **Prepare Audio**: Collect or record audio files or streams in a supported format.
//!
//! ### Basic Client Initialization
//!
//! ```rust
//! use wit_owo::prelude::*;
//! # use dotenvy::dotenv;
//! # use std::env;
//!
//! # dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//! // Initialize the Wit.ai client
//! let client = WitClient::new(&token);
//! ```
//!
//! ## Quick Start Examples
//!
//! ### 1. Asynchronous Streaming Mode
//!
//! Receive partial and final transcription and understanding results as they arrive:
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn speech_stream_example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use futures::StreamExt;
//! use bytes::Bytes;
//! # use dotenvy::dotenv;
//! # use std::env;
//!
//! # dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").unwrap();
//! let client = WitClient::new(&token);
//!
//! // Load or stream audio data
//! let data = std::fs::read("assets/test.wav").unwrap_or_else(|_| vec![0u8; 1024]);
//! let query = SpeechQuery::new(Encoding::Wav, AudioSource::Buffered(Bytes::from(data)));
//!
//! // Start the streaming call
//! let mut stream = Box::pin(client.post_speech(query).await);
//!
//! while let Some(item) = stream.next().await {
//!     match item? {
//!         SpeechResponse::PartialTranscription(p) => println!("Interim: {}", p.text),
//!         SpeechResponse::FinalTranscription(f)   => println!("Final Transcript: {}", f.text),
//!         SpeechResponse::PartialUnderstanding(u) => println!("Interim NLU: {}", u.text),
//!         SpeechResponse::FinalUnderstanding(u)   => {
//!             println!("Final NLU: {}", u.text);
//!             println!("Intents: {:?}", u.intents);
//!             println!("Entities: {:?}", u.entities);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! # #[cfg(feature = "async")]
//! # #[tokio::main]
//! # async fn main() { let _ = speech_stream_example().await; }
//! # #[cfg(not(feature = "async"))]
//! # fn main() { }
//! ```
//!
//! ### 2. Blocking Batch Mode
//!
//! Send a complete audio file and receive all results after processing:
//!
//! ```rust
//! # #[cfg(feature = "blocking")]
//! # fn speech_blocking_example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//! # use dotenvy::dotenv;
//! # use std::env;
//!
//! # dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").unwrap();
//! let client = WitClient::new(&token);
//!
//! // Read audio file into memory
//! let data = std::fs::read("assets/test.mp3").unwrap_or_else(|_| vec![0u8; 1024]);
//! let query = SpeechQuery::new(Encoding::Mp3, AudioSource::Buffered(Bytes::from(data)))
//!     .with_n(3); // limit to top 3 intents
//!
//! // Execute blocking call
//! let results = client.post_blocking_speech(query)?;
//!
//! for res in results {
//!     match res {
//!         SpeechResponse::FinalTranscription(t) => println!("Transcript: {}", t.text),
//!         SpeechResponse::FinalUnderstanding(u) => println!("NLU: {:?}", u.intents),
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! # #[cfg(feature = "blocking")]
//! # fn main() { let _ = speech_blocking_example(); }
//! # #[cfg(not(feature = "blocking"))]
//! # fn main() { }
//! ```
//!
//! ### 3. Raw PCM Audio
//!
//! When sending raw PCM, specify bit depth, sample rate, endian, and encoding:
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn raw_audio_example() -> Result<(), Box<dyn std::error::Error>> {
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//! # use dotenvy::dotenv;
//! # use std::env;
//! use futures::StreamExt;
//!
//! # dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").unwrap();
//! let client = WitClient::new(&token);
//! let mut query = SpeechQuery::new(
//!     Encoding::Raw,
//!     AudioSource::Buffered(Bytes::from(std::fs::read("assets/test.raw").unwrap_or_else(|_| vec![0u8; 1024])))
//! )
//! .with_bits(16)
//! .with_sample_rate(16000)
//! .with_endian(true)
//! .with_raw_encoding("signed-integer".to_string());
//!
//! let mut stream = Box::pin(client.post_speech(query).await);
//! while let Some(item) = stream.next().await {
//!     println!("Raw PCM output: {:?}", item?);
//! }
//! # Ok(())
//! # }
//! # #[cfg(feature = "async")]
//! # #[tokio::main]
//! # async fn main() { let _ = raw_audio_example().await; }
//! # #[cfg(not(feature = "async"))]
//! # fn main() { }
//! ```
//!
//! ## Audio Format Guidelines
//!
//! ### Recommended Settings
//!
//! To achieve optimal transcription and NLU performance:
//!
//! - **Sample Rate**: 16 kHz or higher (8 kHz minimum)
//! - **Bit Depth**: 16-bit or higher
//! - **Channels**: Mono preferred, stereo supported
//! - **Format**: WAV or FLAC for highest fidelity, MP3/OGG for smaller file size
//!
//! ### Format-Specific Notes
//!
//! **WAV**:
//! - Uncompressed PCM audio
//! - Supports various bit depths
//! - No compression artifacts
//!
//! **MP3**:
//! - Good compression-to-quality tradeoff
//! - Widely supported
//! - Minor artifacts possible
//!
//! **OGG**:
//! - Open container with Vorbis or Opus codecs
//! - Efficient streaming support
//!
//! **μ-law**:
//! - Telephony standard
//! - 8 kHz sampling
//!
//! **Raw PCM**:
//! - Requires manual specification of encoding parameters
//! - Highest flexibility for custom audio
//!
//! ### Raw PCM Configuration
//!
//! ```rust
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//! use std::collections::HashMap;
//! // Example: 16-bit signed PCM, 16 kHz, little-endian
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! let mut query = SpeechQuery::new(
//!     Encoding::Raw,
//!     AudioSource::Buffered(Bytes::from(std::fs::read("assets/test.raw").unwrap_or_else(|_| vec![0u8; 1024])))
//! )
//! .with_bits(16)                // bit depth
//! .with_sample_rate(16000)      // sample rate (Hz)
//! .with_endian(true)            // true = little-endian
//! .with_raw_encoding("signed-integer".to_string());
//! # Ok(())
//! # };
//! ```
//!
//! ## Understanding SpeechResponse
//!
//! The `SpeechResponse` enum encapsulates both transcription and NLU output:
//!
//! ```rust
//! use wit_owo::prelude::*;
//! use std::collections::HashMap;
//! pub enum SpeechResponse {
//!     PartialTranscription(SpeechTranscription),
//!     FinalTranscription(SpeechTranscription),
//!     PartialUnderstanding(SpeechUnderstanding),
//!     FinalUnderstanding(SpeechUnderstanding),
//! }
//!
//! pub struct SpeechTranscription {
//!     pub text: String,      // Transcribed text
//!     pub confidence: f32,   // Confidence score (0.0-1.0)
//! }
//!
//! pub struct SpeechUnderstanding {
//!     pub text: String,                              // Transcribed text context
//!     pub intents: Vec<Intent>,                     // Extracted intents
//!     pub entities: HashMap<String, Vec<Entity>>,  // Extracted entities
//!     pub confidence: f32,                        // Overall confidence
//! }
//! ```
//!
//! ### Response Types
//!
//! - **PartialTranscription**: Interim transcription may be updated
//! - **FinalTranscription**: Stable completed transcription
//! - **PartialUnderstanding**: Interim NLU, preliminary intents/entities
//! - **FinalUnderstanding**: Stable NLU output
//!
//! ## Error Handling
//!
//! Handle errors at request level or API/NLU layer:
//!
//! ```rust
//! # #[cfg(feature = "blocking")] {
//! # use wit_owo::prelude::*;
//! # use bytes::Bytes;
//! # dotenvy::dotenv().ok();
//! # let token = std::env::var("WIT_API_TOKEN").unwrap();
//! # let client = WitClient::new(&token);
//! # let audio_data = std::fs::read("assets/test.wav").unwrap_or_else(|_| vec![0u8; 1024]);
//! # let query = SpeechQuery::new(Encoding::Wav, AudioSource::Buffered(Bytes::from(audio_data)));
//! match client.post_blocking_speech(query) {
//!     Ok(results) => {
//!         // Process successful responses
//!         for resp in results {
//!             println!("Result: {:?}", resp);
//!         }
//!     }
//!     Err(ApiError::RequestError(e)) => {
//!         eprintln!("Network error: {}", e);
//!     }
//!     Err(ApiError::WitError(werr)) => {
//!         eprintln!("Wit.ai error: {}", werr);
//!     }
//!     Err(ApiError::SerializationError(e)) => {
//!         eprintln!("Response parse error: {}", e);
//!     }
//!     Err(e) => {
//!         eprintln!("Unexpected error: {}", e);
//!     }
//! }
//! # }
//! ```
//!
//! ## Performance Tips
//!
//! ### Large File Processing
//!
//! - Use streaming mode for files > 5 MB
//! - Chunk size of 1–5 MB for balanced throughput
//! - Offload audio encoding to background threads
//!
//! ### Real-Time Applications
//!
//! - Use smaller chunk sizes (512–2048 bytes)
//! - Buffer and debounce partial results for smooth UI
//! - Implement retry logic on transient failures
//!
//! ### Memory Optimization
//!
//! Prefer streaming audio sources to avoid loading entire file:
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::prelude::*;
//! use bytes::Bytes;
//! // For large data, use this pattern to create a streaming source
//! let audio_stream = futures::stream::iter(
//!     (0..3).map(|_i| {
//!         let chunk = vec![0u8; 256]; // Small chunks for demo
//!         Ok::<Bytes, _>(Bytes::from(chunk))
//!     })
//! );
//! let query = SpeechQuery::new(Encoding::Raw, AudioSource::Stream(Box::pin(audio_stream)))
//!     .with_bits(16)
//!     .with_sample_rate(16000)
//!     .with_raw_encoding("signed-integer".to_string());
//! // Just construct the query, don't actually send it in doctest
//! assert_eq!(query.encoding, Encoding::Raw);
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! In `Cargo.toml`, enable the desired modes:
//!
//! ```toml
//! [dependencies]
//! wit_owo = { version = "1.0", features = ["async", "blocking"] }
//! ```
//!
//! - `async`: Asynchronous streaming API
//! - `blocking`: Synchronous batch API
//!
//! ## Troubleshooting
//!
//! **Invalid audio format**:
//! - Verify file integrity
//! - Match file extension with encoding
//!
//! **Authentication failed**:
//! - Check API token validity
//! - Ensure proper environment variable
//!
//! **No audio detected**:
//! - Confirm audio length > 100 ms
//! - Validate sample rate/channel configuration
//!
//! **Low confidence**:
//! - Improve audio quality
//! - Increase bit depth and sample rate
//!
//! ## Advanced Usage
//!
//! ### Custom Audio Sources
//!
//! Supply your own streams or buffers:
//!
//! ```rust,no_run
//! # use wit_owo::prelude::*;
//! # use bytes::Bytes;
//! // From Vec<u8>
//! let my_vec = vec![0u8; 1024]; // Example audio data
//! let buf_source = AudioSource::Buffered(Bytes::from(my_vec));
//!
//! // From custom async stream (example pattern - actual implementation would vary)
//! // let my_stream = my_custom_stream_source();
//! // let stream_source = AudioSource::Stream(Box::pin(my_stream));
//!
//! # // Verify the source is constructed correctly
//! # match buf_source { AudioSource::Buffered(_) => {}, _ => panic!("Wrong type") }
//! ```
//!
//! ### Integrations
//!
//! Works with popular Rust audio crates:
//! - `hound` for WAV file reading/writing
//! - `rodio` or `cpal` for real-time capture/playback
//! - `lewton` for Ogg decoding
//!

use crate::error::{ApiError, WitError};
use crate::model::speech::{SpeechQuery, SpeechResponse, SpeechTranscription, SpeechUnderstanding};
use crate::prelude::WitClient;
use crate::utils::json::extract_complete_json;
#[cfg(feature = "async")]
use futures::{Stream, StreamExt as _};
use serde_json::Value;

impl WitClient {
  /// Performs speech-to-text with natural language understanding using the Wit.ai API.
  ///
  /// This method sends audio data to the Wit.ai speech endpoint and returns a stream
  /// of partial and final transcription and understanding results as they become available.
  /// Unlike the dictation endpoint which only provides transcription, the speech endpoint
  /// also provides intent and entity recognition.
  ///
  /// # Arguments
  ///
  /// * `params` - A `SpeechQuery` containing the audio encoding format, audio data,
  ///   and optional parameters like context, dynamic entities, and intent limits
  ///
  /// # Returns
  ///
  /// Returns a `Stream` that yields `Result<SpeechResponse, ApiError>` items. Each item contains
  /// either a transcription result or an understanding result (with intents/entities), which may
  /// be partial (interim) or final depending on the response from the API.
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
  /// // Create a speech query with the appropriate encoding
  /// let params = SpeechQuery::new(
  ///     Encoding::Wav,
  ///     AudioSource::Buffered(audio_data)
  /// );
  ///
  /// // Send the audio data to Wit.ai and process the streaming response
  /// let mut stream = Box::pin(client.post_speech(params).await);
  ///
  /// // Iterate through the stream of speech results
  /// while let Some(result) = stream.next().await {
  ///     match result {
  ///         Ok(speech_response) => {
  ///             match speech_response {
  ///                 SpeechResponse::FinalTranscription(transcription) => {
  ///                     println!("Transcription: {}", transcription.text);
  ///                 },
  ///                 SpeechResponse::FinalUnderstanding(understanding) => {
  ///                     println!("Understanding: {}", understanding.text);
  ///                     println!("Intents: {:?}", understanding.intents);
  ///                     println!("Entities: {:?}", understanding.entities);
  ///                 },
  ///                 SpeechResponse::PartialTranscription(_) |
  ///                 SpeechResponse::PartialUnderstanding(_) => {
  ///                     // Handle partial results as needed
  ///                 }
  ///             }
  ///         },
  ///         Err(e) => eprintln!("Error: {}", e)
  ///     }
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "async")]
  pub async fn post_speech(
    &self,
    params: SpeechQuery,
  ) -> impl Stream<Item = Result<SpeechResponse, ApiError>> {
    use async_stream::try_stream;

    try_stream! {
      let content_type = params.to_string();
      let url = params.to_url()?;

      let request = self
        .prepare_post_request(url)
        .header("Content-Type", content_type)
        .body(params.data);

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
          // Deserialize the complete JSON object
          let value = serde_json::from_str::<Value>(&json_str)?;

          // Check if the value is an object with a "type" field
          if let Value::String(type_str) = value.get("type").unwrap() {
            match type_str.as_str() {
              "PARTIAL_TRANSCRIPTION" => {
                // Handle partial transcription
                let data = SpeechResponse::PartialTranscription(serde_json::from_value::<SpeechTranscription>(value)?);

                yield data;
              }
              "PARTIAL_UNDERSTANDING" => {
                // Handle understanding
                let data = SpeechResponse::PartialUnderstanding(serde_json::from_value::<SpeechUnderstanding>(value)?);

                yield data;
              }

              "FINAL_TRANSCRIPTION" => {
                // Handle final transcription
                let data = SpeechResponse::FinalTranscription(serde_json::from_value::<SpeechTranscription>(value)?);

                yield data;
              }
              "FINAL_UNDERSTANDING" => {
                // Handle final understanding
                let data = SpeechResponse::FinalUnderstanding(serde_json::from_value::<SpeechUnderstanding>(value)?);

                yield data;
              }
              _ => {
              }
            }
          }

          // Update buffer with remaining data
          buffer = remaining;
        }
      }

    }
  }

  /// Performs speech-to-text with natural language understanding using the Wit.ai API (blocking version).
  ///
  /// This method sends audio data to the Wit.ai speech endpoint and returns all
  /// transcription and understanding results at once. Unlike the streaming version,
  /// this method blocks until all processing is complete and returns a vector containing
  /// all partial and final results. Unlike the dictation endpoint which only provides
  /// transcription, the speech endpoint also provides intent and entity recognition.
  ///
  /// # Arguments
  ///
  /// * `params` - A `SpeechQuery` containing the audio encoding format, audio data,
  ///   and optional parameters like context, dynamic entities, and intent limits
  ///
  /// # Returns
  ///
  /// Returns a `Vec<SpeechResponse>` containing all transcription and understanding results.
  /// Each item contains either a transcription result or an understanding result (with
  /// intents/entities), which may be partial (interim) or final depending on the response
  /// from the API.
  ///
  /// # Errors
  ///
  /// This method will return an error if:
  /// * The URL parsing fails
  /// * The HTTP request fails to send
  /// * The API returns a non-success status code
  /// * JSON deserialization of the response fails
  ///
  /// # Feature Requirements
  ///
  /// This method is only available when the `blocking` feature is enabled.
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
  /// // Create a speech query with the appropriate encoding
  /// let params = SpeechQuery::new(
  ///     Encoding::Wav,
  ///     AudioSource::Buffered(audio_data)
  /// );
  ///
  /// // Send the audio data to Wit.ai and get all results at once
  /// let results = client.post_blocking_speech(params)?;
  ///
  /// // Process all speech results
  /// for speech_response in results {
  ///     match speech_response {
  ///         SpeechResponse::FinalTranscription(transcription) => {
  ///             println!("Final Transcription: {}", transcription.text);
  ///         },
  ///         SpeechResponse::FinalUnderstanding(understanding) => {
  ///             println!("Final Understanding: {}", understanding.text);
  ///             println!("Intents: {:?}", understanding.intents);
  ///             println!("Entities: {:?}", understanding.entities);
  ///         },
  ///         SpeechResponse::PartialTranscription(transcription) => {
  ///             println!("Partial Transcription: {}", transcription.text);
  ///         },
  ///         SpeechResponse::PartialUnderstanding(understanding) => {
  ///             println!("Partial Understanding: {}", understanding.text);
  ///         }
  ///     }
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "blocking")]
  pub fn post_blocking_speech(&self, params: SpeechQuery) -> Result<Vec<SpeechResponse>, ApiError> {
    let content_type = params.to_string();
    let url = params.to_url()?;

    let request = self
      .prepare_post_blocking(url)
      .header("Content-Type", content_type)
      .body(params.data);

    let response = request.send()?;
    if !response.status().is_success() {
      return Err(serde_json::from_str::<WitError>(&response.text()?)?)?;
    }

    let text = response.text()?;
    let mut results = Vec::new();
    let mut buffer = text;

    while let Some((json_str, remaining)) = extract_complete_json(&buffer) {
      // Deserialize the complete JSON object
      let value = serde_json::from_str::<Value>(&json_str)?;

      // Check if the value is an object with a "type" field
      if let Value::String(type_str) = value.get("type").unwrap() {
        match type_str.as_str() {
          "PARTIAL_TRANSCRIPTION" => {
            // Handle partial transcription
            let data = SpeechResponse::PartialTranscription(serde_json::from_value::<
              SpeechTranscription,
            >(value)?);
            results.push(data);
          }
          "PARTIAL_UNDERSTANDING" => {
            // Handle understanding
            let data = SpeechResponse::PartialUnderstanding(serde_json::from_value::<
              SpeechUnderstanding,
            >(value)?);
            results.push(data);
          }
          "FINAL_TRANSCRIPTION" => {
            // Handle final transcription
            let data = SpeechResponse::FinalTranscription(serde_json::from_value::<
              SpeechTranscription,
            >(value)?);
            results.push(data);
          }
          "FINAL_UNDERSTANDING" => {
            // Handle final understanding
            let data = SpeechResponse::FinalUnderstanding(serde_json::from_value::<
              SpeechUnderstanding,
            >(value)?);
            results.push(data);
          }
          _ => {}
        }
      }

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

  /// Helper function to test async speech with buffered audio data
  #[cfg(feature = "async")]
  async fn test_async_speech_buffered(encoding: Encoding, audio_data: Vec<u8>, format_name: &str) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let mut params = SpeechQuery::new(encoding, AudioSource::Buffered(Bytes::from(audio_data)));

    if encoding == Encoding::Raw {
      // For RAW encoding, we need to specify additional parameters
      params = params
        .with_bits(8)
        .with_sample_rate(8000)
        .with_endian(true)
        .with_raw_encoding("unsigned-integer".to_string());
    }

    let mut stream = Box::pin(client.post_speech(params).await);
    let mut received_results = false;
    let mut last_text: Option<String> = None;

    while let Some(result) = stream.next().await {
      match result {
        Ok(speech_response) => {
          let text = match &speech_response {
            SpeechResponse::PartialTranscription(transcription) => &transcription.text,
            SpeechResponse::FinalTranscription(transcription) => &transcription.text,
            SpeechResponse::PartialUnderstanding(understanding) => &understanding.text,
            SpeechResponse::FinalUnderstanding(understanding) => &understanding.text,
          };

          assert!(
            !text.is_empty(),
            "Speech response text should not be empty for {format_name} format",
          );
          println!("{format_name} Speech Response: {speech_response:?}");
          received_results = true;
          last_text = Some(text.clone());
        }
        Err(e) => {
          panic!("Speech request failed with error for {format_name} format: {e:?}",);
        }
      }
    }

    assert!(
      received_results,
      "Should have received at least one speech response for {format_name} format",
    );

    if let Some(text) = last_text {
      assert!(
        !text.is_empty(),
        "Last speech text should not be empty for {format_name} format",
      );
      assert!(
        levenshtein_distance(text.to_ascii_lowercase().as_str(), EXPECTED_TEXT) < 5,
        "Last speech text is not similar enough to expected text for {format_name} format",
      );
    } else {
      panic!("No speech results were received for {format_name} format",);
    }
  }

  /// Helper function to test async speech with streaming audio data
  #[cfg(feature = "async")]
  async fn test_async_speech_streaming(encoding: Encoding, audio_data: Vec<u8>, format_name: &str) {
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

    // Build the speech query using the streaming audio source
    let mut params = SpeechQuery::new(encoding, AudioSource::Stream(Box::pin(audio_stream)));

    if encoding == Encoding::Raw {
      // For RAW encoding, we need to specify additional parameters
      params = params
        .with_bits(8)
        .with_sample_rate(8000)
        .with_endian(true)
        .with_raw_encoding("unsigned-integer".to_string());
    }

    // Send to Wit.ai and collect the streaming results
    let mut stream = Box::pin(client.post_speech(params).await);
    let mut received = false;
    let mut last_text = None;

    while let Some(item) = stream.next().await {
      let speech_response =
        item.unwrap_or_else(|e| panic!("streaming speech failed for {format_name} format: {e:?}",));

      let text = match &speech_response {
        SpeechResponse::PartialTranscription(transcription) => &transcription.text,
        SpeechResponse::FinalTranscription(transcription) => &transcription.text,
        SpeechResponse::PartialUnderstanding(understanding) => &understanding.text,
        SpeechResponse::FinalUnderstanding(understanding) => &understanding.text,
      };

      assert!(
        !text.is_empty(),
        "Speech response text should not be empty for {format_name} format"
      );
      received = true;
      last_text = Some(text.clone());
    }

    assert!(
      received,
      "Should have received at least one speech response for {format_name} format"
    );

    let final_text = last_text
      .unwrap_or_else(|| panic!("No speech results were received for {format_name} format",));
    assert!(
      levenshtein_distance(final_text.to_ascii_lowercase().as_str(), EXPECTED_TEXT) < 5,
      "Last speech text is not similar enough to expected text for {format_name} format"
    );
  }

  /// Helper function to test blocked speech with buffered audio data
  #[cfg(feature = "blocking")]
  fn test_blocking_speech_buffered(encoding: Encoding, audio_data: Vec<u8>, format_name: &str) {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let mut params = SpeechQuery::new(encoding, AudioSource::Buffered(Bytes::from(audio_data)));

    params = params.with_n(3); // Limit to 3 intents

    if encoding == Encoding::Raw {
      // For RAW encoding, we need to specify additional parameters
      params = params
        .with_bits(8)
        .with_sample_rate(8000)
        .with_endian(true)
        .with_raw_encoding("unsigned-integer".to_string());
    }

    let response = client.post_blocking_speech(params);
    assert!(
      response.is_ok(),
      "Failed to get speech response for {format_name} format"
    );

    let results = response.unwrap();
    assert!(
      !results.is_empty(),
      "Should have received at least one speech response for {format_name} format"
    );

    for speech_response in results.iter().cloned() {
      match speech_response {
        SpeechResponse::PartialTranscription(transcription) => {
          assert!(
            !transcription.text.is_empty(),
            "Partial transcription text should not be empty for {format_name} format"
          );
          println!(
            "{format_name} Partial Transcription: {}",
            transcription.text
          );
        }
        SpeechResponse::FinalTranscription(transcription) => {
          assert!(
            !transcription.text.is_empty(),
            "Final transcription text should not be empty for {format_name} format"
          );
          println!("{format_name} Final Transcription: {}", transcription.text);
        }
        SpeechResponse::PartialUnderstanding(understanding) => {
          assert!(
            !understanding.text.is_empty(),
            "Partial understanding text should not be empty for {format_name} format"
          );
          println!(
            "{format_name} Partial Understanding: {}",
            understanding.text
          );
        }
        SpeechResponse::FinalUnderstanding(understanding) => {
          assert!(
            !understanding.text.is_empty(),
            "Final understanding text should not be empty for {format_name} format"
          );
          println!("{format_name} Final Understanding: {}", understanding.text);
          // Check that intents are limited to max 3 as requested
          assert!(
            understanding.intents.len() <= 3,
            "Should respect n=3 limit on intents for {format_name} format"
          );
        }
      }
    }

    let last_text = results.last().map(|r| match r {
      SpeechResponse::PartialTranscription(transcription) => transcription.text.clone(),
      SpeechResponse::FinalTranscription(transcription) => transcription.text.clone(),
      SpeechResponse::PartialUnderstanding(understanding) => understanding.text.clone(),
      SpeechResponse::FinalUnderstanding(understanding) => understanding.text.clone(),
    });

    assert!(
      last_text.is_some(),
      "No speech results were received for {format_name} format"
    );

    let final_text = last_text.unwrap();
    assert!(
      levenshtein_distance(final_text.to_ascii_lowercase().as_str(), EXPECTED_TEXT) < 5,
      "Last speech text is not similar enough to expected text for {format_name} format"
    );
  }

  // MP3 Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_mp3_buffered() {
    test_async_speech_buffered(
      Encoding::Mp3,
      include_bytes!("../../assets/test.mp3").to_vec(),
      "MP3",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_mp3_streaming() {
    test_async_speech_streaming(
      Encoding::Mp3,
      include_bytes!("../../assets/test.mp3").to_vec(),
      "MP3",
    )
    .await;
  }

  // OGG Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_ogg_buffered() {
    test_async_speech_buffered(
      Encoding::Ogg,
      include_bytes!("../../assets/test.ogg").to_vec(),
      "OGG",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_ogg_streaming() {
    test_async_speech_streaming(
      Encoding::Ogg,
      include_bytes!("../../assets/test.ogg").to_vec(),
      "OGG",
    )
    .await;
  }

  // WAV Tests
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_wav_buffered() {
    test_async_speech_buffered(
      Encoding::Wav,
      include_bytes!("../../assets/test.wav").to_vec(),
      "WAV",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_wav_streaming() {
    test_async_speech_streaming(
      Encoding::Wav,
      include_bytes!("../../assets/test.wav").to_vec(),
      "WAV",
    )
    .await;
  }

  // RAW Tests (PCM 8kHz, u8)
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_raw_buffered() {
    test_async_speech_buffered(
      Encoding::Raw, // 8kHz, 8-bit, mono
      include_bytes!("../../assets/test.raw").to_vec(),
      "RAW",
    )
    .await;
  }

  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_raw_streaming() {
    test_async_speech_streaming(
      Encoding::Raw, // 8kHz, 8-bit, mono
      include_bytes!("../../assets/test.raw").to_vec(),
      "RAW",
    )
    .await;
  }

  // Blocking MP3 Tests
  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_speech_mp3_buffered() {
    test_blocking_speech_buffered(
      Encoding::Mp3,
      include_bytes!("../../assets/test.mp3").to_vec(),
      "MP3",
    );
  }

  /// Blocking OGG Tests
  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_speech_ogg_buffered() {
    test_blocking_speech_buffered(
      Encoding::Ogg,
      include_bytes!("../../assets/test.ogg").to_vec(),
      "OGG",
    );
  }

  // Blocking WAV Tests
  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_speech_wav_buffered() {
    test_blocking_speech_buffered(
      Encoding::Wav,
      include_bytes!("../../assets/test.wav").to_vec(),
      "WAV",
    );
  }

  // Blocking RAW Tests (PCM 8kHz, u8)
  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_speech_raw_buffered() {
    test_blocking_speech_buffered(
      Encoding::Raw, // 8kHz, 8-bit, mono
      include_bytes!("../../assets/test.raw").to_vec(),
      "RAW",
    );
  }

  // Test with context and dynamic entities
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_post_speech_with_context() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let params = SpeechQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    )
    .with_n(3); // Limit to 3 intents

    let mut stream = Box::pin(client.post_speech(params).await);
    let mut received_results = false;

    while let Some(result) = stream.next().await {
      match result {
        Ok(speech_response) => {
          received_results = true;

          // Verify response structure based on type
          match speech_response {
            SpeechResponse::PartialTranscription(transcription)
            | SpeechResponse::FinalTranscription(transcription) => {
              assert!(
                !transcription.text.is_empty(),
                "Transcription text should not be empty"
              );
            }
            SpeechResponse::PartialUnderstanding(understanding)
            | SpeechResponse::FinalUnderstanding(understanding) => {
              assert!(
                !understanding.text.is_empty(),
                "Understanding text should not be empty"
              );
              // Check that intents are limited to max 3 as requested
              assert!(
                understanding.intents.len() <= 3,
                "Should respect n=3 limit on intents"
              );
            }
          }
        }
        Err(e) => {
          panic!("Speech request with context failed: {e:?}");
        }
      }
    }

    assert!(
      received_results,
      "Should have received at least one speech response with context"
    );
  }

  /// Test blocking speech with context and dynamic entities
  #[cfg(feature = "blocking")]
  #[test]
  fn test_post_blocking_speech_with_context() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);
    let params = SpeechQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    )
    .with_n(3); // Limit to 3 intents

    let response = client.post_blocking_speech(params);
    assert!(
      response.is_ok(),
      "Failed to get speech response with context: {}",
      response.unwrap_err()
    );

    let results = response.unwrap();
    assert!(
      !results.is_empty(),
      "Should have received at least one speech response with context"
    );

    for speech_response in results.iter().cloned() {
      match speech_response {
        SpeechResponse::PartialTranscription(transcription) => {
          assert!(
            !transcription.text.is_empty(),
            "Partial transcription text should not be empty"
          );
          println!("Partial Transcription with context: {}", transcription.text);
        }
        SpeechResponse::FinalTranscription(transcription) => {
          assert!(
            !transcription.text.is_empty(),
            "Final transcription text should not be empty"
          );
          println!("Final Transcription with context: {}", transcription.text);
        }
        SpeechResponse::PartialUnderstanding(understanding) => {
          assert!(
            !understanding.text.is_empty(),
            "Partial understanding text should not be empty"
          );
          println!("Partial Understanding with context: {}", understanding.text);
        }
        SpeechResponse::FinalUnderstanding(understanding) => {
          assert!(
            !understanding.text.is_empty(),
            "Final understanding text should not be empty"
          );
          println!("Final Understanding with context: {}", understanding.text);
          // Check that intents are limited to max 3 as requested
          assert!(
            understanding.intents.len() <= 3,
            "Should respect n=3 limit on intents"
          );
        }
      }
    }
  }

  // Test speech response types
  #[cfg(feature = "async")]
  #[tokio::test]
  async fn test_speech_response_types() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let params = SpeechQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    );

    let mut stream = Box::pin(client.post_speech(params).await);
    let mut transcription_received = false;
    let mut understanding_received = false;

    while let Some(result) = stream.next().await {
      match result {
        Ok(speech_response) => match speech_response {
          SpeechResponse::FinalTranscription(_) => {
            transcription_received = true;
            println!("Received transcription response");
          }
          SpeechResponse::FinalUnderstanding(_) => {
            understanding_received = true;
            println!("Received understanding response");
          }
          _ => {}
        },
        Err(e) => {
          panic!("Speech response types test failed: {e:?}");
        }
      }
    }

    // We should receive at least transcription responses
    assert!(
      transcription_received,
      "Should have received at least one transcription response"
    );

    println!("Transcription received: {transcription_received}");
    println!("Understanding received: {understanding_received}");
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn test_blocking_speech_response_types() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let params = SpeechQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    );

    let response = client.post_blocking_speech(params);
    assert!(
      response.is_ok(),
      "Failed to get speech response: {}",
      response.unwrap_err()
    );

    let results = response.unwrap();
    let mut transcription_received = false;
    let mut understanding_received = false;

    for speech_response in results {
      match speech_response {
        SpeechResponse::FinalTranscription(_) => {
          transcription_received = true;
          println!("Received transcription response");
        }
        SpeechResponse::FinalUnderstanding(_) => {
          understanding_received = true;
          println!("Received understanding response");
        }
        _ => {}
      }
    }

    // We should receive at least transcription responses
    assert!(
      transcription_received,
      "Should have received at least one transcription response"
    );
    assert!(
      understanding_received,
      "Should have received at least one understanding response"
    );
  }
}
