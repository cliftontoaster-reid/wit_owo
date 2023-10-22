use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::Deserialize;

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
  use chrono::{Datelike, Timelike};

  #[test]
  fn datetime_parse() {
    let rawr = crate::model::values::datetime::DateValue {
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
}
