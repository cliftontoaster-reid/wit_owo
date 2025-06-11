pub use crate::constants::BASE_URL;
pub use crate::error::ApiError;
pub use crate::model::{
  client::WitClient,
  context::{Context, Coordinates},
  dictation::{AudioSource, Dictation, DictationQuery, Encoding, Speech, SpeechType, Token},
  entities::Entity,
  intents::Intent,
  message::{Message, MessageQuery},
  speech::{SpeechQuery, SpeechResponse, SpeechTranscription, SpeechUnderstanding},
  traits::Trait,
};
pub use url::Url;
