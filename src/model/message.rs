use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::Deserialize;
use std::collections::HashMap;

/// Wit.ai's response format for messages.
#[derive(Deserialize)]
pub struct Message {
  /// The original text, why? Don't ask.
  pub text: String,
  /// The list of intents, you should use [`Message::intent`].
  /// Basically it's the possible meanings of the text.
  pub intents: Vec<Intent>,
  /// The object of every detected trait.
  /// It's a way to describe how the text sounds.
  /// Like greetings would be true if the message is something like.
  /// > Hi, could you X?
  pub traits: HashMap<String, Vec<Trait>>,
  /// The object of every detected entity.
  /// Basically it's possible detected arguments in the text like a name or a date, etc.
  pub entities: HashMap<String, Vec<Entity>>,
}

impl Message {
  /// Get a trait by name and usage.
  ///
  /// # Arguments
  ///
  /// None
  ///
  /// # Returns
  ///
  /// - The specified intent if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let uwu = owo
  ///     .message("OwO what's this", DynamicEntities::default())
  ///     .await
  ///     .unwrap();
  ///
  ///   let intent = uwu.intent().unwrap();
  /// }
  /// ```
  pub fn intent(&self) -> Option<&Intent> {
    self.intents.get(0)
  }

  /// Get a trait by name and usage.
  ///
  /// # Arguments
  ///
  /// - `name` - The name and usage of the trait.
  ///
  /// # Returns
  ///
  /// - The specified list of entities if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let uwu = owo
  ///     .message("OwO what's this", DynamicEntities::default())
  ///     .await
  ///     .unwrap();
  ///
  ///   let trait_name = &uwu.get_trait("sexy").unwrap().get(0).unwrap().value;
  /// }
  /// ```
  pub fn get_trait(&self, name: &str) -> Option<&Vec<Trait>> {
    self.traits.get(name)
  }

  /// Get an entity by name and usage, in the format 'name:usage'.
  ///
  /// # Arguments
  ///
  /// - `name` - The name and usage of the entity in the format 'name:usage'.
  ///
  /// # Returns
  ///
  /// - The specified list of entities if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let uwu = owo
  ///     .message("OwO what's this", DynamicEntities::default())
  ///     .await
  ///     .unwrap();
  ///
  ///   let trait_name = &uwu.get_entity("owo:owo").unwrap().get(0).unwrap().value;
  /// }
  /// ```
  pub fn get_entity(&self, name: &str) -> Option<&Vec<Entity>> {
    self.entities.get(name)
  }
}

#[derive(Deserialize)]
/// Basically it's the possible meanings of the text.///
pub struct Intent {
  /// A random string, don't bother trying to use it.
  pub id: String,
  /// The name of that intent, usually used to identify it.
  pub name: String,
  /// How much from 0 to 1 the computer things it's true.
  pub confidence: f32,
}

#[derive(Deserialize)]
/// Basically it's possible detected arguments in the text like a name or a date, etc.
pub struct Entity {
  /// A random string, don't bother trying to use it.
  pub id: String,
  /// The name of that entity, usually used to identify it.
  pub name: String,
  /// The role in case you are smart and organize multiple entities for a single intent.
  pub role: String,
  /// Where the entity starts in the text.
  pub start: u16,
  /// Where the entity ends in the text.
  pub end: u16,
  /// The raw content of the entity.
  pub body: String,
  /// How much from 0 to 1 the computer things it's true.
  pub confidence: f64,
  /// In case entities has other entities inside... I never had to deal with it.
  pub entities: HashMap<String, Vec<Entity>>,
  /// The value IF it's a simple string.
  pub value: Option<String>,
  #[serde(rename = "type")]
  /// A string representing the data type.
  pub value_type: String,
  /// For now just a time interval value sadly.
  pub values: Option<Vec<ValueTypes>>,
}

#[derive(Deserialize)]
/// It's a way to describe how the text sounds.
/// Like greetings would be true if the message is something like.
/// > Hi, could you X?
pub struct Trait {
  /// A random string, but here you'll have to use it, or you're smart and use the key in the HashMap.
  pub id: String,
  /// The string value of it.
  pub value: String,
  /// How much from 0 to 1 the computer things it's true.
  pub confidence: f64,
}

#[derive(Deserialize)]
/// Represents the multiple values we know the API sends back.
pub enum ValueTypes {
  #[serde(rename = "Interval")]
  /// An interval between two dates.
  Interval(IntervalValue),
}

#[derive(Deserialize)]
/// An interval between two dates.
pub struct IntervalValue {
  /// The from.
  pub from: DateValue,
  /// The to.
  pub to: DateValue,
}

#[derive(Deserialize)]
/// A date and time for Wit.AI.
pub struct DateValue {
  /// Represents how precise the time actually.
  pub grain: String,
  /// An ISO 8601 DateTime.
  pub value: String,
}

impl DateValue {
  /// Parses the ISO 8601 DateTime to a NaiveDateTime.
  ///
  /// # Arguments
  ///
  /// None.
  ///
  /// # Returns
  ///
  /// - Chrono's UTC datetime.
  ///
  pub fn to_utc(&self) -> NaiveDateTime {
    DateTime::parse_from_str(&self.value, "%Y-%m-%dT%H:%M:%S%.3f%:z")
      .unwrap()
      .naive_utc()
  }

  /// Parses the ISO 8601 date time to a Fixed Offset DateTime.
  ///
  /// # Arguments
  ///
  /// None.
  ///
  /// # Returns
  ///
  /// - Chrono's datetime with the data's timezone as own..
  ///
  pub fn to_datetime(&self) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(&self.value, "%Y-%m-%dT%H:%M:%S%.3f%:z").unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{self as owo_whats_this, model::DynamicEntities};
  use chrono::{Datelike, Timelike};
  use dotenv;
  use std::env;

  #[test]
  fn datetime_parse() {
    let rawr = DateValue {
      grain: "day".parse().unwrap(),
      value: "2020-05-12T07:38:23.000+07:00".parse().unwrap(),
    };

    let owo = rawr.to_datetime();
    assert_eq!(owo.day(), 12);
    assert_eq!(owo.month(), 5);
    assert_eq!(owo.year(), 2020);

    assert_eq!(owo.hour(), 7);
    assert_eq!(owo.minute(), 38);
    assert_eq!(owo.second(), 23);

    let uwu = rawr.to_utc();
    assert_eq!(uwu.day(), 12);
    assert_eq!(uwu.month(), 5);
    assert_eq!(uwu.year(), 2020);

    assert_eq!(uwu.hour(), 0);
    assert_eq!(uwu.minute(), 38);
    assert_eq!(uwu.second(), 23);
  }

  #[tokio::test]
  #[cfg(feature = "tokio")]
  async fn api_message() {
    dotenv::dotenv().ok();
    let owo = owo_whats_this::model::client::Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

    let uwu = owo
      .message("OwO what's this", DynamicEntities::default())
      .await
      .unwrap();
    assert_eq!(uwu.intent().unwrap().name, "uwu");
    assert_eq!(
      uwu.entities.get("owo:owo").unwrap().get(0).unwrap().value,
      Some("what's this".to_string())
    );
    assert_eq!(uwu.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn blocking_api_message() {
    dotenv::dotenv().ok();
    let owo = owo_whats_this::model::client::Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

    let uwu = owo
      .blocking_message("OwO what's this", DynamicEntities::default())
      .unwrap();
    assert_eq!(uwu.intent().unwrap().name, "uwu");
    assert_eq!(
      uwu.entities.get("owo:owo").unwrap().get(0).unwrap().value,
      Some("what's this".to_string())
    );
    assert_eq!(uwu.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  }
}
