use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::Deserialize;
use std::collections::HashMap;

/// Wit.ai's response format for messages.
#[derive(Deserialize)]
pub struct Message {
  pub text: String,
  pub intents: Vec<Intent>,
  pub traits: HashMap<String, Vec<Trait>>,
  pub entities: HashMap<String, Vec<Entity>>,
}

impl Message {
  pub fn intent(&self) -> Option<&Intent> {
    self.intents.get(0)
  }

  pub fn get_trait(&self, name: &str) -> Option<&Vec<Trait>> {
    self.traits.get(name)
  }
}

#[derive(Deserialize)]
pub struct Intent {
  pub id: String,
  pub name: String,
  pub confidence: f32,
}

#[derive(Deserialize)]
pub struct Entity {
  pub id: String,
  pub name: String,
  pub role: String,
  pub start: i32,
  pub end: i32,
  pub body: String,
  pub confidence: f64,
  pub entities: HashMap<String, Vec<Entity>>,
  pub value: Option<String>,
  #[serde(rename = "type")]
  pub value_type: String,
  pub values: Option<Vec<ValueTypes>>,
}

#[derive(Deserialize)]
pub struct Trait {
  pub id: String,
  pub value: String,
  pub confidence: f64,
}

#[derive(Deserialize)]
pub enum ValueTypes {
  #[serde(rename = "value")]
  Value,

  #[serde(rename = "Interval")]
  Interval(IntervalValue),
}

#[derive(Deserialize)]
pub struct IntervalValue {
  pub from: DateValue,
  pub to: DateValue,
}

#[derive(Deserialize)]
pub struct DateValue {
  pub grain: String,
  pub value: String,
}

impl DateValue {
  /// Parses the ISO 8601 DateTime to a NaiveDateTime.
  pub fn to_utc(&self) -> NaiveDateTime {
    DateTime::parse_from_str(&self.value, "%Y-%m-%dT%H:%M:%S%.3f%:z")
      .unwrap()
      .naive_utc()
  }
  /// Parses the ISO 8601 date time to a Fixe dOffset DateTime.
  pub fn to_datetime(&self) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(&self.value, "%Y-%m-%dT%H:%M:%S%.3f%:z").unwrap()
  }
}

#[derive(Deserialize)]
pub struct WitTrait {
  pub id: String,
  pub value: String,
  pub confidence: f32,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{self as owo_whats_this, model::DynamicEntities};
  use chrono::{Datelike, Timelike};
  use dotenv;

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
  async fn api_message() {
    dotenv::dotenv().ok();
    let owo = owo_whats_this::model::client::Client::new(&dotenv::var("wit_ai").expect("For testing a .env must have wit_ai set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));

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
}
