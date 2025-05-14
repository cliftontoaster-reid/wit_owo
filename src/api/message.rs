use crate::{
  error::{ApiError, WitError},
  model::{
    client::WitClient,
    message::{Message, MessageQuery},
  },
};
// API methods for the `/message` endpoint.
// Provides synchronous and asynchronous retrieval of `Message` objects.

impl WitClient {
  /// Asynchronously retrieves a `Message` from the Wit.ai API.
  ///
  /// # Arguments
  ///
  /// * `message` - A value convertible into `MessageQuery`, representing the message content or query parameters.
  ///
  /// This value can be a string, or a `MessageQuery` object for better control over the request.
  ///
  /// # Returns
  ///
  /// * `Ok(Message)` containing the parsed response on success.
  /// * `Err(ApiError)` if the request fails or the API returns an error.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use wit_owo::model::client::WitClient;
  /// # async fn example() -> Result<(), wit_owo::error::ApiError> {
  /// let client = WitClient::new("YOUR_TOKEN");
  /// let response = client.get_message("Hello world").await?;
  /// println!("{}", response.text);
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "tokio")]
  pub async fn get_message<T: Into<MessageQuery>>(&self, message: T) -> Result<Message, ApiError> {
    let query: MessageQuery = message.into();
    let request = self.prepare_get_request(query.into());

    let response = request.send().await?;

    if response.status().is_success() {
      let message: Message = response.json().await?;
      Ok(message)
    } else {
      let body: WitError = response.json().await?;
      Err(ApiError::WitError(body))
    }
  }

  /// Synchronously retrieves a `Message` from the Wit.ai API.
  ///
  /// # Arguments
  ///
  /// * `message` - A value convertible into `MessageQuery`, representing the message content or query parameters.
  ///
  /// This value can be a string, or a `MessageQuery` object for better control over the request.
  ///
  /// # Returns
  ///
  /// * `Ok(Message)` containing the parsed response on success.
  /// * `Err(ApiError)` if the request fails or the API returns an error.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use wit_owo::model::client::WitClient;
  /// # fn example() -> Result<(), wit_owo::error::ApiError> {
  /// let client = WitClient::new("YOUR_TOKEN");
  /// let response = client.get_message_blocking("Hello world")?;
  /// println!("{}", response.text);
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "blocking")]
  pub fn get_message_blocking<T: Into<MessageQuery>>(
    &self,
    message: T,
  ) -> Result<Message, ApiError> {
    let query: MessageQuery = message.into();
    let request = self.prepare_get_blocking(query.into());

    let response = request.send()?;

    if response.status().is_success() {
      let message: Message = response.json()?;
      Ok(message)
    } else {
      let body: WitError = response.json()?;
      Err(ApiError::WitError(body))
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::model::client::WitClient;
  use dotenv::dotenv;
  use lipsum::lipsum;
  use std::env;

  pub const LIPSUM_LENGTH: usize = 24;

  #[tokio::test]
  #[cfg(feature = "tokio")]
  async fn test_get_message() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let message = lipsum(LIPSUM_LENGTH);
    let result = client.get_message(&message).await;

    assert!(result.is_ok());
    let msg = result.unwrap();

    assert_eq!(msg.text, message);
  }

  #[tokio::test]
  #[cfg(feature = "tokio")]
  async fn test_get_message_complex() {
    dotenv().ok();
    use crate::model::message::MessageQuery;

    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let message = MessageQuery::new(lipsum(LIPSUM_LENGTH)).with_limit(5);
    let result = client.get_message(message.clone()).await;

    assert!(result.is_ok());
    let msg = result.unwrap();

    assert_eq!(msg.text, message.q);
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn test_get_message_blocking() {
    dotenv().ok();
    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let message = lipsum(LIPSUM_LENGTH);
    let result = client.get_message_blocking(&message);

    assert!(result.is_ok());
    let msg = result.unwrap();

    assert_eq!(msg.text, message);
  }

  #[test]
  #[cfg(feature = "blocking")]
  fn test_get_message_complex_blocking() {
    dotenv().ok();
    use crate::model::message::MessageQuery;

    let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
    let client = WitClient::new(&token);

    let message = MessageQuery::new(lipsum(LIPSUM_LENGTH)).with_limit(5);
    let result = client.get_message_blocking(message.clone());

    assert!(result.is_ok());
    let msg = result.unwrap();

    assert_eq!(msg.text, message.q);
  }
}
