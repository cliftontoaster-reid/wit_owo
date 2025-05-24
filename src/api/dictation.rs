use crate::error::ApiError;
use crate::model::dictation::{Dictation, DictationQuery};
use crate::prelude::BASE_URL;
use crate::prelude::WitClient;
use futures::stream::{Stream, StreamExt};
use url::Url;

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
  /// use wit_owo::{WitClient, DictationQuery, Encoding, AudioSource};
  /// use bytes::Bytes;
  /// use futures::stream::StreamExt;
  ///
  /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
  /// let client = WitClient::new("your_token_here");
  /// let audio_data = Bytes::from(vec![/* audio bytes */]);
  /// let params = DictationQuery::new(Encoding::Wav, AudioSource::Buffered(audio_data));
  ///
  /// let mut stream = client.post_dictation(params).await?;
  /// while let Some(result) = stream.next().await {
  ///     let dictation = result?;
  ///     println!("Transcription: {}", dictation.text);
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

    use crate::error::WitError;

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
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::dictation::{AudioSource, Encoding};
  use bytes::Bytes;
  use dotenv::dotenv;
  use std::env;

  #[tokio::test]
  async fn test_post_dictation() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let params = DictationQuery::new(
      Encoding::Wav,
      AudioSource::Buffered(Bytes::from(
        include_bytes!("../../assets/test.wav").as_ref(),
      )),
    );

    let mut stream = Box::pin(client.post_dictation(params).await);
    let mut received_results = false;

    while let Some(result) = stream.next().await {
      match result {
        Ok(dictation) => {
          assert!(
            !dictation.text.is_empty(),
            "Dictation text should not be empty"
          );
          println!("Transcription: {}", dictation.text);
          received_results = true;
        }
        Err(e) => {
          panic!("Dictation failed with error: {:?}", e);
        }
      }
    }

    assert!(
      received_results,
      "Should have received at least one dictation result"
    );
  }

  #[test]
  fn test_extract_complete_json() {
    // Test with a complete JSON object
    let buffer = r#"{"text": "hello"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, "");

    // Test with partial JSON (incomplete)
    let buffer = r#"{"text": "hel"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with complete JSON followed by partial
    let buffer = r#"{"text": "hello"}{"text": "wor"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"{"text": "wor"#);

    // Test with multiple complete JSON objects
    let buffer = r#"{"text": "hello"}{"text": "world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"{"text": "world"}"#);

    // Test with nested braces in string (should still work for simple JSON)
    let buffer = r#"{"text": "hello world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello world"}"#);
    assert_eq!(remaining, "");
  }

  #[test]
  fn test_extract_complete_json_edge_cases() {
    // Test with whitespace around JSON
    let buffer = r#"  {"text": "hello"}  {"text": "world"}"#;
    let result = extract_complete_json(buffer);
    assert!(result.is_some());
    let (json, remaining) = result.unwrap();
    assert_eq!(json, r#"{"text": "hello"}"#);
    assert_eq!(remaining, r#"  {"text": "world"}"#);

    // Test with no JSON objects
    let buffer = "some random text without braces";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with only opening brace
    let buffer = "{";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test with unmatched braces
    let buffer = "{{";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());

    // Test empty buffer
    let buffer = "";
    let result = extract_complete_json(buffer);
    assert!(result.is_none());
  }
}

/// Extracts the first complete JSON object from a buffer and returns it along with the remaining data.
///
/// This function counts '{' and '}' characters to determine when a complete JSON object is found.
/// When the brace count goes from >0 back to 0, we know we have a complete JSON object.
///
/// # Arguments
///
/// * `buffer` - The string buffer containing potentially partial JSON data
///
/// # Returns
///
/// Returns `Some((json_string, remaining_buffer))` if a complete JSON object is found,
/// or `None` if no complete JSON object is available yet.
fn extract_complete_json(buffer: &str) -> Option<(String, String)> {
  let mut brace_count = 0;
  let mut start_idx = None;

  for (i, ch) in buffer.char_indices() {
    match ch {
      '{' => {
        if brace_count == 0 {
          start_idx = Some(i);
        }
        brace_count += 1;
      }
      '}' => {
        brace_count -= 1;
        if brace_count == 0 && start_idx.is_some() {
          // We found a complete JSON object
          let start = start_idx.unwrap();
          let end = i + 1; // Include the closing brace
          let json_str = buffer[start..end].to_string();
          let remaining = buffer[end..].to_string();
          return Some((json_str, remaining));
        }
      }
      _ => {}
    }
  }

  None
}
