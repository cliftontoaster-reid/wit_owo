use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::context::Coordinates;

/// A simple value type that can be represented in various formats.
#[derive(Deserialize, Debug, Clone)]
pub enum Value {
  /// A simple value, represented as a string.
  Simple(String),
  /// A simple value, represented as an integer.
  Integer(isize),
  /// A simple value, represented as a float.
  Float(f64),
}

/// A reference to any well‐known entity (books, characters, people, etc.)
#[derive(Deserialize, Debug, Clone)]
pub struct ReferenceValue {
  /// The canonical name (e.g. “Jeff Bezos”, “The Lord of the Rings”).
  pub name: String,
  /// A domain or type label (e.g. “person”, “book”, “movie”).
  pub domain: String,
  /// External IDs (e.g. IMDb, Wikidata).
  #[serde(default)]
  pub external: HashMap<String, String>,
  /// Other metadata.
  #[serde(default)]
  pub attributes: HashMap<String, String>,
}

/// Variants for classifying geographical locations.
///
/// During deserialization, the JSON values for each variant
/// are expected to be lowercase ("locality", "region", "country").
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
  /// A small administrative unit such as a city, town, or neighborhood.
  Locality,
  /// A larger administrative area such as a state or province.
  Region,
  /// A sovereign nation or country.
  Country,
}

/// A detailed representation of a geographical location.
///
/// Includes the location’s display name, its classification,
/// optional timezone and coordinates, plus any external identifiers.
#[derive(Deserialize, Debug, Clone)]
pub struct LocationValue {
  /// The common name of the location (e.g., "Paris", "California").
  pub name: String,
  /// The classification of this location (locality, region, or country).
  pub domain: LocationType,
  /// Optional IANA timezone for this location (e.g., Europe/Paris).
  pub timezone: Option<Tz>,
  /// Optional geographic coordinates (latitude and longitude).
  pub coords: Option<Coordinates>,
  /// Map of external identifiers (e.g., GeoNames ID, OpenStreetMap ID).
  #[serde(default)]
  pub external: HashMap<String, String>,
}

/// Enumerates the supported types of resolved values returned by the system.
/// Each variant wraps a strongly-typed value struct along with its metadata.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResolvedValueType {
  /// A resolved external reference (e.g., a book, movie, or person).
  Reference(ReferenceValue),

  /// A geographical location with detailed metadata.
  Location(LocationValue),
}

#[derive(Deserialize, Debug, Clone)]
/// Represents a simple structured value with an optional type,
/// a resolution grain, and the actual value as a string.
pub struct StructValue {
  /// Optional data type or category of this value (e.g., "number", "date").
  pub type_: Option<String>,

  /// Granularity or precision level for the value (e.g., "day", "hour").
  pub grain: String,

  /// The raw string representation of the value.
  pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
/// Represents a value that spans an interval, with explicit start and end points.
pub struct IntervalValue {
  /// The type or category of the interval (e.g., "time", "date-range").
  pub type_: String,

  /// The start of the interval, as a string (e.g., "2023-01-01T00:00:00Z").
  pub start: String,

  /// The end of the interval, as a string (e.g., "2023-01-07T23:59:59Z").
  pub end: String,
}

#[derive(Deserialize, Debug, Clone)]
/// A wrapper enum for structured values, covering both single values and intervals.
pub enum StructuredValue {
  /// A standalone structured value (with optional type and grain).
  Value(StructValue),

  /// A ranged or interval-based structured value.
  Interval(IntervalValue),
}

/// A container for one or more typed resolution results associated with an entity.
/// Use this when an entity can map to multiple distinct value types.
#[derive(Deserialize, Debug, Clone)]
pub struct ResolvedValue {
  /// A list of typed resolved values, each describing a specific result type.
  pub values: Vec<ResolvedValueType>,
}

/// A normalized value pairing a standardized unit with its underlying data.
///
/// Use this struct to represent values that have been converted into
/// a common unit (for example, converting "5 miles" into "8046.72 meters").
/// The `unit` field denotes the measurement unit, and `value` holds
/// the actual normalized quantity.
#[derive(Deserialize, Debug, Clone)]
pub struct NormanisedValue {
  /// The standardized measurement unit for this value
  /// (e.g., "kg", "m", "s").
  pub unit: String,

  /// The normalized quantity, represented by the generic `Value` enum.
  /// This may be a float, integer, or simple string depending on context.
  pub value: Value,
}

/// A recognized entity extracted from text.
///
/// Entities can have nested sub-entities, optional metadata, and
/// timing or value information for richer analysis.
#[derive(Deserialize, Debug, Clone)]
pub struct Entity {
  /// Unique identifier for this entity.
  pub id: String,

  /// The surface form or name of the entity.
  pub name: String,

  /// The semantic role of the entity (e.g., "person", "location").
  pub role: String,

  /// Start offset in the source text.
  pub start: u32,

  /// End offset in the source text.
  pub end: u32,

  /// The text chunk corresponding to the entity.
  pub body: String,

  /// Confidence score for the entity detection (0.0–1.0).
  pub confidence: f32,

  /// Nested entities grouped by a string key.
  ///
  /// Each key maps to a list of sub-entities.
  pub entities: HashMap<String, Vec<Entity>>,

  /// Wether the entity was detected by a built-in engine from Wit.ai
  /// or from your training data.
  ///
  /// `true` if detected by a built-in engine, `false` otherwise.
  pub suggested: Option<bool>,

  /// Data returned when type is "value".
  pub value: Option<Value>,

  /// Optional unit name of [`value`](Entity::value).
  pub unit: Option<String>,
  /// The optional grain specification for the entity's value.
  pub grain: Option<String>,
  /// Optional domain or category for the entity.
  pub domain: Option<String>,

  /// Optional resolved values of the entity.
  pub resolved: Option<ResolvedValue>,
  /// Optional normalized value of the entity.
  pub normalised: Option<NormanisedValue>,

  /// Optional from value for interval entities.
  pub from: Option<StructValue>,
  /// Optional to value for interval entities.
  pub to: Option<StructValue>,

  /// Optional structured value of the entity.
  #[serde(default)]
  pub values: Vec<StructuredValue>,

  /// Optional time value.
  pub second: Option<usize>,

  /// The type of the entity, renamed from `type` in JSON.
  #[serde(rename = "type")]
  pub type_: String,
}

/// A serializable representation of a dynamic entity value.
///
/// Each `EntityValue` holds a primary `keyword` and
/// a list of `synonyms` used for matching and normalization.
#[derive(Serialize, Debug, Clone)]
pub struct EntityValue {
  /// The canonical keyword for this entity value.
  pub keyword: String,
  /// Alternative forms or synonyms for the keyword.
  pub synonyms: Vec<String>,
}

/// A dynamic entity grouping multiple `EntityValue` items under a name.
///
/// `DynamicEntity` is useful for slot‐filling or enum‐style entity sets
/// that can be built or modified at runtime.
#[derive(Serialize, Debug, Clone)]
pub struct DynamicEntity {
  /// The runtime name of this entity set (skipped during Serde operations).
  #[serde(skip)]
  pub name: String,
  /// The collection of values available for this dynamic entity.
  pub values: Vec<EntityValue>,
}

impl DynamicEntity {
  /// Creates a new `DynamicEntity` with the given name and values.
  pub fn new(name: String) -> Self {
    DynamicEntity {
      name,
      values: Vec::new(),
    }
  }

  /// Adds a new value to the `DynamicEntity`.
  pub fn add_value(&mut self, value: EntityValue) -> &mut Self {
    self.values.push(value);
    self
  }
}
