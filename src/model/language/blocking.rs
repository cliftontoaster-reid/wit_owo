use crate::constants::check_message;
use crate::model::language::LanguageResponse;
use crate::prelude::*;

impl Client {
  /// Detects the language of the language,
  /// returns `n` numbers of language possibility, as long as n is from 1 to 8, included.
  ///
  /// To use it first initialize a [`Client`]
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// let owo = Client::new(&token);
  /// # let languages = owo.blocking_detect_language("OwO I'm a silly toaster.", 1).unwrap();
  /// # let number = languages.detected_locales.len();
  /// # assert_eq!(number, 1);
  /// # assert!(languages.detected_locales.first().unwrap().locale.starts_with("en"));
  /// ```
  /// And we run the function giving it a lovely text to analyse.
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use std::env;
  /// # dotenv::dotenv().ok();
  /// # let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
  /// # let owo = Client::new(&token);
  /// let languages = owo.blocking_detect_language("OwO I'm a silly toaster.", 1).unwrap();
  /// # let number = languages.detected_locales.len();
  /// # assert_eq!(number, 1);
  /// # assert!(languages.detected_locales.first().unwrap().locale.starts_with("en"));
  /// ```
  pub fn blocking_detect_language(&self, text: &str, n: u8) -> Result<LanguageResponse, WitError> {
    check_message(text)?;
    if !(1..=8).contains(&n) {
      return Err(WitError {
        error: format!(
          "The value `n` is equal to {n}, witch is not is the correct bound `1 <= n <= 8`."
        ),
        code: "INTERNAL_INVALID_QUERY".parse().unwrap(),
      });
    }

    let uwu = self
      .prepare_blocking_get_request("https://api.wit.ai/language")
      .query(&vec![("q", text), ("n", &n.to_string())])
      .send()
      .unwrap()
      .json()
      .unwrap();

    Self::extract(&uwu)
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;
  use std::env;

  #[test]
  fn blocking_api_language() {
    dotenv::dotenv().ok();
    let token: String = dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
    let owo = Client::new(&token);
    let languages = owo
      .blocking_detect_language("OwO I'm a silly toaster.", 1)
      .unwrap();
    let number = languages.detected_locales.len();
    assert_eq!(number, 1);
    assert!(languages
      .detected_locales
      .first()
      .unwrap()
      .locale
      .starts_with("en"));
  }
}
