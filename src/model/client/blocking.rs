use super::Client;
use crate::constants::MAX_MESSAGE_LENGTH;
use crate::model::{DynamicEntities, WitError};
use crate::prelude::*;
use reqwest::blocking::{Client as RequestClient, RequestBuilder};
use serde_json::de::StrRead;
use serde_json::{from_str, Deserializer, StreamDeserializer, Value};

impl Client {
  /// It prepares a get request with bearer auth.
  fn prepare_blocking_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a post request with bearer auth.  
  fn prepare_blocking_post_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .post(uri)
      .bearer_auth(self.token.clone())
  }

  /// It takes the audio to transcribe then analyse and dynamic entities if you need some.
  /// If not use the default method.
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
  /// and since we do not need dynamic entities in this example we will use the default values.
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

  /// It takes the text to analyse and dynamic entities if you need some.
  /// If not use the default method.
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
  ///   # let entities = DynamicEntities::default();
  ///   # let text = "OwO what's this";
  ///   #
  ///   # let response = owo_client.blocking_message(text, entities).unwrap();
  ///   #
  ///   # assert_eq!(response.intent().unwrap().name, "uwu");
  ///   # assert_eq!(
  ///   #   response.entities.get("owo:owo").unwrap().get(0).unwrap().value,
  ///   #   Some("what's this".to_string())
  ///   # );
  ///   # assert_eq!(response.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  /// }
  /// ```
  /// Then we prepare the options, with the default Dynamic Entities because we don't need it.
  /// For more informations please visit [`DynamicEntities`].
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   # let owo_client = Client::new(&token);
  ///   #
  ///   let entities = DynamicEntities::default();
  ///   let text = "OwO what's this";
  ///   #
  ///   # let response = owo_client.blocking_message(text, entities).unwrap();
  ///   #
  ///   # assert_eq!(response.intent().unwrap().name, "uwu");
  ///   # assert_eq!(
  ///   #   response.entities.get("owo:owo").unwrap().get(0).unwrap().value,
  ///   #   Some("what's this".to_string())
  ///   # );
  ///   # assert_eq!(response.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  /// }
  /// ```
  /// Then we send the request and it's done.
  /// ```
  /// use wit_owo::prelude::*;
  /// # use std::env;
  ///
  /// fn main() {
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///   # let owo_client = Client::new(&token);
  ///   #
  ///   # let entities = DynamicEntities::default();
  ///   # let text = "OwO what's this";
  ///   #
  ///   let response = owo_client.blocking_message(text, entities).unwrap();
  ///   #
  ///   # assert_eq!(response.intent().unwrap().name, "uwu");
  ///   # assert_eq!(
  ///   #   response.entities.get("owo:owo").unwrap().get(0).unwrap().value,
  ///   #   Some("what's this".to_string())
  ///   # );
  ///   # assert_eq!(response.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  /// }
  /// ```
  /// Voila, bravo!
  pub fn blocking_message(
    &self,
    text: &str,
    dyn_entities: DynamicEntities,
  ) -> Result<Message, WitError> {
    if text.len() > MAX_MESSAGE_LENGTH {
      return Err(WitError {
        error: format!(
          "The message with a length of {} is greater than the max limit {}",
          text.len(),
          MAX_MESSAGE_LENGTH
        )
        .to_string(),
        code: "INTERNAL_MESSAGE_LEN_OVER_LIMIT".to_string(),
      });
    }

    let mut hihi = Vec::new();
    let omg = serde_json::to_string(&dyn_entities).unwrap();

    hihi.push(("q", text));
    if !dyn_entities.entities.is_empty() {
      hihi.push(("entities", &omg));
    }

    let uwu = self
      .prepare_blocking_get_request("https://api.wit.ai/message")
      .query(&hihi)
      .send()
      .unwrap();

    let owo = uwu.text().unwrap();
    println!("{}", &owo);
    let v: Value = from_str(&owo).unwrap();
    Self::extract(v, &owo)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[tokio::test]
  #[should_panic]
  async fn blocking_message_over() {
    dotenv::dotenv().ok();
    let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

    owo
            .blocking_message("Lorem ipsum dolor sidi amas doloro, tiel in amet amas esti loŝvortaĵo. Subskribo al bonaj gazetoj, eklaboro kaj brilas. Kaj aliaj, kaj la lingvo de Esp, estas ne nur la lingvo de Esp. En bona penso ĉiu homo batalas, dum brila, asimilas konataj ajn. Skatolo uzas kompleksajn skribaĵojn, ĉiu efiko estas malfacila. La forta manko de akiri ŝaltukon en brilanta ĉielo. Ne surtera forto, kaj penso estis, sed saĝa paŝo. La malnova ĉeestas sur la peza tavolo. Pafu ŝtonon al kiel libera, tiu tiel ankaŭ ŝajnas. Nur tre aperas super, aŭ tiuj mi. La paco de la instruado, ke alia lingvo kaj ekonomiaj demandoj. Kaj la lingvo de Esperanto, en sia forteco estas laŭdo kaj muziko. Post longa tago, ni estos la lastaj vivuloj. Tamen, kiam la malpli facile, kaj ekonomiaj demandoj.", DynamicEntities::default())
            .unwrap();
  }
}
