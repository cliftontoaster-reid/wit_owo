/// The blocking version of the synthesize API.
#[cfg(feature = "blocking")]
pub mod blocking;

use crate::constants::check_message;
use crate::prelude::*;
use bytes::Bytes;
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize, Debug)]
/// The request to synthesize an audio from text.
pub struct SynthesizeRequest {
  /// The text to say, include [SSML](https://wit.ai/docs/ssml) support.
  pub q: String,
  /// The voice's name.
  ///
  /// It is usually recommended to use results from [`Client::get_voices`].
  pub voice: String,
  /// The voice's style.
  ///
  /// It is usually recommended to use results from [`Client::get_voices`].
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<String>,
  /// How fast the text is spoken.
  ///
  /// Default 100.
  ///
  /// Between 10 and 400. 10 is very slow, 400 is very fast.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub speed: Option<u16>,
  /// Pitch of the voice. Between 25 and 400.
  ///
  /// Default 100.
  ///
  /// 25 is low, 400 is high.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pitch: Option<u16>,
}

#[cfg(feature = "async")]
impl Client {
  /// Uses TTS technology to turn your text into audio.
  ///
  /// In this example we will first create a client
  /// and get the available voices to then take the first `en_US`
  /// available for making sure it will never break.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # #[tokio::main]
  /// # async fn main() {
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// let owo = Client::new(&token);
  /// let voices = owo.get_voices().await.unwrap();
  /// let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// # let options = SynthesizeRequest {
  /// # q: "OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.".to_string(),
  /// # voice: chosen_voice.name.to_string(),
  /// # style: None,speed: None,pitch: None,};
  /// # let audio = owo.synthesize(&options, "audio/wav").await.unwrap();
  /// # }
  /// ```
  /// Then we set the content we want the TTS to say, and in the voice we already prepared.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # #[tokio::main]
  /// # async fn main() {
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo = Client::new(&token);
  /// # let voices = owo.get_voices().await.unwrap();
  /// # let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// let options = SynthesizeRequest {
  ///   q: "<speak>OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.</speak>".to_string(),
  ///   voice: chosen_voice.name.to_string(),
  ///   style: None,speed: None,pitch: None,};
  /// # let audio = owo.synthesize(&options, "audio/wav").await.unwrap();
  /// # }
  /// ```
  /// We then send the request with the parameter `"audio/wav"`
  /// to get a wav file, note that this isn't the only available format,
  /// refer to the [official documentation](https://wit.ai/docs/http/20230215/#post__synthesize_link) for more information.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # #[tokio::main]
  /// # async fn main() {
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo = Client::new(&token);
  /// # let voices = owo.get_voices().await.unwrap();
  /// # let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// # let options = SynthesizeRequest {
  /// #   q: "<speak>OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.</speak>".to_string(),
  /// #   voice: chosen_voice.name.to_string(),
  /// #   style: None,speed: None,pitch: None,};
  /// let audio = owo.synthesize(&options, "audio/wav").await.unwrap();
  /// # }
  /// ```
  /// And here you have it, the [`Bytes`] from a wav file.
  /// The WAV file containing your awesome text, that has just been read using wit.ai's TTS.
  pub async fn synthesize(
    &self,
    options: &SynthesizeRequest,
    audio_format: &str,
  ) -> Result<Bytes, WitError> {
    check_message(&options.q)?;

    let op = to_string(&options).unwrap();
    #[cfg(test)]
    println!("Option string: {}", &op);

    let uwu = self
      .prepare_post_request("https://api.wit.ai/synthesize")
      .header("Content-Type", "application/json")
      .header("Accept", audio_format)
      .body(op)
      .send()
      .await
      .unwrap();

    if uwu.headers().get("Content-Type").unwrap() == "application/json" {
      Err(uwu.json().await.unwrap())
    } else {
      Ok(uwu.bytes().await.unwrap())
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;
  use crate::utils::levenshtein_distance;
  use std::env;

  #[tokio::test]
  async fn api_synthesize() {
    dotenv::dotenv().ok();
    let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
    let owo = Client::new(&token);
    let voices = owo.get_voices().await.unwrap();

    let chosen_voice = voices.get("en_US").unwrap().first().unwrap();

    let options = SynthesizeRequest {
      q: "<speak>OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.</speak>"
        .to_string(),
      voice: chosen_voice.name.to_string(),
      style: None,
      speed: None,
      pitch: None,
    };

    println!("Options: {:?}", &options);
    let audio = owo.synthesize(&options, "audio/wav").await.unwrap();

    let dictation = owo
      .dictation(
        audio.to_vec(),
        SpeechRequest {
          content_type: AudioContentType::Wav,
          context: None,
          tag: None,
          n: 1,
          entities: Default::default(),
        },
      )
      .await
      .unwrap();
    let text = match dictation.last().unwrap() {
      DictationResponse::Full(s) => s,
      DictationResponse::Half(_) => {
        unreachable!("Last should be final.")
      }
    };

    assert!(levenshtein_distance(&text.text, "OwO, I am silly toaster.") < 10);
  }
}
