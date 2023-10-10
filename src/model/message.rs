use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wit.ai's response format for messages.
#[derive(Deserialize)]
pub struct Message {
  pub text: String,
  pub intent: Vec<Intent>,
  pub entities: HashMap<String, Entity>,
}

/// Define an entity, usually referred as the meaning of a message.
#[derive(Deserialize)]
pub struct Intent {
  id: String,
  name: String,
  confidence: f32,
}

/// Define an entity, usually referred as an argument comming from a message.
#[derive(Deserialize)]
pub struct Entity {
  pub id: String,
  pub name: String,
  pub role: String,
  pub start: i32,
  pub end: i32,
  pub body: String,
  pub confidence: i64,
  pub entities: HashMap<String, Entity>,
  pub value: Option<String>,
  pub root_type: String,
  pub values: Option<Vec<ValueTypes>>,
}

/// A list of a few types, yes I was lazy and didn't want to implement the x different values structures including all the ones inside entity.
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
  /// The date is an ISO 8601 DateTime string.
  /// A `NaiveDateTime` can be parsed automatically using `utc()`
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

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{Datelike, Timelike};

  fn datetime_parse() {
    let rawr = DateValue {
      grain: "day".parse().unwrap(),
      value: "2020-05-12T00:00:00.000-07:00".parse().unwrap(),
    };

    let owo = rawr.to_utc();
    assert_eq!(owo.day(), 12);
    assert_eq!(owo.month(), 5);
    assert_eq!(owo.year(), 2020);

    assert_eq!(owo.hour(),);
    assert_eq!(owo.month(), 5);
    assert_eq!(owo.year(), 2020);
    assert_eq!(owo.year(), 2020);
  }
}
