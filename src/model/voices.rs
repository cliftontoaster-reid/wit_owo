#[cfg(feature = "blocking")]
/// The blocking version of the voices APIs.
pub mod blocking;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
/// A list of voices.
pub struct VoicesList(HashMap<String, Vec<Voice>>);

impl VoicesList {
  /// Get a list of voices by locale, easier, and more concisely.
  pub fn get(&self, key: &str) -> Option<&Vec<Voice>> {
    self.0.get(key)
  }
}

#[derive(Deserialize, PartialEq, Debug)]
/// An available voice.
pub struct Voice {
  /// The voice's name.
  pub name: String,
  /// The locale, and so accent of the voice.
  pub locale: String,
  /// The voice's gender.
  pub gender: VoiceGender,
  /// The voice's variations.
  pub styles: Vec<String>,
}

/// The gender of the voice.
#[derive(Deserialize, PartialEq, Debug)]
pub enum VoiceGender {
  #[serde(rename = "male")]
  /// Rather masculine voice.
  Male,
  #[serde(rename = "female")]
  /// Rather feminine voice.
  Female,
  #[serde(rename = "nonbinary")]
  /// None of those.
  NonBinary,
}

#[cfg(feature = "async")]
impl Client {
  /// This is used to get the list of all the available voices.
  ///
  /// To start using it we need to initialize a [`Client`].
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # use std::env;
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///
  ///   let owo_client = Client::new(&token);
  ///
  ///   # let res = owo_client.get_voices().await.unwrap();
  /// }
  /// ```
  /// The only thing left is to call the function and enjoy the results.
  /// use wit_owo::prelude::*;
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # use std::env;
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///
  ///   # let owo_client = Client::new(&token);
  ///
  ///   let res = owo_client.get_voices().await.unwrap();
  /// }
  /// ```
  pub async fn get_voices(&self) -> Result<VoicesList, WitError> {
    let uwu = self
      .prepare_get_request("https://api.wit.ai/voices")
      .send()
      .await
      .unwrap();
    let v: Value = uwu.json().await.unwrap();

    Self::extract(&v)
  }

  /// This is used to get information on one specific voice.
  /// The name argument isn't case sensitive.
  ///
  /// To start using it we need to initialize a [`Client`].
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # use std::env;
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///
  ///   let owo_client = Client::new(&token);
  ///
  ///   # let res = owo_client.get_voice_info("Rebecca").await.unwrap();
  /// }
  /// ```
  /// The only thing left is to call the function and enjoy the results.
  /// use wit_owo::prelude::*;
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   # use std::env;
  ///   # dotenv::dotenv().ok();
  ///   # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  ///
  ///   # let owo_client = Client::new(&token);
  ///
  ///   let res = owo_client.get_voice_info("Rebecca").await.unwrap();
  /// }
  /// ```
  pub async fn get_voice_info(&self, name: &str) -> Result<Voice, WitError> {
    let uwu = self
      .prepare_get_request(&format!(
        "https://api.wit.ai/voices/{}",
        name.to_lowercase()
      ))
      .send()
      .await
      .unwrap();

    let v: Value = uwu.json().await.unwrap();

    Self::extract(&v)
  }
}

#[cfg(test)]
mod tests {
  use crate::model::voices::VoiceGender::Female;
  use crate::prelude::*;
  use std::env;

  #[tokio::test]
  #[cfg(feature = "async")]
  async fn api_voice_info() {
    dotenv::dotenv().ok();

    let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));

    let owo_client = Client::new(&token);

    let res = owo_client.get_voice_info("Rebecca").await.unwrap();

    assert_eq!(res.gender, Female);
    assert_eq!(res.name, "Rebecca");
    assert_eq!(res.locale, "en_US");
    assert_eq!(
      res.styles,
      vec!["default", "soft", "formal", "fast", "projected"]
    );
  }
}
