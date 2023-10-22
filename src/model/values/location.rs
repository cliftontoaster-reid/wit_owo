use super::super::Coords;
use serde::Deserialize;
use std::collections::HashMap;

/// Captures free text that's a typical location, place or address like `350 Cambridge Ave Palo Alto, 925 Alma Street, LAX`, and `Sausalito, CA`.
///
/// For English, we try to resolve locations that are a Locality (e.g. a district or city), Region or Country. We currently do not support address resolution.
#[derive(Deserialize)]
pub struct Location {
  /// The name of that location.
  pub name: String,
  /// Can be one of the following.
  /// - `locality`
  /// - `region`
  /// - `country`
  pub domain: Domains,
  /// A valid timezone e.g. "America/Los_Angeles"
  pub timezone: Option<String>,
  /// Possible GPS coordinates.
  pub coords: Option<Coords>,
  /// External data id/names to use with websites like Wikipedia.
  pub external: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
/// How precise the location is, note that adresses are not supported yet.
pub enum Domains {
  #[serde(rename = "Locality")]
  /// A district, a city.
  Locality,
  #[serde(rename = "Region")]
  /// Regions, states.
  Region,
  #[serde(rename = "Country")]
  /// Countries.
  Country,
}
