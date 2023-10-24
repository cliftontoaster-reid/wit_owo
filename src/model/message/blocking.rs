use crate::constants::check_message;
use crate::prelude::*;
use serde_json::{from_str, Value};

impl Client {
  /// It takes the text to analyse and dynamic entities if you need some.
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
  /// Then we prepare the options, with the Default Dynamic Entities because we don't need it.
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
    check_message(text)?;

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
    Self::extract(&v)
  }
}
