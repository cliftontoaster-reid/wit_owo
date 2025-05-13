use url::Url;

#[derive(Debug, Clone, Default)]
pub struct WitClient(String);

impl WitClient {
  pub fn new(token: &str) -> Self {
    WitClient(token.to_string())
  }

  #[cfg(feature = "tokio")]
  pub(crate) fn prepare_get_request(&self, uri: Url) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();
    client
      .get(uri)
      .header("Authorization", format!("Bearer {}", self.0))
  }

  #[cfg(feature = "blocking")]
  pub(crate) fn prepare_get_blocking(&self, uri: Url) -> reqwest::blocking::RequestBuilder {
    let client = reqwest::blocking::Client::new();
    client
      .get(uri)
      .header("Authorization", format!("Bearer {}", self.0))
  }
}
