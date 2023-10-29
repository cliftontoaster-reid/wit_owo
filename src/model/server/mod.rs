use reqwest::{Client as RequestClient, RequestBuilder};

/// The blocking versions of the utility functions.
#[cfg(feature = "blocking")]
pub mod blocking;
/// Entity related server APIs.
pub mod entities;
/// The server client prelude.
pub mod prelude;

/// The interface used to manage the connected application.
pub struct ServerClient {
  /// The server-side wit.ai token.
  pub token: String,
}

impl ServerClient {
  /// Creates a new server client.
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_owned(),
    }
  }
}

#[cfg(feature = "async")]
impl ServerClient {
  /// It prepares a get request with bearer auth.  
  pub fn prepare_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a post request with bearer auth.  
  pub fn prepare_post_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .post(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a put request with bearer auth.  
  pub fn prepare_put_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .put(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a delete request with bearer auth.  
  pub fn prepare_delete_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .delete(uri)
      .bearer_auth(self.token.clone())
  }
}
