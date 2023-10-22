use super::super::Coords;
use serde::Deserialize;

/// Captures free text that's a typical location, place or address like `350 Cambridge Ave Palo Alto, 925 Alma Street, LAX`, and `Sausalito, CA`.
///
/// For English, we try to resolve locations that are a Locality (e.g. a district or city), Region or Country. We currently do not support address resolution.
#[derive(Deserialize)]
pub struct Location {
  pub name: String,
  pub domain: Domains,
  pub timezone: Option<String>,
  pub coords: Option<Coords>,
}

#[derive(Deserialize)]
pub enum Domains {
  #[serde(rename = "Locality")]
  Locality,
  #[serde(rename = "Region")]
  Region,
  #[serde(rename = "Country")]
  Country,
}
