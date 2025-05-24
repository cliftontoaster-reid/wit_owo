use crate::error::ApiError;
use crate::model::dictation::{Dictation, DictationQuery};
use crate::prelude::BASE_URL;
use crate::prelude::WitClient;
use crate::utils::json::extract_complete_json;
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
    use crate::error::WitError;
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
  use bytes::Bytes;
  use dotenv::dotenv;
  use std::env;

  #[cfg(feature = "async")]
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

    for dictation in results {
      assert!(
        !dictation.text.is_empty(),
        "Dictation text should not be empty"
      );
      println!("Transcription: {}", dictation.text);
    }
  }
}
