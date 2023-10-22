use crate::model::message::Trait;
use crate::model::{Context, DynamicEntities};
use crate::prelude::{Entity, Intent};
use serde::Deserialize;
use std::collections::HashMap;

/// The optional headers used for this request.
pub struct SpeechRequest {
  /// The type of the audio you are going to send. Can be one of the following.
  ///
  /// audio/raw needs other headers set.
  /// Sadly we still do not have an audio engineer in our teem,
  /// witch means
  /// that for more informations you should refer to the [official documentation](https://wit.ai/docs/http/20230215/#post__speech_link).
  /// Even though we copied that part of it.
  pub content_type: AudioContentType,
  /// Context is key in natural language. For instance, at the same absolute instant, `today` will be resolved to a different value depending on the timezone of the user.
  pub context: Option<Context>,
  /// We have for now no idea what it is because we didn't find a documentation for it.
  pub tag: Option<String>,
  /// Number of intents you want at most.
  pub n: u8,
  /// Some entities can be different for each request. Maybe they are personalized and depend on the user issuing the query. Maybe they evolve dynamically based on the state of the environment at the time of the request.
  pub entities: DynamicEntities,
}

/// The type of the audio you are going to send. Can be one of the following.
pub enum AudioContentType {
  /// audio/wav
  Wav,
  /// audio/mpeg3
  Mp3,
  /// audio/ogg
  Ogg,
  /// audio/ulaw
  Ulaw,
  /// audio/raw
  Raw(AudioContentOptions),
}

impl AudioContentType {
  /// It's needed to create the header, it's little code that helps keeping sanity.
  /// And since the optional options are only used with [`Raw`]
  /// since it's contained in the other formats already, it's easier that way.
  pub fn to_str(&self) -> String {
    match self {
      AudioContentType::Wav => "audio/wav".to_string(),
      AudioContentType::Mp3 => "audio/mpeg3".to_string(),
      AudioContentType::Ogg => "audio/ogg".to_string(),
      AudioContentType::Ulaw => "audio/ulaw".to_string(),
      AudioContentType::Raw(owo) => {
        format!(
          "audio/raw;encoding={};bits={};rate={};endian={}",
          owo.encoding, owo.bits, owo.rate, owo.endian
        )
      }
    }
  }
}

/// Required options for [`AudioContentType::Raw`]
pub struct AudioContentOptions {
  /// Can be one of the following.
  /// - signed-integer
  /// - unsigned-integer
  /// - floating-point
  /// - mu-law
  /// - a-law
  /// - ima-adpcm
  /// - ms-adpcm
  /// - gsm-full-rate
  pub encoding: String,
  /// Can be one of the following.
  /// - 8
  /// - 16
  /// - 32
  pub bits: u8,
  /// An integer value like 8000 or 8k
  pub rate: u16,
  ///	Can be `big` or `little` (usually little, cf. this [Wikipedia article](http://en.wikipedia.org/wiki/Comparison_of_instruction_set_architectures#Instruction_sets))
  pub endian: String,
}

/// What wit.ai returns,
/// since it can sometimes be only the text,
/// we decided
/// to create this Enum to simplify
/// and adding the possibility of you having the entire process of computing in case of need.
pub enum SpeechResponse {
  /// The full response, see [`FullSpeechResponse`].
  Full(FullSpeechResponse),
  /// The half empty response, see [`HalfSpeechResponse`].
  Half(HalfSpeechResponse),
}

#[derive(Deserialize)]
/// The response for a speech request, similar to the [`Message`] struct.
pub struct FullSpeechResponse {
  /// The original text, but this time it makes sense.
  pub text: String,
  /// More information on the speech recognition process.
  pub speech: SpeechInfo,
  /// The list of intents, you should use [`FullSpeechResponse::intent`].
  /// Basically it's the possible meanings of the text.
  pub intents: Vec<Intent>,
  /// The object of every detected trait.
  /// It's a way to describe how the text sounds.
  /// Like greetings would be true if the message is something like.
  /// > Hi, could you X?
  pub entities: HashMap<String, Vec<Entity>>,
  /// The object of every detected entity.
  /// Basically it's possible detected arguments in the text like a name or a date, etc.
  pub traits: HashMap<String, Vec<Trait>>,
  /// To know if it's the final response.
  pub is_final: bool,
}

impl FullSpeechResponse {
  /// Get a trait by name and usage.
  ///
  /// # Arguments
  ///
  /// None
  ///
  /// # Returns
  ///
  /// - The specified intent if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let options = SpeechRequest {
  ///     content_type: AudioContentType::Mp3,
  ///     context: None,
  ///     tag: None,
  ///     n: 0,
  ///     entities: Default::default(),
  ///   };
  ///   let audio = include_bytes!("../../owo/test.mp3");
  ///
  ///   let uwu = owo
  ///     .speech(audio.to_vec(), options)
  ///     .await
  ///     .unwrap();
  ///
  ///   let full: &FullSpeechResponse = match uwu.last().unwrap() {
  ///     SpeechResponse::Full(d) => d,
  ///     SpeechResponse::Half(_) => unreachable!("Last should be final.")
  ///   };
  ///   let intent = full.intent().unwrap().name.clone();
  /// }
  /// ```
  pub fn intent(&self) -> Option<&Intent> {
    self.intents.get(0)
  }

  /// Get a trait by name and usage.
  ///
  /// # Arguments
  ///
  /// - `name` - The name and usage of the trait.
  ///
  /// # Returns
  ///
  /// - The specified list of entities if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let options = SpeechRequest {
  ///     content_type: AudioContentType::Mp3,
  ///     context: None,
  ///     tag: None,
  ///     n: 0,
  ///     entities: Default::default(),
  ///   };
  ///   let audio = include_bytes!("../../owo/test.mp3");
  ///
  ///   let uwu = owo
  ///     .speech(audio.to_vec(), options)
  ///     .await
  ///     .unwrap();
  ///
  ///   let full: &FullSpeechResponse = match uwu.last().unwrap() {
  ///     SpeechResponse::Full(d) => d,
  ///     SpeechResponse::Half(_) => unreachable!("Last should be final.")
  ///   };
  ///   let trait_name = full.get_trait("sexy").unwrap().get(0).unwrap().value.clone();
  /// }
  /// ```
  pub fn get_trait(&self, name: &str) -> Option<&Vec<Trait>> {
    self.traits.get(name)
  }

  /// Get an entity by name and usage, in the format 'name:usage'.
  ///
  /// # Arguments
  ///
  /// - `name` - The name and usage of the entity in the format 'name:usage'.
  ///
  /// # Returns
  ///
  /// - The specified list of entities if it exists.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  /// use std::env;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   dotenv::dotenv().ok();
  ///   let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
  ///
  ///   let options = SpeechRequest {
  ///     content_type: AudioContentType::Mp3,
  ///     context: None,
  ///     tag: None,
  ///     n: 0,
  ///     entities: Default::default(),
  ///   };
  ///   let audio = include_bytes!("../../owo/test.mp3");
  ///
  ///   let uwu = owo
  ///     .speech(audio.to_vec(), options)
  ///     .await
  ///     .unwrap();
  ///
  ///   let full: &FullSpeechResponse = match uwu.last().unwrap() {
  ///     SpeechResponse::Full(d) => d,
  ///     SpeechResponse::Half(_) => unreachable!("Last should be final.")
  ///   };
  ///   let trait_name = full.get_entity("owo:owo").unwrap().get(0).unwrap().value.clone();
  /// }
  /// ```
  pub fn get_entity(&self, name: &str) -> Option<&Vec<Entity>> {
    self.entities.get(name)
  }
}

/// Only the text, it is given to us, so we implement it.
/// We think it could be useful somewhere.
#[derive(Deserialize)]
pub struct HalfSpeechResponse {
  /// The detected text.
  pub text: String,
}

#[derive(Deserialize)]
/// If you want to get nerdy.
pub struct SpeechInfo {
  /// How much from 0 to 1 the computer things it's true.
  pub confidence: f32,
  /// The tokens, the text decided in pieces.
  pub tokens: Vec<SpeechToken>,
}

#[derive(Deserialize, Debug)]
/// An actual token.
pub struct SpeechToken {
  /// It's content.
  pub token: String,
  /// It's first character's position.
  pub start: i32,
  /// It's last character's position.
  pub end: i32,
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;
  use dotenv;
  use std::env;

  #[tokio::test]
  #[cfg(feature = "async")]
  async fn api_speech() {
    dotenv::dotenv().ok();
    let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));
    let options = SpeechRequest {
      content_type: AudioContentType::Mp3,
      context: None,
      tag: None,
      n: 0,
      entities: Default::default(),
    };
    let audio = include_bytes!("../../owo/test.mp3");
    let rawr = owo.speech(audio.to_vec(), options).await.unwrap();
    let uwu = match rawr.last().unwrap() {
      SpeechResponse::Full(d) => d,
      SpeechResponse::Half(_) => unreachable!("Last should be final."),
    };
    assert_eq!(uwu.intent().unwrap().name, "uwu");
    assert!(
      (uwu.entities.get("owo:owo").unwrap().get(0).unwrap().value
        == Some("what's this".to_string()))
        | (uwu.entities.get("owo:owo").unwrap().get(0).unwrap().value
          == Some("watch this".to_string()))
    );
    assert_eq!(uwu.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn blocking_api_message() {
    dotenv::dotenv().ok();
    let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

    let uwu = owo
      .blocking_message("OwO what's this", DynamicEntities::default())
      .unwrap();
    assert_eq!(uwu.intent().unwrap().name, "uwu");
    assert_eq!(
      uwu.entities.get("owo:owo").unwrap().get(0).unwrap().value,
      Some("what's this".to_string())
    );
    assert_eq!(uwu.get_trait("sexy").unwrap().get(0).unwrap().value, "very");
  }
}
