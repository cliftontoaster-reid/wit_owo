use crate::prelude::*;
use serde_json::Value;

#[cfg(feature = "blocking")]
impl Client {
  /// This is used to get the list of all the available voices.
  ///
  /// To start using it we need to initialize a [`Client`].
  /// ```
  /// # use wit_owo::prelude::*;
  /// #
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// let owo_client = Client::new(&token);
  /// # let res = owo_client.get_blocking_voices().unwrap();
  /// ```
  /// The only thing left is to call the function and enjoy the results.
  /// use wit_owo::prelude::*;
  /// ```
  /// # use wit_owo::prelude::*;
  /// #
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo_client = Client::new(&token);
  /// let res = owo_client.get_blocking_voices().unwrap();
  /// ```
  pub fn get_blocking_voices(&self) -> Result<VoicesList, WitError> {
    let uwu = self
      .prepare_blocking_get_request("https://api.wit.ai/voices")
      .send()
      .unwrap();
    let v: Value = uwu.json().unwrap();

    Self::extract(&v)
  }

  /// This is used to get information on one specific voice.
  /// The name argument isn't case sensitive.
  ///
  /// To start using it we need to initialize a [`Client`].
  /// ```
  /// # use wit_owo::prelude::*;
  /// #
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// let owo_client = Client::new(&token);
  /// # let res = owo_client.get_blocking_voice_info("Rebecca").unwrap();
  ///
  /// ```
  /// The only thing left is to call the function and enjoy the results.
  /// use wit_owo::prelude::*;
  /// ```
  /// # use wit_owo::prelude::*;
  /// #
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo_client = Client::new(&token);
  /// let res = owo_client.get_blocking_voice_info("Rebecca").unwrap();
  /// ```
  pub fn get_blocking_voice_info(&self, name: &str) -> Result<Voice, WitError> {
    let uwu = self
      .prepare_blocking_get_request(&format!(
        "https://api.wit.ai/voices/{}",
        name.to_lowercase()
      ))
      .send()
      .unwrap();

    let v: Value = uwu.json().unwrap();

    Self::extract(&v)
  }
}

#[cfg(test)]
mod tests {
  use crate::model::voices::VoiceGender::Female;
  use crate::prelude::*;
  use std::env;

  #[tokio::test]
  #[cfg(feature = "blocking")]
  async fn api_blocking_voice_info() {
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
