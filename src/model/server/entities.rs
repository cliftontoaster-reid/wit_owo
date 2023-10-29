/// The blocking version of the entities related APIs.
#[cfg(feature = "blocking")]
pub mod blocking;

use crate::model::server::entities::LookupStrategy::{Both, FreeText, Keywords};
use crate::model::server::ServerClient;
use crate::model::WitError;
use crate::prelude::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Deserialize, Debug, PartialEq)]
/// A generic form for an entity, carrying only the essentials.
pub struct GenericEntity {
  /// The random string ID of that entity.
  pub id: String,
  /// The entity's name.
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// A more detailed version of [`GenericEntity`]
/// containing all the information available for that entity.
pub struct DetailedEntity {
  /// The random string ID of that entity.
  ///
  /// Useless for serialization.
  #[serde(skip_serializing)]
  pub id: String,
  /// The entity's name.
  pub name: String,
  /// The lookup strategies used.
  pub lookups: Vec<String>,
  /// The keywords for detection.
  pub keywords: Vec<Keyword>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// A keyword for a possible entity's value.
pub struct Keyword {
  /// The name that you'll get from a request, like [`Client::message`].
  pub keyword: String,
  /// The possible values that could and should be interpreted as the keyword.
  pub synonyms: Vec<String>,
}

impl DetailedEntity {
  /// This gives a the parsed version of this value.
  pub fn lookups(&self) -> LookupStrategy {
    self.lookups.clone().into()
  }
}

/// For custom entities, list of lookup strategies (FreeText, Keywords). Both lookup strategies will be created if empty.
///
/// Here it is one single enumeration for simplicity.
#[derive(Debug, PartialEq)]
pub enum LookupStrategy {
  /// The entity is to be found by the position in the text, it could contain mostly any value.
  FreeText,
  /// The entity is only defined keywords.
  Keywords,
  /// The entity can be keywords but also any value positioned in the text.
  Both,
}

impl From<Vec<String>> for LookupStrategy {
  fn from(v: Vec<String>) -> Self {
    if v.len() > 2 {
      unreachable!("Dear god, why is it more than 2.")
    }
    if v.len() == 1 {
      match v.first().unwrap().as_str() {
        "free-text" => FreeText,
        "keywords" => Keywords,
        _ => unreachable!(),
      }
    } else {
      Both
    }
  }
}

impl From<LookupStrategy> for Vec<String> {
  fn from(v: LookupStrategy) -> Self {
    match v {
      FreeText => {
        vec!["free-text".to_string()]
      }
      Keywords => {
        vec!["keywords".to_string()]
      }
      Both => {
        vec!["keywords".to_string(), "free-text".to_string()]
      }
    }
  }
}

#[cfg(feature = "async")]
impl ServerClient {
  /// Lists the entities that are in your app.
  /// It only give the minimal [`GenericEntity`] version.
  ///
  /// To get more information use [`ServerClient::get_entity_info`].
  pub async fn list_entities(&self) -> Result<Vec<GenericEntity>, WitError> {
    let uwu = self
      .prepare_get_request("https://api.wit.ai/entities")
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Gets the information for an entity.
  ///
  /// Including keywords.
  pub async fn get_entity_info(&self, entity: &str) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_get_request(&format!("https://api.wit.ai/entities/{}", entity))
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Creates an intent. Using a [`DetailedEntity`] as config.
  pub async fn create_entity_info(
    &self,
    entity: &DetailedEntity,
  ) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_post_request("https://api.wit.ai/entities/")
      .header("Content-Type", "application/json")
      .json(entity)
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity and updates the entity with the same name, replacing it.
  pub async fn update_entity(&self, entity: &DetailedEntity) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_put_request("https://api.wit.ai/entities/")
      .header("Content-Type", "application/json")
      .json(entity)
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity's name and deletes it.
  pub async fn delete_entity(&self, entity: &str) -> Result<String, WitError> {
    let uwu = self
      .prepare_delete_request(&format!("https://api.wit.ai/entities/{}", entity))
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }

  /// Permanently deletes the role associated with the entity.
  ///
  /// Please refer to this recipe for information about roles.
  ///
  /// When the role is the last one of the entity, the entity is also deleted.
  pub async fn delete_entity_role(
    &self,
    entity: &str,
    role: &str,
  ) -> Result<(String, String), WitError> {
    let uwu = self
      .prepare_delete_request(&format!("https://api.wit.ai/entities/{entity}:{role}"))
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    match owo {
      Ok(v) => {
        let str: String = v
          .as_object()
          .unwrap()
          .get("deleted")
          .unwrap()
          .as_str()
          .unwrap()
          .parse()
          .unwrap();

        let mut owo: Vec<&str> = str.split(":").collect();

        let last = owo.pop().unwrap();

        Ok((owo.join(":"), last.to_string()))
      }
      Err(uwu) => Err(uwu),
    }
  }

  /// Adds a keyword to an entity.
  pub async fn entity_add_keyword(
    &self,
    entity: &str,
    keyword: &Keyword,
  ) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_post_request(&format!("https://api.wit.ai/entities/{entity}/keywords"))
      .header("Content-Type", "application/json")
      .json(keyword)
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity's keyword name and deletes it.
  pub async fn entity_delete_keyword(
    &self,
    entity: &str,
    keyword: &str,
  ) -> Result<String, WitError> {
    let uwu = self
      .prepare_delete_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}"
      ))
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }

  /// Adds a keyword to an entity.
  pub async fn entity_add_keyword_synonym(
    &self,
    entity: &str,
    keyword: &str,
    synonym: &str,
  ) -> Result<DetailedEntity, WitError> {
    let mut owo = Value::from_str("{}").unwrap();
    owo
      .as_object_mut()
      .unwrap()
      .insert("synonym".parse().unwrap(), synonym.parse().unwrap());

    let uwu = self
      .prepare_post_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}/synonyms"
      ))
      .header("Content-Type", "application/json")
      .json(&owo)
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    Client::extract(&uwu)
  }

  /// Delete a synonym of the keyword of the entity.
  pub async fn entity_delete_keyword_synonym(
    &self,
    entity: &str,
    keyword: &str,
    synonym: &str,
  ) -> Result<String, WitError> {
    let uwu = self
      .prepare_delete_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}/synonyms/{synonym}"
      ))
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }

  fn parse_standard_delete_response(owo: Result<Value, WitError>) -> Result<String, WitError> {
    match owo {
      Ok(v) => Ok(
        v.as_object()
          .unwrap()
          .get("deleted")
          .unwrap()
          .as_str()
          .unwrap()
          .parse()
          .unwrap(),
      ),
      Err(uwu) => Err(uwu),
    }
  }
}
