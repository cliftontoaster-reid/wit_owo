//! # Wit.ai Voice API
//!
//! This module provides access to the Wit.ai Voice API endpoints for retrieving
//! available text-to-speech voices and their details.
//!
//! ## Endpoints
//!
//! - **GET** `/voices` - Retrieve a list of available voices grouped by locale.
//! - **GET** `/voices/:voice` - Retrieve details of a specific voice.
//!
//! ## Voice Features
//!
//! Each voice in the Wit.ai TTS system supports various features and styles:
//!
//! - **Styles**: Different vocal styles like "default", "soft", "formal", "fast", "projected"
//! - **Features**: Technical capabilities like "style", "pitch", "speed", "sfx", "viseme_events", "phoneme_events", "word_events"
//! - **Locales**: Voices are available for different locales (en_US, en_CA, en_GB, etc.)
//!
//! ## Quick Start (Async)
//!
//! ```no_run
//! use wit_owo::model::client::WitClient;
//! # use std::env;
//!
//! # #[tokio::main]
//! # #[cfg(feature = "async")]
//! # async fn main() {
//! # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
//! let client = WitClient::new(&token);
//!
//! // List all voices (returns a flat list from all locales)
//! let voices = client.get_voices().await.unwrap();
//! println!("Available voices: {:?}", voices);
//!
//! // Get a specific voice
//! let voice = client.get_voice(&voices[0].name).await.unwrap();
//! println!("Voice details: {:?}", voice);
//! # }
//! # #[cfg(not(feature = "async"))]
//! # fn main() {}
//! ```
//!
//! ## Quick Start (Blocking)
//!
//! ```no_run
//! use wit_owo::model::client::WitClient;
//! # use std::env;
//!
//! # #[cfg(feature = "blocking")]
//! # fn main() {
//! # dotenvy::dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
//! let client = WitClient::new(&token);
//!
//! let voices = client.get_voices_blocking().unwrap();
//! println!("Available voices: {:?}", voices);
//!
//! let voice = client.get_voice_blocking(&voices[0].name).unwrap();
//! println!("Voice details: {:?}", voice);
//! # }
//! # #[cfg(not(feature = "blocking"))]
//! # fn main() {
//! #     println!("Please enable the 'blocking' feature to use synchronous API calls.");
//! # }
//! ```
//!
//! ## Working with Voice Features
//!
//! ```no_run
//! # use wit_owo::model::client::WitClient;
//! # use std::env;
//! # #[cfg(feature = "blocking")]
//! # fn main() {
//! # dotenvy::dotenv().ok();
//! # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
//! let client = WitClient::new(&token);
//! let voices = client.get_voices_blocking().unwrap();
//!
//! // Find voices that support specific features
//! let voices_with_pitch_control: Vec<_> = voices
//!     .iter()
//!     .filter(|v| v.supported_features.contains(&"pitch".to_string()))
//!     .collect();
//!
//! // Find voices with formal style
//! let formal_voices: Vec<_> = voices
//!     .iter()
//!     .filter(|v| v.styles.contains(&"formal".to_string()))
//!     .collect();
//!
//! // Find female voices
//! let female_voices: Vec<_> = voices
//!     .iter()
//!     .filter(|v| v.gender == "female")
//!     .collect();
//! # }
//! # #[cfg(not(feature = "blocking"))]
//! # fn main() {
//! #     println!("Please enable the 'blocking' feature to use synchronous API calls.");
//! # }
//! ```

use crate::constants::BASE_URL;
use crate::{
  error::{ApiError, WitError},
  model::{
    client::WitClient,
    voice::{Voice, VoicesResponse},
  },
};
use url::Url;

impl WitClient {
  /// Retrieves the list of available text-to-speech voices from the Wit.ai API.
  #[cfg(feature = "async")]
  pub async fn get_voices(&self) -> Result<Vec<Voice>, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices"))?;
    let request = self.prepare_get_request(url);

    let response = request.send().await?;
    if response.status().is_success() {
      let voices_response: VoicesResponse = response.json().await?;
      Ok(voices_response.all_voices())
    } else {
      let body: WitError = response.json().await?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves detailed information about a specific voice by name.
  #[cfg(feature = "async")]
  pub async fn get_voice(&self, voice: &str) -> Result<Voice, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices/{voice}"))?;
    let request = self.prepare_get_request(url);

    let response = request.send().await?;
    if response.status().is_success() {
      let voice = response.json().await?;
      Ok(voice)
    } else {
      let body: WitError = response.json().await?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves the list of available text-to-speech voices in a blocking manner.
  #[cfg(feature = "blocking")]
  pub fn get_voices_blocking(&self) -> Result<Vec<Voice>, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices"))?;
    let request = self.prepare_get_blocking(url);

    let response = request.send()?;
    if response.status().is_success() {
      let voices_response: VoicesResponse = response.json()?;
      Ok(voices_response.all_voices())
    } else {
      let body: WitError = response.json()?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves detailed information about a specific voice by name in a blocking manner.
  #[cfg(feature = "blocking")]
  pub fn get_voice_blocking(&self, voice: &str) -> Result<Voice, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices/{voice}"))?;
    let request = self.prepare_get_blocking(url);

    let response = request.send()?;
    if response.status().is_success() {
      let voice = response.json()?;
      Ok(voice)
    } else {
      let body: WitError = response.json()?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves voices grouped by locale from the Wit.ai API.
  #[cfg(feature = "async")]
  pub async fn get_voices_by_locale(&self) -> Result<VoicesResponse, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices"))?;
    let request = self.prepare_get_request(url);

    let response = request.send().await?;
    if response.status().is_success() {
      let voices_response: VoicesResponse = response.json().await?;
      Ok(voices_response)
    } else {
      let body: WitError = response.json().await?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves voices for a specific locale from the Wit.ai API.
  #[cfg(feature = "async")]
  pub async fn get_voices_for_locale(&self, locale: &str) -> Result<Vec<Voice>, ApiError> {
    let voices_response = self.get_voices_by_locale().await?;
    Ok(
      voices_response
        .voices_for_locale(locale)
        .cloned()
        .unwrap_or_default(),
    )
  }

  /// Retrieves voices grouped by locale in a blocking manner.
  #[cfg(feature = "blocking")]
  pub fn get_voices_by_locale_blocking(&self) -> Result<VoicesResponse, ApiError> {
    let url = Url::parse(&format!("{BASE_URL}voices"))?;
    let request = self.prepare_get_blocking(url);

    let response = request.send()?;
    if response.status().is_success() {
      let voices_response: VoicesResponse = response.json()?;
      Ok(voices_response)
    } else {
      let body: WitError = response.json()?;
      Err(ApiError::WitError(body))
    }
  }

  /// Retrieves voices for a specific locale in a blocking manner.
  #[cfg(feature = "blocking")]
  pub fn get_voices_for_locale_blocking(&self, locale: &str) -> Result<Vec<Voice>, ApiError> {
    let voices_response = self.get_voices_by_locale_blocking()?;
    Ok(
      voices_response
        .voices_for_locale(locale)
        .cloned()
        .unwrap_or_default(),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::model::client::WitClient;
  use dotenvy::dotenv;
  use std::env;

  #[tokio::test]
  #[cfg(feature = "async")]
  async fn test_get_voices() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let result = client.get_voices().await;
    if result.is_err() {
      panic!("Error fetching voices: {:?}", result.err());
    }
    let voices = result.unwrap();
    assert!(!voices.is_empty());
  }

  #[tokio::test]
  #[cfg(feature = "async")]
  async fn test_get_voice() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let voices = client.get_voices().await.expect("get_voices failed");
    let voice_name = &voices[0].name;
    let result = client.get_voice(voice_name).await;
    assert!(result.is_ok());
    let voice = result.unwrap();
    assert_eq!(voice.name, *voice_name);
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn test_get_voices_blocking() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let result = client.get_voices_blocking();
    assert!(result.is_ok());
    let voices = result.unwrap();
    assert!(!voices.is_empty());
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn test_get_voice_blocking() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let voices = client
      .get_voices_blocking()
      .expect("get_voices_blocking failed");
    let voice_name = &voices[0].name;
    let result = client.get_voice_blocking(voice_name);
    assert!(result.is_ok());
    let voice = result.unwrap();
    assert_eq!(voice.name, *voice_name);
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn test_get_voices_by_locale_blocking() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
    let client = WitClient::new(&token);

    let voices_response = client
      .get_voices_by_locale_blocking()
      .expect("get_voices_by_locale_blocking failed");

    // Check that we have some locales
    assert!(!voices_response.locales().is_empty());

    // Check that we can get voices for en_US
    let en_us_voices = client
      .get_voices_for_locale_blocking("en_US")
      .expect("get_voices_for_locale_blocking failed");
    assert!(!en_us_voices.is_empty());

    // Verify all returned voices are from en_US locale
    for voice in &en_us_voices {
      assert_eq!(voice.locale, "en_US");
    }
  }
}
