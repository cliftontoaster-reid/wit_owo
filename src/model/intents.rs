use serde::Deserialize;

/// Represents an intent recognized by the Wit.ai API.
///
/// Intents are the actions or goals that the user might want to achieve with their message.
/// Each intent has an ID, a name, and a confidence score indicating how likely it is that the intent
/// was correctly identified.
#[derive(Deserialize, Debug, Clone)]
pub struct Intent {
  /// Unique identifier for this intent.
  pub id: String,
  /// The name of the intent.
  pub name: String,
  /// The confidence score for the intent detection (`0.0–1.0`) stored as a 16 bit float.
  pub confidence: f32,
}

impl PartialEq for Intent {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.name == other.name
  }
}

impl Eq for Intent {}
