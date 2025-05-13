use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Trait {
  pub id: String,
  pub value: String,
  pub confidence: f32,
}
