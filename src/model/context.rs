use chrono::{DateTime, Local};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// Geographical coordinates of the user.
/// Used to improve ranking for wit/location’s resolved values.
/// Example: `{ "lat": 37.47104, "long": -122.14703 }`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinates {
  /// Latitude in decimal degrees.
  pub lat: f64,
  /// Longitude in decimal degrees.
  pub long: f64,
}

/// Context for natural‐language requests, providing optional user‐specific
/// information to resolve temporal and spatial entities correctly.
///
/// At the same absolute instant, “today” will be resolved differently depending
/// on the user’s timezone.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Context {
  /// Local date and time of the user in ISO 8601 / RFC 3339 format.
  /// Do not use UTC here. Example: `"2014-10-30T12:18:45-07:00"`.
  pub reference_time: Option<DateTime<Local>>,

  /// Local IANA timezone of the user, used if `reference_time` is omitted.
  /// The server will compute `reference_time` from this and the UTC time.
  /// Defaults to your app’s timezone if neither field is provided.
  /// Example: `"America/Los_Angeles"`.
  pub timezone: Option<Tz>,

  /// User locale in the form `<ISO639-1>_<ISO3166-1 alpha2>`.
  /// Drives Duckling’s parsing (e.g. wit/datetime). Falls back to the
  /// parent language if unavailable. Example: `"en_GB"`.
  pub locale: Option<String>,

  /// User’s geographic coordinates.
  /// Must be `{ "lat": float, "long": float }`.
  /// Used to refine wit/location outcomes.
  /// Example: `{ "lat": 37.47104, "long": -122.14703 }`.
  pub coords: Option<Coordinates>,
}
impl Coordinates {
  /// Create a new `Coordinates` from latitude and longitude.
  ///
  /// # Arguments
  ///
  /// * `lat` - Latitude in decimal degrees.
  /// * `long` - Longitude in decimal degrees.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// let coords = Coordinates::new(37.47104, -122.14703);
  /// assert_eq!(coords.lat, 37.47104);
  /// assert_eq!(coords.long, -122.14703);
  /// ```
  pub fn new(lat: f64, long: f64) -> Self {
    Coordinates { lat, long }
  }
}

impl Context {
  /// Create an empty `Context`, with all optional fields set to `None`.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// let ctx = Context::new();
  /// # assert!(ctx.reference_time.is_none());
  /// # assert!(ctx.timezone.is_none());
  /// # assert!(ctx.locale.is_none());
  /// # assert!(ctx.coords.is_none());
  /// ```
  pub fn new() -> Self {
    Context::default()
  }

  /// Set a fixed reference time.
  ///
  /// This will be used for temporal resolutions. Overrides any
  /// timezone-based computation.
  ///
  /// # Arguments
  ///
  /// * `dt` - A `DateTime<Local>` to use as the reference.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// # use chrono::Local;
  /// let now = Local::now();
  /// let ctx = Context::new().with_reference_time(now);
  /// assert_eq!(ctx.reference_time.unwrap(), now);
  /// ```
  pub fn with_reference_time(mut self, dt: DateTime<Local>) -> Self {
    self.reference_time = Some(dt);
    self
  }

  /// Specify a timezone to derive a reference time if none is set.
  ///
  /// When `reference_time` is `None`, the current UTC time is
  /// converted into this timezone.
  ///
  /// # Arguments
  ///
  /// * `tz` - An IANA timezone (`chrono_tz::Tz`).
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// let tz = "America/Los_Angeles".parse().unwrap();
  /// let ctx = Context::new().with_timezone(tz);
  /// assert_eq!(ctx.timezone.unwrap(), tz);
  /// ```
  pub fn with_timezone(mut self, tz: Tz) -> Self {
    self.timezone = Some(tz);
    self
  }

  /// Set the user locale.
  ///
  /// Should be in `<ISO639-1>_<ISO3166-1 alpha2>` format
  /// (e.g. `"en_GB"`). Drives language‐dependent parsing.
  ///
  /// # Arguments
  ///
  /// * `locale` - A locale string convertible to `String`.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// let ctx = Context::new().with_locale("en_GB");
  /// assert_eq!(ctx.locale.unwrap(), "en_GB");
  /// ```
  pub fn with_locale<S: Into<String>>(mut self, locale: S) -> Self {
    self.locale = Some(locale.into());
    self
  }

  /// Set the user's geographic coordinates.
  ///
  /// These refine location‐based parsing.
  ///
  /// # Arguments
  ///
  /// * `lat`  - Latitude in decimal degrees.
  /// * `long` - Longitude in decimal degrees.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wit_owo::prelude::*;
  /// let ctx = Context::new().with_coordinates(37.47, -122.14);
  /// let coords = ctx.coords.unwrap();
  /// assert_eq!(coords.lat, 37.47);
  /// assert_eq!(coords.long, -122.14);
  /// ```
  pub fn with_coordinates(mut self, lat: f64, long: f64) -> Self {
    self.coords = Some(Coordinates::new(lat, long));
    self
  }

  /// Retrieve the reference time, or compute "now" otherwise.
  ///
  /// Precedence:
  /// 1. Explicit `reference_time`.
  /// 2. Current UTC time converted to `timezone` (if set).
  /// 3. System local time.
  ///
  /// # Returns
  ///
  /// A `DateTime<Local>` representing the chosen reference point.
  pub fn reference_time_or_now(&self) -> DateTime<Local> {
    if let Some(rt) = &self.reference_time {
      *rt
    } else {
      Local::now()
    }
  }
}
