use serde::Deserialize;

use crate::constants::BASE_URL;
use crate::error::ApiError;
use crate::prelude::{AudioSource, Encoding, Entity, Intent, Speech, Trait};
use serde_json;
use std::collections::HashMap;
use url::Url;

use super::{context::Context, entities::DynamicEntity};

/// Represents a speech query for audio transcription or intent recognition.
#[derive(Debug, Default)]
pub struct SpeechQuery {
  /// The audio source to transcribe, either buffered or streaming.
  pub data: AudioSource,

  /// Encoding format of the audio (e.g., "pcm", "mp3", "wav").
  pub encoding: Encoding,

  /// Optional raw encoding type (e.g., "pcm", "mulaw").
  pub raw_encoding: Option<String>,

  /// Optional bit depth of the audio samples (e.g., 8, 16, 24, 32).
  pub bits: Option<u8>,

  /// Optional sample rate in Hertz (e.g., 8000, 16000, 44100).
  pub sample_rate: Option<u16>,

  /// Optional endianness of the audio data.
  /// `true` for little-endian, `false` for big-endian.
  pub endian: Option<bool>,

  /// Optional tag to be associated with the message.
  pub tag: Option<String>,

  /// Optional limit on the number of intents to return.
  pub n: Option<u8>,

  /// The dynamic entity array to be used in the request.
  pub dynamic_entities: Option<Vec<DynamicEntity>>,

  /// The context for the speech query.
  pub context: Option<Context>,
}

impl SpeechQuery {
  /// Creates a new `SpeechQuery` with the specified encoding and audio data.
  pub fn new(encoding: Encoding, data: AudioSource) -> Self {
    Self {
      encoding,
      data,
      ..Default::default()
    }
  }

  /// Sets the raw encoding type for raw audio data.
  pub fn with_raw_encoding(mut self, raw_encoding: String) -> Self {
    self.raw_encoding = Some(raw_encoding);
    self
  }

  /// Sets the bit depth of the audio samples.
  pub fn with_bits(mut self, bits: u8) -> Self {
    self.bits = Some(bits);
    self
  }

  /// Sets the sample rate in Hertz.
  pub fn with_sample_rate(mut self, sample_rate: u16) -> Self {
    self.sample_rate = Some(sample_rate);
    self
  }

  /// Sets the endianness of the audio data.
  pub fn with_endian(mut self, endian: bool) -> Self {
    self.endian = Some(endian);
    self
  }

  /// Sets the tag to be associated with the message.
  pub fn with_tag(mut self, tag: String) -> Self {
    self.tag = Some(tag);
    self
  }

  /// Sets the limit on the number of intents to return.
  pub fn with_n(mut self, n: u8) -> Self {
    self.n = Some(n);
    self
  }

  /// Sets the dynamic entities to be used in the request.
  pub fn with_dynamic_entities(mut self, dynamic_entities: Vec<DynamicEntity>) -> Self {
    self.dynamic_entities = Some(dynamic_entities);
    self
  }

  /// Sets the context for the speech query.
  pub fn with_context(mut self, context: Context) -> Self {
    self.context = Some(context);
    self
  }

  /// Converts the `SpeechQuery` into a `Url` for the Wit.ai API's /speech endpoint.
  ///
  /// This method constructs the URL with query parameters based on the fields of the `SpeechQuery`.
  /// Note: The audio data itself is sent in the request body, not as a URL parameter.
  pub(crate) fn to_url(&self) -> Result<Url, ApiError> {
    let mut params: Vec<(String, String)> = Vec::new();

    if let Some(tag) = &self.tag {
      params.push(("tag".to_string(), tag.clone()));
    }
    if let Some(n) = self.n {
      params.push(("n".to_string(), n.to_string()));
    }
    if let Some(context) = &self.context {
      if let Ok(context_json) = serde_json::to_string(context) {
        params.push((
          "context".to_string(),
          urlencoding::encode(&context_json).into_owned(),
        ));
      }
    }
    if let Some(dynamic_entities) = &self.dynamic_entities {
      // Similar to MessageQuery, serialize dynamic_entities to JSON
      // and then URL-encode it.
      let mut entities_map: HashMap<String, serde_json::Value> = HashMap::new();
      for entity in dynamic_entities {
        let name = entity.name.clone();
        // Assuming DynamicEntity is serializable
        if let Ok(data) = serde_json::to_value(entity) {
          entities_map.insert(name, data);
        }
      }
      if !entities_map.is_empty() {
        if let Ok(json_raw) = serde_json::to_string(&entities_map) {
          let json_safe = urlencoding::encode(&json_raw);
          params.push(("entities".to_string(), json_safe.into_owned()));
        }
      }
    }

    Url::parse_with_params(&format!("{BASE_URL}speech"), params).map_err(|e| e.into())
  }
}

impl std::fmt::Display for SpeechQuery {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.encoding {
      Encoding::Wav => write!(f, "audio/wav"),
      Encoding::Mp3 => write!(f, "audio/mpeg3"), // Or audio/mpeg if mpeg3 is not standard
      Encoding::Ogg => write!(f, "audio/ogg"),
      Encoding::Ulaw => write!(f, "audio/ulaw"), // Consider if sample_rate needs to be part of this
      Encoding::Raw => {
        let mut content_type = String::from("audio/raw");
        if let Some(raw_encoding) = &self.raw_encoding {
          content_type.push_str(&format!(";encoding={raw_encoding}"));
        }
        if let Some(bits) = self.bits {
          content_type.push_str(&format!(";bits={bits}"));
        }
        if let Some(sample_rate) = self.sample_rate {
          content_type.push_str(&format!(";rate={sample_rate}"));
        }
        if let Some(endian) = self.endian {
          let endian_str = if endian { "little" } else { "big" };
          content_type.push_str(&format!(";endian={endian_str}"));
        }
        write!(f, "{content_type}")
      }
    }
  }
}

/// The type of speech event returned by the API, indicating whether the data
/// represents a transcription or an understanding (NLU), and whether it’s
/// partial (intermediate) or final (complete).
#[derive(Debug, Clone, Deserialize)]
pub enum SpeechResponse {
  /// A streaming intermediate transcription result from speech recognition.
  /// This may update as more audio is processed.
  #[serde(rename = "PARTIAL_TRANSCRIPTION")]
  PartialTranscription(SpeechTranscription),

  /// The completed transcription result for the provided audio segment.
  /// No further updates will be sent for this segment.
  #[serde(rename = "FINAL_TRANSCRIPTION")]
  FinalTranscription(SpeechTranscription),

  /// A streaming intermediate understanding result from natural language
  /// processing (e.g., intents/entities). This may refine as more context
  /// becomes available.
  #[serde(rename = "PARTIAL_UNDERSTANDING")]
  PartialUnderstanding(SpeechUnderstanding),

  /// The final understanding result (intents/entities) for the audio segment.
  /// No further updates will be sent for this segment.
  #[serde(rename = "FINAL_UNDERSTANDING")]
  FinalUnderstanding(SpeechUnderstanding),
}

/// Represents a speech transcription event returned by the API when spoken
/// audio is converted into raw text.
#[derive(Debug, Clone, Deserialize)]
pub struct SpeechTranscription {
  /// The raw transcription text produced by the speech recognition engine.
  pub text: String,

  /// Speech metadata (e.g., timing, confidence scores) associated with this transcription.
  pub speech: Option<Speech>,
}

/// Represents a speech understanding event returned by the API when
/// transcribed audio is parsed for intents, entities, and traits.
#[derive(Debug, Clone, Deserialize)]
pub struct SpeechUnderstanding {
  /// A map of detected entities, keyed by entity name, where each value
  /// is a list of `Entity` instances recognized in the speech.
  pub entities: HashMap<String, Vec<Entity>>,

  /// A list of `Intent` instances, representing the user intents
  /// inferred from the speech input.
  pub intents: Vec<Intent>,

  /// The raw transcription text corresponding to this understanding event.
  pub text: String,

  /// A map of detected traits (custom attributes), keyed by trait name,
  /// where each value is a `Trait` with associated values.
  pub traits: HashMap<String, Trait>,
}
