use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

  /// Optional raw value assigned to the entity.
  ///
  /// More value types will be added in the future.
  /// Currently, it is only in the test phase as the
  /// library is being rebuilt.
  pub value: Option<String>,

  /// The type of the entity, renamed from `type` in JSON.
  #[serde(rename = "type")]
  pub type_: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct EntityValue {
  pub keyword: String,
  pub synonyms: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct DynamicEntity {
  #[serde(skip)]
  pub name: String,
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
