use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod client;
pub mod message;

#[derive(Debug, Deserialize)]
pub struct WitError {
  pub error: String,
  pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DynamicEntities {
  pub entities: HashMap<String, DynamicEntitie>,
}
impl DynamicEntities {
  pub fn add_entity(&mut self, entity: (String, HashMap<String, Vec<String>>)) -> &mut DynamicEntities {
    for (k, i) in entity.1 {
      self.entities.insert(entity.0.clone(),
        DynamicEntitie { keyword: k, synonyms: i }
      );
    }

    self
  }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DynamicEntitie {
  pub keyword: String,
  pub synonyms: Vec<String>
}