use reqwest::{Client as RequestClient, RequestBuilder};
use serde_json::{Value, from_str};

use super::{message::Message, WitError, DynamicEntities};

/// Wit OwO client, the interface that will power your doom creations (hopefully).
pub struct Client {
  token: String,
}

impl Client {
  /// Initializes a wit.ai client.
  ///
  /// Takes the client side token and returns the Wit OwO.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// let rawr = Client::new("owo fancy");
  /// ```
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_owned(),
    }
  }

  fn prepare_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  pub async fn message(&self, text: &str, dyn_entityes: DynamicEntities) -> Result<Message, WitError> {
    let mut hihi = Vec::new();
    let omygod = serde_json::to_string(&dyn_entityes).unwrap();


    hihi.push(("q", text));
    if !dyn_entityes.entities.is_empty() {
      hihi.push(("entities", &omygod));
    }

    let uwu = self.prepare_get_request("https://api.wit.ai/message")
      .query(&hihi).send().await.unwrap();

    let owo = uwu.text().await.unwrap();
    println!("{}", &owo);
    let v: Value = serde_json::from_str(&owo).unwrap();
    match v.as_object().unwrap().get("error") {
      Some(_) => {
        Err(from_str(&owo).unwrap())
      },
      None => {
        Ok(from_str(&owo).unwrap())
      },
    }
  }
}