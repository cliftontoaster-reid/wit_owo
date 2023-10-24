use crate::constants::check_message;
use crate::prelude::*;
use bytes::Bytes;
use serde_json::to_string;

impl Client {
  /// Uses TTS technology to turn your text into audio.
  ///
  /// In this example we will first create a client
  /// and get the available voices to then take the first `en_US`
  /// available for making sure it will never break.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// let owo = Client::new(&token);
  /// let voices = owo.get_blocking_voices().unwrap();
  /// let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// # let options = SynthesizeRequest {
  /// # q: "OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.".to_string(),
  /// # voice: chosen_voice.name.to_string(),
  /// # style: None,speed: None,pitch: None,};
  /// # let audio = owo.blocking_synthesize(&options, "audio/wav").unwrap();
  /// ```
  /// Then we set the content we want the TTS to say, and in the voice we already prepared.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo = Client::new(&token);
  /// # let voices = owo.get_blocking_voices().unwrap();
  /// # let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// let options = SynthesizeRequest {
  ///   q: "<speak>OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.</speak>".to_string(),
  ///   voice: chosen_voice.name.to_string(),
  ///   style: None,speed: None,pitch: None,};
  /// # let audio = owo.blocking_synthesize(&options, "audio/wav").unwrap();
  /// ```
  /// We then send the request with the parameter `"audio/wav"`
  /// to get a wav file, note that this isn't the only available format,
  /// refer to the [official documentation](https://wit.ai/docs/http/20230215/#post__synthesize_link) for more information.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo = Client::new(&token);
  /// # let voices = owo.get_blocking_voices().unwrap();
  /// # let chosen_voice = voices.get("en_US").unwrap().first().unwrap();
  /// # let options = SynthesizeRequest {
  /// #   q: "<speak>OwO, I am <emphasis level=\"strong\">silly</emphasis> toaster.</speak>".to_string(),
  /// #   voice: chosen_voice.name.to_string(),
  /// #   style: None,speed: None,pitch: None,};
  /// let audio = owo.blocking_synthesize(&options, "audio/wav").unwrap();
  /// ```
  /// And here you have it, the [`Bytes`] from a wav file.
  /// The WAV file containing your awesome text, that has just been read using wit.ai's TTS.
  pub fn blocking_synthesize(
    &self,
    options: &SynthesizeRequest,
    audio_format: &str,
  ) -> Result<Bytes, WitError> {
    check_message(&options.q)?;

    let op = to_string(&options).unwrap();
    #[cfg(test)]
    println!("Option string: {}", &op);

    let uwu = self
      .prepare_blocking_post_request("https://api.wit.ai/synthesize")
      .header("Content-Type", "application/json")
      .header("Accept", audio_format)
      .body(op)
      .send()
      .unwrap();

    if uwu.headers().get("Content-Type").unwrap() == "application/json" {
      Err(uwu.json().unwrap())
    } else {
      Ok(uwu.bytes().unwrap())
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;
  use crate::utils::levenshtein_distance;
  use std::env;

  #[test]
  fn blocking_api_synthesize() {
    dotenv::dotenv().ok();
    let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
    let owo = Client::new(&token);
    let voices = owo.get_blocking_voices().unwrap();

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
    let audio = owo.blocking_synthesize(&options, "audio/wav").unwrap();

    let dictation = owo
      .blocking_dictation(
        audio.to_vec(),
        SpeechRequest {
          content_type: AudioContentType::Wav,
          context: None,
          tag: None,
          n: 1,
          entities: Default::default(),
        },
      )
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
