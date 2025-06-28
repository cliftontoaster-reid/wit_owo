use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the gender of a voice
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceGender {
  /// Female voice
  #[serde(rename = "female")]
  Female,
  /// Male voice
  #[serde(rename = "male")]
  Male,
  /// Neutral or non-binary voice
  #[serde(rename = "neutral")]
  Neutral,
  /// Non-binary voice (alternative name used by API)
  #[serde(rename = "nonbinary")]
  NonBinary,
}

/// Represents a voice available for text-to-speech synthesis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Voice {
  /// The name of the voice (e.g., "wit$Rebecca")
  pub name: String,
  /// The locale of the voice (e.g., "en_US")
  pub locale: String,
  /// The gender of the voice as a string
  pub gender: String,
  /// Available styles for this voice (e.g., "default", "soft", "formal")
  pub styles: Vec<String>,
  /// Features supported by this voice (e.g., "style", "pitch", "speed")
  #[serde(default)]
  pub supported_features: Vec<String>,
}

/// Response structure for the voices API endpoint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicesResponse {
  /// Voices grouped by locale (e.g., "en_US", "en_CA", "en_GB")
  #[serde(flatten)]
  pub voices_by_locale: HashMap<String, Vec<Voice>>,
}

impl VoicesResponse {
  /// Get all voices as a flat vector
  pub fn all_voices(self) -> Vec<Voice> {
    self.voices_by_locale.into_values().flatten().collect()
  }

  /// Get voices for a specific locale
  pub fn voices_for_locale(&self, locale: &str) -> Option<&Vec<Voice>> {
    self.voices_by_locale.get(locale)
  }

  /// Get all available locales
  pub fn locales(&self) -> Vec<&String> {
    self.voices_by_locale.keys().collect()
  }
}

impl Voice {
  /// Check if the voice supports a specific feature
  pub fn supports_feature(&self, feature: &str) -> bool {
    self.supported_features.contains(&feature.to_string())
  }

  /// Check if the voice supports a specific style
  pub fn supports_style(&self, style: &str) -> bool {
    self.styles.contains(&style.to_string())
  }

  /// Get the voice gender as an enum (if it matches known values)
  pub fn gender_enum(&self) -> Option<VoiceGender> {
    match self.gender.as_str() {
      "female" => Some(VoiceGender::Female),
      "male" => Some(VoiceGender::Male),
      "neutral" => Some(VoiceGender::Neutral),
      "nonbinary" => Some(VoiceGender::NonBinary),
      _ => None,
    }
  }

  /// Check if the voice is from a specific locale
  pub fn is_locale(&self, locale: &str) -> bool {
    self.locale == locale
  }
}
