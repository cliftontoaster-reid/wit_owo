use url::Url;

/// A client for interacting with the Wit.ai HTTP API.
///
/// Holds the bearer token and provides helpers for building requests.
#[derive(Debug, Clone, Default)]
pub struct WitClient(String);

impl WitClient {
  /// Creates a new `WitClient` with the given bearer token.
  ///
  /// # Arguments
  ///
  /// * `token` - Your Wit.ai server access token.
  ///
  /// # Returns
  ///
  /// A `WitClient` instance which can be used to prepare authenticated requests.
  pub fn new(token: &str) -> Self {
    WitClient(token.to_string())
  }

  /// Prepares an asynchronous GET request with the `tokio` feature enabled.
  ///
  /// This function returns a `reqwest::RequestBuilder` that is already
  /// configured with the Authorization header.
  ///
  /// # Arguments
  ///
  /// * `uri` - The full URL of the Wit.ai endpoint you want to call.
  ///
  /// # Panics
  ///
  /// Panics if the URI is not valid.
  ///
  /// # Example
  ///
  /// ```ignore
  /// # use url::Url;
  /// # use wit_owo::model::client::WitClient;
  /// # async fn example() {
  /// let client = WitClient::new("TOKEN");
  /// let uri = Url::parse("https://api.wit.ai/message?q=hello").unwrap();
  /// let request = client.prepare_get_request(uri);
  /// let response = request.send().await.unwrap();
  /// # }
  /// ```
  #[cfg(feature = "async")]
  pub(crate) fn prepare_get_request(&self, uri: Url) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();
    client
      .get(uri)
      .header("Authorization", format!("Bearer {}", self.0))
  }

  /// Prepares a blocking GET request with the `blocking` feature enabled.
  ///
  /// This function returns a `reqwest::blocking::RequestBuilder` that is already
  /// configured with the Authorization header.
  ///
  /// # Arguments
  ///
  /// * `uri` - The full URL of the Wit.ai endpoint you want to call.
  ///
  /// # Panics
  ///
  /// Panics if the URI is not valid.
  ///
  /// # Example
  ///
  /// ```ignore
  /// # use url::Url;
  /// # use wit_owo::model::client::WitClient;
  /// let client = WitClient::new("TOKEN");
  /// let uri = Url::parse("https://api.wit.ai/message?q=hello").unwrap();
  /// let response = client
  ///     .prepare_get_blocking(uri)
  ///     .send()
  ///     .unwrap();
  /// ```
  #[cfg(feature = "blocking")]
  pub(crate) fn prepare_get_blocking(&self, uri: Url) -> reqwest::blocking::RequestBuilder {
    let client = reqwest::blocking::Client::new();
    client
      .get(uri)
      .header("Authorization", format!("Bearer {}", self.0))
  }
}
