use crate::model::server::ServerClient;
use reqwest::blocking::{Client as RequestClient, RequestBuilder};

impl ServerClient {
  /// It prepares a get request with bearer auth.
  pub fn prepare_blocking_get_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .get(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a post request with bearer auth.  
  pub fn prepare_blocking_post_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .post(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a put request with bearer auth.  
  pub fn prepare_blocking_put_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .put(uri)
      .bearer_auth(self.token.clone())
  }

  /// It prepares a delete request with bearer auth.  
  pub fn prepare_blocking_delete_request(&self, uri: &str) -> RequestBuilder {
    RequestClient::new()
      .delete(uri)
      .bearer_auth(self.token.clone())
  }
}
