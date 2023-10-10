use reqwest::{Client as RequestClient, RequestBuilder};

/// Wit OwO client, the interface that will power your doom creations (hopefully).
pub struct Client {
  token: String,
}

impl Client {
  /// Initializes a wit.ai client.
  ///
  /// Takes the client side token and returns the Wit OwO.
  ///
  /// ```
  /// use wit_owo::prelude::*;
  ///
  /// let rawr = Client::new("owo fancy");
  /// ```
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_owned(),
    }
  }

  fn prepare_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  pub async fn message() {}
}
