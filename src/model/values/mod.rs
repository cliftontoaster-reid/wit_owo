/// The time values related structs.
pub mod datetime;
/// The location values related structs.
pub mod location;

use datetime::IntervalValue;
use location::Location;
use serde::Deserialize;

#[derive(Deserialize)]
/// Represents the multiple values we know the API sends back.
pub enum ValueTypes {
  /// An interval between two dates.
  ///
  /// See [`IntervalValue`]
  Interval(IntervalValue),
  /// A location.
  ///
  /// See [`Location`]
  Location(Location),
}
