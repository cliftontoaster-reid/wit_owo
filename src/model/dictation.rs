use crate::prelude::*;
use serde::Deserialize;
use serde_json::de::StrRead;
use serde_json::{from_value, Deserializer, StreamDeserializer, Value};

/// Blocking version of dictation API.
#[cfg(feature = "blocking")]
pub mod blocking;

/// What wit.ai returns,
/// since it can sometimes be only the text,
/// we decided
/// to create this Enum to simplify
/// and adding the possibility of you having the entire process of computing in case of need.
pub enum DictationResponse {
  /// The full response, see [`FullSpeechResponse`].
  Full(FullDictationResponse),
  /// The half empty response, see [`HalfSpeechResponse`].
  Half(HalfSpeechResponse),
}

#[derive(Deserialize)]
/// The response for a speech request, similar to the [`Message`] struct.
pub struct FullDictationResponse {
  /// The original text, but this time it makes sense.
  pub text: String,
  /// More information on the speech recognition process.
  pub speech: SpeechInfo,
  /// To know if it's the final response.
  pub is_final: bool,
}
#[cfg(feature = "async")]
impl Client {
  /// It takes the audio to transcribe.
  ///
  /// To use it you will first need to create a client
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   let owo_client = Client::new(&token);
  ///   #
  ///   # let options = SpeechRequest {
  ///   #   content_type: AudioContentType::Mp3,
  ///   #   context: None,
  ///   #   tag: None,
  ///   #   n: 0,
  ///   #   entities: Default::default(),
  ///   # };
  ///   # let audio = include_bytes!("../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.dictation(audio.to_vec(), options).await.unwrap();
  /// }
  /// ```
  /// Then we initialize the options with the file format,
  /// note that here the dynamic entities don't have any impact,
  /// since it doesn't analyse the produced.
  /// We will also import the audio from a file in the repertory.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   # let owo_client = Client::new(&token);
  ///   #
  ///   let options = SpeechRequest {
  ///     content_type: AudioContentType::Mp3,
  ///     context: None,
  ///     tag: None,
  ///     n: 0,
  ///     entities: Default::default(),
  ///   };
  ///   let audio = include_bytes!("../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.dictation(audio.to_vec(), options).await.unwrap();
  /// }
  /// ```
  /// We can then send the request and wait from results.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   # let owo_client = Client::new(&token);
  ///   #
  ///   # let options = SpeechRequest {
  ///   #   content_type: AudioContentType::Mp3,
  ///   #   context: None,
  ///   #   tag: None,
  ///   #   n: 0,
  ///   #   entities: Default::default(),
  ///   # };
  ///   # let audio = include_bytes!("../../owo/test.mp3");
  ///   #
  ///   let response = owo_client.dictation(audio.to_vec(), options).await.unwrap();
  /// }
  /// ```
  /// And finally we extract the final response.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   # let owo_client = Client::new(&token);
  ///   #
  ///   # let options = SpeechRequest {
  ///   #   content_type: AudioContentType::Mp3,
  ///   #   context: None,
  ///   #   tag: None,
  ///   #   n: 0,
  ///   #   entities: Default::default(),
  ///   # };
  ///   # let audio = include_bytes!("../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.dictation(audio.to_vec(), options).await.unwrap();
  ///   let owo = response.last().unwrap();
  /// }
  /// ```
  pub async fn dictation(
    &self,
    audio: Vec<u8>,
    options: SpeechRequest,
  ) -> Result<Vec<DictationResponse>, WitError> {
    let uwu = self
      .prepare_post_request("https://api.wit.ai/speech")
      .header("content-type", options.content_type.to_str())
      .body(audio)
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap();

    let murr = Deserializer::from_str(&uwu).into_iter::<Value>();

    prepare_dictation_response(murr)
  }
}

/// Converts the JSON chunks to a [`Vec`] of [`DictationResponse`]
pub fn prepare_dictation_response(
  murr: StreamDeserializer<StrRead, Value>,
) -> Result<Vec<DictationResponse>, WitError> {
  let mut owo: Vec<DictationResponse> = Vec::new();

  for u in murr {
    let v: Value = u.unwrap();
    Client::extract::<Value>(&v)?;
    match v.as_object().unwrap().get("is_final") {
      None => {
        owo.push(DictationResponse::Half(from_value(v).unwrap()));
      }
      Some(_) => {
        owo.push(DictationResponse::Full(from_value(v).unwrap()));
      }
    }
  }

  Ok(owo)
}

#[cfg(test)]
mod tests {
  use crate::model::dictation::DictationResponse;
  use crate::prelude::*;
  use crate::utils::levenshtein_distance;
  use dotenv;
  use std::env;

  #[tokio::test]
  #[cfg(feature = "async")]
  async fn api_dictation() {
    dotenv::dotenv().ok();
    let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
    let options = SpeechRequest {
      content_type: AudioContentType::Mp3,
      context: None,
      tag: None,
      n: 0,
      entities: Default::default(),
    };
    let audio = include_bytes!("../../owo/test.mp3");
    let rawr = owo.dictation(audio.to_vec(), options).await.unwrap();
    let uwu = match rawr.last().unwrap() {
      DictationResponse::Full(d) => d,
      DictationResponse::Half(_) => unreachable!("Last should be final."),
    };

    assert!(levenshtein_distance("OwO what's this", &uwu.text) < 10);
  }
}
