#[cfg(feature = "blocking")]
/// The blocking version of the API
pub mod blocking;
#[cfg(feature = "lingua")]
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use reqwest::{Client as RequestClient, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};
use std::collections::HashMap;

use super::WitError;

/// Wit OwO client, the interface that will power your doom creations (hopefully).
pub struct Client {
  /// The wit.ai's token.
  token: String,
}

impl Client {
  /// Initializes a wit.ai client.
  ///
  /// Takes the client side token and returns the Wit OwO.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// let rawr = Client::new("owo fancy key");
  /// ```
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_owned(),
    }
  }

  /// It's just an easier and less painful way to check if there were any errors.
  pub fn extract<T: DeserializeOwned>(v: &Value) -> Result<T, WitError> {
    #[cfg(test)]
    println!("{:?}", v);
    match v.as_object().unwrap().get("error") {
      Some(_) => Err(from_value(v.clone()).unwrap()),
      None => Ok(from_value::<T>(v.clone()).unwrap()),
    }
  }
}

#[cfg(feature = "async")]
impl Client {
  /// It prepares a get request with bearer auth.  
  pub fn prepare_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a post request with bearer auth.  
  pub fn prepare_post_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .post(uri)
      .bearer_auth(self.token.clone())
  }
}

#[cfg(feature = "lingua")]
/// A multi lingual client to handle multiple languages without working much.
pub struct MultiLingualClient {
  clients: HashMap<Language, Client>,
  lingua: LanguageDetector,
}

#[cfg(feature = "lingua")]
impl MultiLingualClient {
  /// Initializes a wit.ai multilingual client.
  ///
  /// Takes a HashMap with for each language to support,
  /// its token for a wit.ai application and returns the Wit OwO [`MultiLingualClient`].
  ///
  /// ```
  /// use std::collections::HashMap;
  /// use wit_owo::prelude::*;
  /// use lingua::Language;
  /// use lingua::Language::{English, French};
  /// # let token_fr = "owo fancy";
  /// # let token_en = "owo fancy";
  ///
  /// let mut owo: HashMap<Language, String> = HashMap::new();
  /// owo.insert(French, token_fr.to_owned());
  /// owo.insert(English, token_en.to_owned());
  ///
  /// let rawr = MultiLingualClient::new(&owo);
  /// ```
  pub fn new(tokens: &HashMap<Language, String>) -> Self {
    if tokens.is_empty() {
      panic!("Tokens are required.");
    }
    let languages: Vec<Language> = tokens.keys().clone().copied().collect();

    let mut uwu = Self {
      clients: Default::default(),
      lingua: LanguageDetectorBuilder::from_languages(&languages).build(),
    };

    for (k, i) in tokens {
      uwu.clients.insert(*k, Client::new(i));
    }

    uwu
  }

  /// Returns the client for the given language.
  pub fn get(&self, language: &Language) -> Option<&Client> {
    self.clients.get(language)
  }

  /// Guesses the language of the given text and returns the client associated with it if available.
  pub fn guess_language(&self, text: &str) -> Option<&Client> {
    match self.lingua.detect_language_of(text) {
      None => None,
      Some(uwu) => self.clients.get(&uwu),
    }
  }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
  use super::*;
  use crate::model::DynamicEntities;
  use std::env;

  #[tokio::test]
  #[should_panic]
  async fn message_over() {
    dotenv::dotenv().ok();
    let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

    owo
      .message("Lorem ipsum dolor sidi amas doloro, tiel in amet amas esti loŝvortaĵo. Subskribo al bonaj gazetoj, eklaboro kaj brilas. Kaj aliaj, kaj la lingvo de Esp, estas ne nur la lingvo de Esp. En bona penso ĉiu homo batalas, dum brila, asimilas konataj ajn. Skatolo uzas kompleksajn skribaĵojn, ĉiu efiko estas malfacila. La forta manko de akiri ŝaltukon en brilanta ĉielo. Ne surtera forto, kaj penso estis, sed saĝa paŝo. La malnova ĉeestas sur la peza tavolo. Pafu ŝtonon al kiel libera, tiu tiel ankaŭ ŝajnas. Nur tre aperas super, aŭ tiuj mi. La paco de la instruado, ke alia lingvo kaj ekonomiaj demandoj. Kaj la lingvo de Esperanto, en sia forteco estas laŭdo kaj muziko. Post longa tago, ni estos la lastaj vivuloj. Tamen, kiam la malpli facile, kaj ekonomiaj demandoj.", DynamicEntities::default())
      .await
      .unwrap();
  }
}
