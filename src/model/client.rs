#[cfg(feature = "blocking")]
/// The blocking version of the API
pub mod blocking;

use reqwest::{Client as RequestClient, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

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

#[cfg(test)]
#[cfg(feature = "tokio")]
mod tests {
  use super::*;
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
