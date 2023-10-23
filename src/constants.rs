use crate::prelude::WitError;

/// Wit.ai's messages limitations.
pub const MAX_MESSAGE_LENGTH: usize = 280;

pub fn check_message(text: &str) -> Result<bool, WitError> {
  if text.len() > MAX_MESSAGE_LENGTH {
    return Err(WitError {
      error: format!(
        "The message with a length of {} is greater than the max limit {}",
        text.len(),
        MAX_MESSAGE_LENGTH
      )
      .to_string(),
      code: "INTERNAL_MESSAGE_LEN_OVER_LIMIT".to_string(),
    });
  }
  Ok(true)
}
