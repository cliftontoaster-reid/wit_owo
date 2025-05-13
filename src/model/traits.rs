use serde::Deserialize;

/// A struct representing a trait extracted from a message by the Wit.ai API.
///
/// A trait is a specific characteristic or feature that the API identifies in the input message.
/// Each trait having an ID, a value, and a confidence score.
#[derive(Deserialize, Debug, Clone)]
pub struct Trait {
  /// The unique identifier for the trait.
  pub id: String,
  /// The name of the trait.
  pub value: String,
  /// The confidence score of the trait, indicating how certain the API is about this trait being present in the message.
  pub confidence: f32,
}
