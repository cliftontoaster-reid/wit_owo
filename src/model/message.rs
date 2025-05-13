use std::collections::HashMap;

use crate::{constants::MAX_TEXT_LENGTH, error::ApiError};
use serde::Deserialize;
use url::Url;

use crate::constants::BASE_URL;

use super::{
  entities::{DynamicEntity, Entity},
  intents::Intent,
  traits::Trait,
};

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
  /// The dynamic entity array to be used in the request.
  pub dynamic_entities: Option<Vec<DynamicEntity>>,
}

impl MessageQuery {
  /// Creates a new `MessageQuery` with the given message text.
  ///
  /// # Arguments
  ///
  /// * `message` - The message text to be processed.
  ///
  /// # Panics
  ///
  /// Panics if the message is empty or longer than `MAX_TEXT_LENGTH` characters.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::message::MessageQuery;
  /// let query = MessageQuery::new("Hello world".to_string());
  /// # assert_eq!(query.q, "Hello world");
  /// ```
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
      dynamic_entities: None,
    }
  }

  /// Sets the tag for the `MessageQuery`.
  ///
  /// # Arguments
  ///
  /// * `tag` - The tag to be associated with the message.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::message::MessageQuery;
  /// let query = MessageQuery::new("Hello".to_string()).with_tag("greeting".to_string());
  /// # assert_eq!(query.tag, Some("greeting".to_string()));
  /// ```
  pub fn with_tag(mut self, tag: String) -> Self {
    self.tag = Some(tag);
    self
  }

  /// Sets the limit on the number of intents to return for the `MessageQuery`.
  ///
  /// # Arguments
  ///
  /// * `limit` - The maximum number of intents to return. Must be between 1 and 8.
  ///
  /// # Panics
  ///
  /// Panics if the limit is greater than 8.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::message::MessageQuery;
  /// let query = MessageQuery::new("What is the weather?".to_string()).with_limit(3);
  /// # assert_eq!(query.n, Some(3));
  /// ```
  pub fn with_limit(mut self, limit: u8) -> Self {
    if limit > 8 {
      panic!("Cannot request more than 8 intents from Wit.ai");
    }
    self.n = Some(limit);
    self
  }

  /// Sets the dynamic entities for the `MessageQuery`.
  ///
  /// # Arguments
  ///
  /// * `dynamic_entities` - A vector of `DynamicEntity` objects.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::model::message::MessageQuery;
  /// # use wit_owo::model::entities::DynamicEntity;
  /// let value = wit_owo::model::entities::EntityValue {
  ///   keyword: "purple".to_string(),
  ///   synonyms: vec!["violet".to_string(), "magenta".to_string()],
  /// };
  ///
  /// let mut entities = vec![
  ///   DynamicEntity::new("color".to_string()),
  /// ];
  /// entities[0].add_value(value);
  /// let query = MessageQuery::new("My favorite color is purple".to_string()).with_dynamic_entities(entities);
  /// # assert!(query.dynamic_entities.is_some());
  /// # let owo = query.dynamic_entities.unwrap();
  /// # assert_eq!(owo.len(), 1);
  /// # assert_eq!(owo[0].name, "color");
  /// # assert_eq!(owo[0].values[0].keyword, "purple");
  /// # assert_eq!(owo[0].values[0].synonyms, vec!["violet", "magenta"]);
  /// ```
  pub fn with_dynamic_entities(mut self, dynamic_entities: Vec<DynamicEntity>) -> Self {
    self.dynamic_entities = Some(dynamic_entities);
    self
  }

  /// Converts the `MessageQuery` into a `Url` for the Wit.ai API.
  ///
  /// This method constructs the URL with the query parameters based on the fields of the `MessageQuery`.
  ///
  /// # Returns
  ///
  /// * `Ok(Url)` containing the constructed URL on success.
  /// * `Err(ApiError)` if there is an error during URL parsing or JSON serialization of dynamic entities.
  ///
  /// # Examples
  ///
  /// ```compile_fail
  /// # use wit_owo::model::message::MessageQuery;
  /// let query = MessageQuery::new("Test query".to_string());
  /// let url = query.to_url().unwrap();
  /// # assert!(url.to_string().contains("q=Test%20query"));
  /// ```
  pub(crate) fn to_url(&self) -> Result<Url, ApiError> {
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("q".to_string(), self.q.clone()));
    if let Some(tag) = &self.tag {
      params.push(("tag".to_string(), tag.clone()));
    }
    if let Some(n) = self.n {
      params.push(("n".to_string(), n.to_string()));
    }
    // The dynamic entities should be remade into the following format:
    // HashMap<String, HashMap<String, {keyword: String, synonyms: Vec<String>}>>
    //
    // An example of the expected format:
    // ```json
    // {
    //   "entities": {
    //     "color": [
    //       {
    //         "keyword": "purple",
    //         "synonyms": ["violet", "magenta"]
    //       },
    //       {
    //         "keyword": "blue",
    //         "synonyms": ["aqua blue", "marine blue"]
    //       }
    //     ]
    //   }
    // }
    // ```
    //
    // It should then be serialized into a JSON string, made url safe, and added to the params as `entities`.
    if let Some(dynamic_entities) = &self.dynamic_entities {
      let mut entities: HashMap<String, serde_json::Value> = HashMap::new();
      for entity in dynamic_entities {
        let name = entity.name.clone();
        let data: serde_json::Value = serde_json::to_value(entity)?;

        entities.insert(name, data);
      }
      // We now are able to turn this into a JSON string
      // and make it url safe
      let json_raw = serde_json::to_string(&entities)?;
      let json_safe = urlencoding::encode(&json_raw);

      params.push(("entities".to_string(), json_safe.to_string()));
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
  /// Each key is a trait name and the value is a list of trait entries.
  #[serde(default)]
  pub traits: HashMap<String, Vec<Trait>>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;
  use std::collections::HashMap;

  #[test]
  #[should_panic(expected = "Message cannot be empty")]
  fn new_empty_panics() {
    let _ = MessageQuery::new("".to_string());
  }

  #[test]
  #[should_panic(expected = "Message cannot be longer than")]
  fn new_too_long_panics() {
    // Create a string longer than MAX_TEXT_LENGTH
    let long = "a".repeat(MAX_TEXT_LENGTH + 1);
    let _ = MessageQuery::new(long);
  }

  #[test]
  fn with_tag_sets_tag() {
    let mq = MessageQuery::new("test".into()).with_tag("tagged".into());
    assert_eq!(mq.tag.as_deref(), Some("tagged"));
  }

  #[test]
  #[should_panic(expected = "Cannot request more than 8 intents")]
  fn with_limit_panics_when_exceeds() {
    let _ = MessageQuery::new("hi".into()).with_limit(9);
  }

  #[test]
  fn with_limit_sets_n() {
    let mq = MessageQuery::new("test".into()).with_limit(5);
    assert_eq!(mq.n, Some(5));
  }

  #[test]
  fn to_url_only_query() {
    let mq = MessageQuery::new("hello world".into());
    let url = mq.to_url().unwrap();
    assert!(url.path().ends_with("/message"));
    let pairs: HashMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(pairs.get("q"), Some(&"hello world".to_string()));
    assert!(!pairs.contains_key("tag"));
    assert!(!pairs.contains_key("n"));
    assert!(!pairs.contains_key("entities"));
  }

  #[test]
  fn to_url_with_tag_and_limit() {
    let mq = MessageQuery::new("hello".into())
      .with_tag("greeting".into())
      .with_limit(2);
    let url = mq.to_url().unwrap();
    let pairs: HashMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(pairs.get("q"), Some(&"hello".to_string()));
    assert_eq!(pairs.get("tag"), Some(&"greeting".to_string()));
    assert_eq!(pairs.get("n"), Some(&"2".to_string()));
  }

  #[test]
  fn from_string_and_str() {
    let mq1: MessageQuery = "foo".into();
    let mq2 = MessageQuery::new("foo".into());
    assert_eq!(mq1.q, mq2.q);

    let s = "bar".to_string();
    let mq3: MessageQuery = s.clone().into();
    assert_eq!(mq3.q, "bar");
  }

  #[test]
  fn deserialize_message() {
    let json = r#"
{
  "entities": {},
  "intents": [
    {
      "confidence": 0.9048754466499055,
      "id": "776124090880944",
      "name": "enable_welcome_message"
    }
  ],
  "text": "Please",
  "traits": {
    "please": [
      {
        "confidence": 0.740637952351606,
        "id": "844096410415880",
        "value": "true"
      }
    ]
  }
}

    "#;
    let msg: Message = serde_json::from_str(json).unwrap();

    assert_eq!(msg.entities.len(), 0);
    assert_eq!(msg.intents.len(), 1);
    let mut conf = msg.intents[0].confidence;
    let mut expected = 0.904_875_46;
    assert!(
      (conf - expected).abs() < 1e-6,
      "expected ~{expected}, got {conf}"
    );
    assert_eq!(msg.intents[0].id, "776124090880944");
    assert_eq!(msg.intents[0].name, "enable_welcome_message");
    assert_eq!(msg.text, "Please");
    assert_eq!(msg.traits.len(), 1);
    assert_eq!(msg.traits["please"].len(), 1);
    conf = msg.traits["please"][0].confidence;
    expected = 0.740_637_96;
    assert!(
      (conf - expected).abs() < 1e-6,
      "expected ~{expected}, got {conf}"
    );
    assert_eq!(msg.traits["please"][0].id, "844096410415880");
    assert_eq!(msg.traits["please"][0].value, "true");
  }
}
