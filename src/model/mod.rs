use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Client module.
/// The main thing.
pub mod client;
/// The message API structs.
pub mod message;
/// The speech APi structs.
pub mod speech;
pub mod values;

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

#[derive(Serialize, Deserialize)]
/// Context is key in natural language.
///
/// For instance, at the same absolute instant, "today"
/// will be resolved to a different value depending on the timezone of the user.
pub struct Context {
  /// Local date and time of the user in [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html)
  /// format (more specifically, [RFC3339](https://www.ietf.org/rfc/rfc3339.txt).
  ///
  /// Do not use UTC time, which would defeat the purpose of this field.
  ///
  /// Example: `2014-10-30T12:18:45-07:00`
  pub reference_time: String,
  /// Local timezone of the user.
  /// Must be a valid [IANA timezone](https://www.iana.org/time-zones).
  ///
  /// Used only if no `reference_time` is provided.
  ///
  /// In this case, we will compute `reference_time` from timezone and the UTC time of the API server.
  ///
  /// If neither reference_time nor timezone are provided
  /// (or a fortiori if no context at all is provided),
  /// we will use the default timezone of your app,
  /// which you can set in 'Settings' in the web console.
  ///
  /// Example: `America/Los_Angeles`
  pub timezone: String,
  /// Locale of the user.
  ///
  /// The first 2 letters must be a valid [ISO639-1 language](https://www.iso.org/iso-639-language-codes.html),
  /// followed by an underscore,
  /// followed by a valid [ISO3166 alpha2 Country code](https://www.iso.org/obp/ui/#search/code/).
  ///
  /// `locale` is used to resolve the entities powered by our open-source linguistic parser, Duckling
  /// (e.g. `wit/datetime`, `wit/amount_of_money`).
  ///
  /// If you have locale-specific needs for dates and times, please contribute directly to [Duckling](https://github.com/facebook/duckling).
  ///
  /// If a locale is not yet available in Duckling, it will default to the "parent"
  /// language, with no locale-specific customization.
  ///
  /// Example: `en_GB`.
  pub locale: String,
  /// Coordinates of the user.
  ///
  /// coords is used to improve ranking for wit/location's resolved values.
  ///
  /// Learn more [here](https://wit.ai/docs/built-in-entities/#wit_location).
  ///
  /// Example: `{"lat": 37.47104, "long": -122.14703}`
  pub coords: Coords,
}

#[derive(Serialize, Deserialize)]
/// Coordinates of the user.
/// Must be in the form of an object with {"lat": float, "long": float}.
/// coords is used to improve ranking for wit/location's resolved values.
/// Learn more here.
/// Example: {"lat": 37.47104, "long": -122.14703}
pub struct Coords {
  /// User's latitude.
  pub lat: f64,
  /// User's longitude.
  pub long: f64,
}
