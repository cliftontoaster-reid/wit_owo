use std::collections::HashMap;

use crate::{constants::MAX_TEXT_LENGTH, error::ApiError};
use serde::Deserialize;
use url::Url;

use crate::constants::BASE_URL;

use super::{entities::Entity, intents::Intent};

/// A query to the Wit.ai API for message processing.
///
/// This struct is used to create a request to the Wit.ai API to process a message.
/// It contains the message text, optional tags, and a limit on the number of intents to return.
#[derive(Clone, Debug)]
pub struct MessageQuery {
  /// The message text to be processed.
  pub q: String,
  /// Optional tag to be associated with the message.
  pub tag: Option<String>,
  /// Optional limit on the number of intents to return.
  pub n: Option<u8>,
}

impl MessageQuery {
  pub fn new(message: String) -> Self {
    if message.is_empty() {
      panic!("Message cannot be empty");
    }
    if message.len() > MAX_TEXT_LENGTH {
      panic!("Message cannot be longer than {MAX_TEXT_LENGTH} characters");
    }
    Self {
      q: message,
      tag: None,
      n: None,
    }
  }

  pub fn with_tag(mut self, tag: String) -> Self {
    self.tag = Some(tag);
    self
  }

  pub fn with_limit(mut self, limit: u8) -> Self {
    if limit > 8 {
      panic!("Cannot request more than 8 intents from Wit.ai");
    }
    self.n = Some(limit);
    self
  }

  pub(crate) fn to_url(&self) -> Result<Url, ApiError> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("q".to_string(), self.q.clone()));
    if let Some(tag) = &self.tag {
      params.push(("tag".to_string(), tag.clone()));
    }
    if let Some(n) = self.n {
      params.push(("n".to_string(), n.to_string()));
    }

    Url::parse_with_params(&format!("{BASE_URL}message"), params).map_err(|e| e.into())
  }
}

impl From<MessageQuery> for Url {
  fn from(val: MessageQuery) -> Self {
    val.to_url().unwrap()
  }
}

impl From<String> for MessageQuery {
  fn from(val: String) -> Self {
    MessageQuery::new(val)
  }
}

impl From<&String> for MessageQuery {
  fn from(val: &String) -> Self {
    MessageQuery::new(val.clone())
  }
}

impl From<&str> for MessageQuery {
  fn from(val: &str) -> Self {
    MessageQuery::new(val.to_string())
  }
}

/// Represents a message processed by the Wit.ai API.
/// This struct contains the original message text, the entities extracted from the message,
/// the intents identified in the message, and any traits associated with the message.
///
/// - The `entities` field is a map where the keys are entity names and the values are vectors of `Entity` structs.
///
/// - The `intents` field is a vector of `Intent` structs representing the intents identified in the message.
///
/// - The `traits` field is a map where the keys are trait names and the values are vectors of strings representing the trait values.
#[derive(Deserialize, Debug, Clone)]
pub struct Message {
  /// The original message text.
  pub text: String,
  /// The entities extracted from the message.
  pub entities: HashMap<String, Vec<Entity>>,
  /// The intents identified in the message.
  pub intents: Vec<Intent>,
  /// The traits associated with the message.
  pub traits: HashMap<String, Vec<String>>,
}
