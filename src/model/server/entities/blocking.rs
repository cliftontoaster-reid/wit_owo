use super::*;

impl ServerClient {
  /// Lists the entities that are in your app.
  /// It only give the minimal [`GenericEntity`] version.
  ///
  /// To get more information use [`ServerClient::get_entity_info`].
  pub fn blocking_list_entities(&self) -> Result<Vec<GenericEntity>, WitError> {
    let uwu = self
      .prepare_blocking_get_request("https://api.wit.ai/entities")
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Gets the information for an entity.
  ///
  /// Including keywords.
  pub fn blocking_get_entity_info(&self, entity: &str) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_blocking_get_request(&format!("https://api.wit.ai/entities/{}", entity))
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Creates an intent. Using a [`DetailedEntity`] as config.
  pub fn blocking_create_entity_info(
    &self,
    entity: &DetailedEntity,
  ) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_blocking_post_request("https://api.wit.ai/entities/")
      .header("Content-Type", "application/json")
      .json(entity)
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity and updates the entity with the same name, replacing it.
  pub fn blocking_update_entity(&self, entity: &DetailedEntity) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_blocking_put_request("https://api.wit.ai/entities/")
      .header("Content-Type", "application/json")
      .json(entity)
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity's name and deletes it.
  pub fn blocking_delete_entity(&self, entity: &str) -> Result<String, WitError> {
    let uwu = self
      .prepare_blocking_delete_request(&format!("https://api.wit.ai/entities/{}", entity))
      .send()
      .unwrap()
      .json()
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }

  /// Permanently deletes the role associated with the entity.
  ///
  /// Please refer to this recipe for information about roles.
  ///
  /// When the role is the last one of the entity, the entity is also deleted.
  pub fn blocking_delete_entity_role(
    &self,
    entity: &str,
    role: &str,
  ) -> Result<(String, String), WitError> {
    let uwu = self
      .prepare_blocking_delete_request(&format!("https://api.wit.ai/entities/{entity}:{role}"))
      .send()
      .unwrap()
      .json()
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
  pub fn blocking_entity_add_keyword(
    &self,
    entity: &str,
    keyword: &Keyword,
  ) -> Result<DetailedEntity, WitError> {
    let uwu = self
      .prepare_blocking_post_request(&format!("https://api.wit.ai/entities/{entity}/keywords"))
      .header("Content-Type", "application/json")
      .json(keyword)
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Takes an entity's keyword name and deletes it.
  pub fn blocking_entity_delete_keyword(
    &self,
    entity: &str,
    keyword: &str,
  ) -> Result<String, WitError> {
    let uwu = self
      .prepare_blocking_delete_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}"
      ))
      .send()
      .unwrap()
      .json()
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }

  /// Adds a keyword to an entity.
  pub fn blocking_entity_add_keyword_synonym(
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
      .prepare_blocking_post_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}/synonyms"
      ))
      .header("Content-Type", "application/json")
      .json(&owo)
      .send()
      .unwrap()
      .json()
      .unwrap();

    Client::extract(&uwu)
  }

  /// Delete a synonym of the keyword of the entity.
  pub fn blocking_entity_delete_keyword_synonym(
    &self,
    entity: &str,
    keyword: &str,
    synonym: &str,
  ) -> Result<String, WitError> {
    let uwu = self
      .prepare_blocking_delete_request(&format!(
        "https://api.wit.ai/entities/{entity}/keywords/{keyword}/synonyms/{synonym}"
      ))
      .send()
      .unwrap()
      .json()
      .unwrap();

    let owo: Result<Value, WitError> = Client::extract(&uwu);

    Self::parse_standard_delete_response(owo)
  }
}
