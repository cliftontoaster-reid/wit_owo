//! # Wit.ai Message API
//!
//! This module provides a comprehensive interface for interacting with the Wit.ai Message API endpoint.
//! The Message API is the core functionality for processing natural language text and extracting
//! intents, entities, and traits from user messages.
//!
//! ## Overview
//!
//! The Wit.ai Message API allows you to:
//! - Send text messages for natural language understanding (NLU) processing
//! - Extract intents (what the user wants to do)
//! - Identify entities (structured data within the message)
//! - Detect traits (characteristics or attributes of the message)
//! - Use dynamic entities for context-specific recognition
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::client::WitClient;
//! # use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     // Initialize the client with your Wit.ai API token
//!     let client = WitClient::new(&token);
//!
//!     // Send a simple message for processing
//!     let response = client.get_message("Book a flight to Paris tomorrow").await?;
//!
//!     // Access the results
//!     println!("Original text: {}", response.text);
//!     println!("Intents: {:?}", response.intents);
//!     println!("Entities: {:?}", response.entities);
//!     println!("Traits: {:?}", response.traits);
//!
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Using MessageQuery for Advanced Options
//!
//! For more control over the request, use the `MessageQuery` struct:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{client::WitClient, message::MessageQuery};
//! # use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new(&token);
//!
//!     // Create a detailed query with additional parameters
//!     let query = MessageQuery::new("What's the weather like?".to_string())
//!         .with_tag("weather_query".to_string())  // Tag for analytics
//!         .with_limit(3);             // Limit to top 3 intents
//!
//!     let response = client.get_message(query).await?;
//!
//!     // Process the response...
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Synchronous API (blocking feature)
//!
//! If you prefer synchronous operations, enable the `blocking` feature:
//!
//! ```toml
//! [dependencies]
//! wit_owo = { version = "1.1.5", features = ["blocking"] }
//! ```
//!
//! ```no_run
//! use wit_owo::model::client::WitClient;
//! # use std::env;
//!
//! # #[cfg(feature = "blocking")]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new(&token);
//!
//!     // Blocking call - no async/await needed
//!     let response = client.get_message_blocking("Turn on the lights")?;
//!
//!     println!("Processed: {}", response.text);
//!     Ok(())
//! }
//!
//! # #[cfg(not(feature = "blocking"))]
//! # fn main() {
//! #     println!("Please enable the 'blocking' feature to use synchronous API calls.");
//! # }
//! ```
//!
//! ## Working with Responses
//!
//! The `Message` struct contains all the processed information from Wit.ai:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{client::WitClient, message::Message};
//!
//! async fn process_message(client: &WitClient, text: &str) -> Result<(), Box<dyn std::error::Error>> {
//!     let message: Message = client.get_message(text).await?;
//!
//!     // Access the original text
//!     println!("Input: {}", message.text);
//!
//!     // Check identified intents
//!     if let Some(intent) = message.intents.first() {
//!         println!("Top intent: {} (confidence: {})", intent.name, intent.confidence);
//!     }
//!
//!     // Extract entities
//!     for (entity_type, entities) in &message.entities {
//!         println!("Found {} entities of type '{}':", entities.len(), entity_type);
//!         for entity in entities {
//!             if let Some(ref value) = entity.value {
//!                 println!("  - {:?} (confidence: {})", value, entity.confidence);
//!             } else {
//!                 println!("  - {} (confidence: {})", entity.body, entity.confidence);
//!             }
//!         }
//!     }
//!
//!     // Check traits
//!     if !message.traits.is_empty() {
//!         println!("Detected traits:");
//!         for (trait_name, trait_values) in &message.traits {
//!             println!("  {}: {:?}", trait_name, trait_values);
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! #     let client = WitClient::new("test_token");
//! #     let _ = process_message(&client, "test message").await;
//! # });
//! # }
//! ```
//!
//! ## Advanced Features
//!
//! ### Dynamic Entities
//!
//! Dynamic entities allow you to provide context-specific entity values at request time:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::model::{
//!     client::WitClient,
//!     message::MessageQuery,
//!     entities::{DynamicEntity, EntityValue},
//! };
//! # use std::env;
//!
//! async fn use_dynamic_entities() -> Result<(), Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new(&token);
//!
//!     // Define dynamic entities for this specific request
//!     let mut contact_entity = DynamicEntity::new("contact".to_string());
//!     contact_entity.add_value(EntityValue {
//!         keyword: "john".to_string(),
//!         synonyms: vec!["john".to_string(), "johnny".to_string()],
//!     });
//!     contact_entity.add_value(EntityValue {
//!         keyword: "mary".to_string(),
//!         synonyms: vec!["mary".to_string(), "maria".to_string()],
//!     });
//!
//!     let dynamic_entities = vec![contact_entity];
//!
//!     let query = MessageQuery::new("Call John".to_string())
//!         .with_dynamic_entities(dynamic_entities);
//!
//!     let response = client.get_message(query).await?;
//!
//!     // The 'contact' entity should now recognize "John" as "john"
//!     if let Some(contacts) = response.entities.get("contact") {
//!         for contact in contacts {
//!             if let Some(ref value) = contact.value {
//!                 println!("Contact: {:?}", value);
//!             } else {
//!                 println!("Contact: {}", contact.body);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(use_dynamic_entities()).unwrap();
//! # }
//! ```
//!
//! ### Error Handling
//!
//! The API returns detailed error information when requests fail:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use wit_owo::{model::client::WitClient, error::ApiError};
//! # use std::env;
//!
//! async fn handle_errors() {
//!     # dotenvy::dotenv().ok();
//!     # let _token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let client = WitClient::new("INVALID_TOKEN");
//!
//!     match client.get_message("test message").await {
//!         Ok(message) => {
//!             println!("Success: {}", message.text);
//!         }
//!         Err(ApiError::WitError(wit_error)) => {
//!             println!("Wit.ai API error: {}", wit_error.error);
//!             println!("Error code: {}", wit_error.code);
//!         }
//!         Err(ApiError::RequestError(req_error)) => {
//!             println!("HTTP request error: {}", req_error);
//!         }
//!         Err(ApiError::SerializationError(json_error)) => {
//!             println!("JSON parsing error: {}", json_error);
//!         }
//!         Err(ApiError::UrlError(url_error)) => {
//!             println!("URL parsing error: {}", url_error);
//!         }
//!     }
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(handle_errors());
//! # }
//! ```
//!
//! ## Best Practices
//!
//! ### 1. **API Token Management**
//! Store your Wit.ai API token securely using environment variables:
//!
//! ```no_run
//! use std::env;
//! use wit_owo::model::client::WitClient;
//!
//! # fn main() {
//! # dotenvy::dotenv().ok();
//! let token = env::var("WIT_API_TOKEN")
//!     .expect("WIT_API_TOKEN environment variable not set");
//! let client = WitClient::new(&token);
//! # let _ = client; // Suppress unused variable warning
//! # }
//! ```
//!
//! ### 2. **Message Length Limits**
//! Keep messages under the maximum length limit. The library will validate this:
//!
//! ```no_run
//! use wit_owo::model::message::MessageQuery;
//! # fn main() {
//! // This will panic if the message is too long
//! let query = MessageQuery::new("your message here".to_string());
//! # let _ = query; // Suppress unused variable warning
//! # }
//! ```
//!
//! ### 3. **Intent Confidence Thresholds**
//! Always check confidence scores when processing intents:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! # use wit_owo::model::client::WitClient;
//! # async fn example(client: &WitClient) -> Result<(), wit_owo::error::ApiError> {
//! let message = client.get_message("book a table").await?;
//!
//! for intent in &message.intents {
//!     if intent.confidence > 0.8 {
//!         println!("High confidence intent: {}", intent.name);
//!         // Process this intent
//!     } else if intent.confidence > 0.5 {
//!         println!("Medium confidence intent: {}", intent.name);
//!         // Maybe ask for clarification
//!     }
//!     // no_run low confidence intents
//! }
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### 4. **Batch Processing**
//! For multiple messages, process them concurrently:
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # {
//! use futures::future::join_all;
//! use wit_owo::model::{client::WitClient, message::Message};
//! # use std::env;
//!
//! async fn process_multiple_messages(
//!     client: &WitClient,
//!     messages: Vec<&str>
//! ) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
//!     # dotenvy::dotenv().ok();
//!     # let _token = env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not found");
//!     let futures = messages.into_iter()
//!         .map(|msg| client.get_message(msg));
//!
//!     let results = join_all(futures).await;
//!
//!     results.into_iter().collect::<Result<Vec<_>, _>>()
//!         .map_err(|e| e.into())
//! }
//!
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! #     let client = WitClient::new("test_token");
//! #     let messages = vec!["hello", "world"];
//! #     let _ = process_multiple_messages(&client, messages).await;
//! # });
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! This module's functionality depends on cargo features:
//!
//! - **`async`** (default): Enables `get_message()` for asynchronous operations
//! - **`blocking`**: Enables `get_message_blocking()` for synchronous operations
//!
//! You can enable both features to use whichever pattern fits your application best.
//!
//! ## Examples
//!
//! See the `tests` module in this file for comprehensive examples of all functionality.
//!
//! ## Related Documentation
//!
//! - [`WitClient`]: The main client struct
//! - [`Message`]: Response structure containing processed results
//! - [`MessageQuery`]: Query builder for advanced message processing options
//! - [`ApiError`]: Error types returned by the API
//!
//! [`WitClient`]: crate::model::client::WitClient
//! [`Message`]: crate::model::message::Message
//! [`MessageQuery`]: crate::model::message::MessageQuery
//! [`ApiError`]: crate::error::ApiError
//!

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
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), wit_owo::error::ApiError> {
  /// let client = WitClient::new("YOUR_TOKEN");
  /// let response = client.get_message("Hello world").await?;
  /// println!("{}", response.text);
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "async")]
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
  /// # fn main() -> Result<(), wit_owo::error::ApiError> {
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
  use dotenvy::dotenv;
  use lipsum::lipsum;
  use std::env;

  pub const LIPSUM_LENGTH: usize = 24;

  #[tokio::test]
  #[cfg(feature = "async")]
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
  #[cfg(feature = "async")]
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
