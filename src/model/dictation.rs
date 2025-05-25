use bytes::Bytes;
use futures::stream::Stream;
#[cfg(feature = "async")]
use reqwest::Body;
#[cfg(feature = "blocking")]
use reqwest::blocking::Body as BlockingBody;
use serde::Deserialize;
use std::{fmt::Debug, pin::Pin};

/// Represents the encoding format of the audio data.
#[derive(Debug, Clone, Default)]
pub enum Encoding {
  /// Waveform Audio File Format.
  #[default]
  Wav,
  /// MPEG Audio Layer III.
  Mp3,
  /// Ogg Vorbis audio format.
  Ogg,
  /// µ-law algorithm, primarily used in telephony.
  Ulaw,
  /// Raw audio data, requires additional parameters like bit depth, sample rate, and endianness.
  Raw,
}

/// Represents the set of parameters for a dictation request,
/// including the audio source and its format details.
#[derive(Debug, Default)]
pub struct DictationQuery {
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
}

impl DictationQuery {
  /// Creates a new `DictationQuery` with the specified audio source and encoding.
  ///
  /// # Parameters
  /// - `encoding`: The encoding format of the audio data.
  /// - `data`: The audio source to transcribe, either buffered or streaming.
  ///
  /// # Returns
  /// A new `DictationQuery` instance with the specified parameters.
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
  /// `true` for little-endian, `false` for big-endian.
  pub fn with_endian(mut self, endian: bool) -> Self {
    self.endian = Some(endian);
    self
  }

  /// Generates the URL with query parameters for the dictation request.
  pub(crate) fn to_url(&self) -> Result<url::Url, crate::error::ApiError> {
    use crate::prelude::BASE_URL;
    use url::Url;

    let mut params: Vec<(String, String)> = Vec::new();

    // Add raw encoding parameters as query parameters if present
    if let Some(raw_encoding) = &self.raw_encoding {
      params.push(("encoding".to_string(), raw_encoding.clone()));
    }

    if let Some(bits) = self.bits {
      params.push(("bits".to_string(), bits.to_string()));
    }

    if let Some(sample_rate) = self.sample_rate {
      params.push(("rate".to_string(), sample_rate.to_string()));
    }

    if let Some(endian) = self.endian {
      params.push((
        "endian".to_string(),
        (if endian { "little" } else { "big" }).to_string(),
      ));
    }

    Url::parse_with_params(&format!("{BASE_URL}dictation"), params).map_err(|e| e.into())
  }
}

impl std::fmt::Display for DictationQuery {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let raw: Result<String, std::fmt::Error> = match &self.encoding {
      Encoding::Wav => Ok("audio/wav".to_string()),
      Encoding::Mp3 => Ok("audio/mp3".to_string()),
      Encoding::Ogg => Ok("audio/ogg".to_string()),
      Encoding::Ulaw => Ok("audio/ulaw".to_string()),
      Encoding::Raw => {
        // 'content-type': 'audio/raw;encoding={raw_encoding};bits={bits};rate={sample_rate};endian=[little|big]'
        if self.raw_encoding.is_none() {
          return Err(std::fmt::Error);
        }
        if self.bits.is_none() {
          return Err(std::fmt::Error);
        }
        if self.sample_rate.is_none() {
          return Err(std::fmt::Error);
        }
        if self.endian.is_none() {
          return Err(std::fmt::Error);
        }
        Ok(format!(
          "audio/raw;encoding={};bits={};rate={};endian={}",
          self.raw_encoding.as_ref().unwrap(),
          self.bits.as_ref().unwrap(),
          self.sample_rate.as_ref().unwrap(),
          if self.endian.unwrap() {
            "little"
          } else {
            "big"
          }
        ))
      }
    };

    write!(f, "{}", raw?)
  }
}

/// Represents a single audio chunk in a speech transcription.
pub enum AudioSource {
  /// Represents a buffered audio source.
  Buffered(Bytes),
  /// Represents a streaming audio source.
  Stream(Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>),
}

impl Default for AudioSource {
  fn default() -> Self {
    AudioSource::Buffered(Bytes::new())
  }
}

impl Debug for AudioSource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AudioSource::Buffered(_) => write!(f, "AudioSource::Buffered"),
      AudioSource::Stream(_) => write!(f, "AudioSource::Stream"),
    }
  }
}

#[cfg(feature = "async")]
impl From<AudioSource> for Body {
  fn from(source: AudioSource) -> Body {
    match source {
      AudioSource::Buffered(bytes) => Body::from(bytes),
      AudioSource::Stream(stream) => Body::wrap_stream(stream),
    }
  }
}

#[cfg(feature = "blocking")]
impl From<AudioSource> for BlockingBody {
  fn from(source: AudioSource) -> BlockingBody {
    match source {
      AudioSource::Buffered(bytes) => BlockingBody::from(bytes),
      AudioSource::Stream(_) => panic!("BlockingBody cannot be created from a stream"),
    }
  }
}

impl From<Bytes> for AudioSource {
  fn from(bytes: Bytes) -> Self {
    AudioSource::Buffered(bytes)
  }
}

impl From<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>> for AudioSource {
  fn from(stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>) -> Self {
    AudioSource::Stream(stream)
  }
}

impl From<Vec<u8>> for AudioSource {
  fn from(bytes: Vec<u8>) -> Self {
    AudioSource::Buffered(Bytes::from(bytes))
  }
}

impl From<&[u8]> for AudioSource {
  fn from(bytes: &[u8]) -> Self {
    AudioSource::Buffered(Bytes::copy_from_slice(bytes))
  }
}

impl AudioSource {
  /// Converts the audio source into a `Body` for use in HTTP requests.
  /// Converts the audio source into a `Body` for use in HTTP requests.
  pub fn into_body(self) -> Body {
    self.into()
  }

  /// Creates a new buffered `AudioSource` from the provided bytes.
  ///
  /// # Parameters
  /// - `bytes`: A `Bytes` buffer containing the audio data to be buffered.
  ///
  /// # Returns
  /// A new `AudioSource::Buffered` containing the provided bytes.
  pub fn new_buffered(bytes: Bytes) -> Self {
    AudioSource::Buffered(bytes)
  }

  /// Creates a new streaming `AudioSource` from the provided asynchronous stream.
  ///
  /// # Parameters
  /// - `stream`: A pinned boxed `Stream` yielding `Result<Bytes, reqwest::Error>`.
  ///
  /// # Returns
  /// A new `AudioSource::Stream` wrapping the provided stream.
  pub fn new_stream(
    stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
  ) -> Self {
    AudioSource::Stream(stream)
  }
}

/// Represents a single token in a speech transcription.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Token {
  /// The start index of the token in the original audio or text.
  pub start: usize,
  /// The end index of the token in the original audio or text.
  pub end: usize,
  /// The textual content of the token.
  pub token: String,
}

/// Represents a speech transcription.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Speech {
  /// The confidence level of the transcription, typically a value between 0.0 and 1.0.
  pub confidence: f32,
  /// A vector of tokens that make up the transcribed speech.
  pub tokens: Vec<Token>,
}

/// Represents the type of speech transcription.
#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum SpeechType {
  /// Indicates a partial transcription, meaning the transcription is ongoing or incomplete.
  #[serde(rename = "PARTIAL_TRANSCRIPTION")]
  #[default]
  PartialTranscription,
  /// Indicates a final transcription, meaning the transcription is complete.
  #[serde(rename = "FINAL_TRANSCRIPTION")]
  FinalTranscription,
}

/// Represents a dictation event, combining speech transcription and the resulting text.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Dictation {
  /// The speech transcription details.
  pub speech: Speech,
  /// The full transcribed text.
  pub text: String,
  /// The type of transcription (partial or final).
  #[serde(rename = "type")]
  pub speech_type: SpeechType,
}
