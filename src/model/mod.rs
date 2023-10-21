use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Client module.
/// The main thing.
pub mod client;
/// The message API structs.
pub mod message;

#[derive(Debug, Deserialize)]
/// An error.
///
/// We also use this to display internally detected errors.
/// Just consider anything starting with `INTERNAL_` our stuff.
pub struct WitError {
  /// A message, if you need one.
  pub error: String,
  /// And the actual code, because it's more useful.
  pub code: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Makes it easier to make entities that relies on user related data without storing it on wit.ai.
pub struct DynamicEntities {
  /// The actual entities. You shouldn't have to deal with it.
  pub entities: HashMap<String, DynamicEntity>,
}
impl DynamicEntities {
  /// A function used to add an entity to the stack.
  ///
  /// The entity is supposed to be used the following way.
  ///
  /// First we initialize the variable with an empty stack.
  /// It will soon contain our `cuties` entity.
  /// ```
  /// use std::collections::HashMap;
  /// use wit_owo::prelude::*;
  ///
  /// let mut owo = DynamicEntities::default();
  /// #
  /// # let mut uwu = HashMap::new();
  /// #
  /// # uwu.insert("protogen", vec!["protogen", "toaster"]);
  /// # uwu.insert("sergal", vec!["sergal", "cheese"]);
  /// #
  /// # owo.add_entity(("cuties", uwu));
  /// ```
  /// Then we create a HashMap for what the example things is cute.
  /// Witch here are protogens and sergals.
  /// Those two possibilities can be called differently.
  /// For example protogens can be called toasters and sergals can be called cheese.
  /// ```
  /// # use std::collections::HashMap;
  /// # use wit_owo::prelude::*;
  /// #
  /// # let mut owo = DynamicEntities::default();
  /// #
  /// let mut uwu = HashMap::new();
  ///
  /// uwu.insert("protogen", vec!["protogen", "toaster"]);
  /// uwu.insert("sergal", vec!["sergal", "cheese"]);
  /// #
  /// # owo.add_entity(("cuties", uwu));
  /// ```
  /// And finally we add the `cuties` Hashmap to the stack.
  /// ```
  /// # use std::collections::HashMap;
  /// # use wit_owo::prelude::*;
  /// #
  /// # let mut owo = DynamicEntities::default();
  /// #
  /// # let mut uwu = HashMap::new();
  /// #
  /// # uwu.insert("protogen", vec!["protogen", "toaster"]);
  /// # uwu.insert("sergal", vec!["sergal", "cheese"]);
  /// #
  /// owo.add_entity(("cuties", uwu));
  /// ```
  /// And it's now ready to use.
  pub fn add_entity(&mut self, entity: (&str, HashMap<&str, Vec<&str>>)) -> &mut DynamicEntities {
    for (k, i) in entity.1 {
      self.entities.insert(
        entity.0.to_owned(),
        DynamicEntity {
          keyword: k.to_owned(),
          synonyms: i.iter().map(|uwu| uwu.to_string()).collect(),
        },
      );
    }

    self
  }
}

/// The raw representation of a dynamic entity.
/// Don't worry you will **noy** have to worry about this most of the time.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DynamicEntity {
  /// The keyword/name of the entity.
  pub keyword: String,
  /// The possibilities.
  pub synonyms: Vec<String>,
}
