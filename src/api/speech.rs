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
    println!("Request {request:?}");

    let response = request.send()?;
    if !response.status().is_success() {
      return Err(serde_json::from_str::<WitError>(&response.text()?)?)?;
    }

    let text = response.text()?;
    let mut results = Vec::new();
    let mut buffer = text;

    while let Some((json_str, remaining)) = extract_complete_json(&buffer) {
      // We print the JSON for debugging purposes
      println!("Received complete JSON: {json_str:?}");

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
          println!("Speech Response with context: {speech_response:?}");
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
        _ => {
          panic!("Unexpected speech response type with context: {speech_response:?}");
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
