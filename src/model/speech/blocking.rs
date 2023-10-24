use crate::model::speech::prepare_speech_response;
use crate::prelude::*;
use serde_json::de::StrRead;
use serde_json::{Deserializer, StreamDeserializer, Value};

impl Client {
  /// It takes the audio to transcribe then analyse and dynamic entities if you need some.
  /// If not use the Default method.
  ///
  /// To use it you will first need to create a client
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
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
  ///   # let audio = include_bytes!("../../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.blocking_speech(audio.to_vec(), options).unwrap();
  /// }
  /// ```
  /// Then we initialize the options with the file format,
  /// and since we do not need dynamic entities in this example we will use the Default values.
  /// We will also import the audio from a file in the repertory.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
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
  ///   let audio = include_bytes!("../../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.blocking_speech(audio.to_vec(), options).unwrap();
  /// }
  /// ```
  /// We can then send the request and wait from results.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
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
  ///   # let audio = include_bytes!("../../../owo/test.mp3");
  ///   #
  ///   let response = owo_client.blocking_speech(audio.to_vec(), options).unwrap();
  /// }
  /// ```
  /// And finally we extract the final response.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
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
  ///   # let audio = include_bytes!("../../../owo/test.mp3");
  ///   #
  ///   # let response = owo_client.blocking_speech(audio.to_vec(), options).unwrap();
  ///   let owo = response.last().unwrap();
  /// }
  /// ```
  pub fn blocking_speech(
    &self,
    audio: Vec<u8>,
    options: SpeechRequest,
  ) -> Result<Vec<SpeechResponse>, WitError> {
    let uwu = self
      .prepare_blocking_post_request("https://api.wit.ai/speech")
      .header("content-type", options.content_type.to_str())
      .body(audio)
      .send()
      .unwrap()
      .text()
      .unwrap();

    let murr: StreamDeserializer<StrRead, Value> =
      Deserializer::from_str(&uwu).into_iter::<Value>();

    prepare_speech_response(murr)
  }
}
