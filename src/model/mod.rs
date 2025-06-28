//! This module contains the structures, enums, and traits used in the Wit.ai API client.
//!
//! Although most of the functionality is implemented in the `api` module, this module provides
//! the data structures and traits that are used to represent the data returned by the Wit.ai API.

/// This module contains the main structures and traits for the Wit.ai API client.
pub mod client;
/// This module contains the structures related to the entities returned by the Wit.ai API.
pub mod entities;
/// This module contains the structures related to the intents returned by the Wit.ai API.
pub mod intents;
/// This module contains the structures related to the messages sent to and received from the Wit.ai API.
pub mod message;
/// This module contains the structures related to the traits returned by the Wit.ai API.
pub mod traits;

/// This module contains the structures related to the contexts returned by the Wit.ai API.
pub mod context;

/// This module contains the structures related to the dictation returned by the Wit.ai API.
pub mod dictation;

/// This module contains the structures related to the speech returned by the Wit.ai API.
pub mod speech;
/// This module contains the structures related to the voices returned by the Wit.ai API.
pub mod voice;
