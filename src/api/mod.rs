//! # Wit.ai API Modules
//!
//! This module contains the core API functionality for interacting with Wit.ai's endpoints.
//! Each module includes comprehensive tutorials and examples to get you started quickly.

/// **Message API** - Natural Language Understanding for text processing.
///
/// Contains comprehensive tutorials and examples for intent recognition, entity extraction,
/// and trait detection. See the module documentation for step-by-step guides.
pub mod message;

/// **Dictation API** - Speech-to-Text transcription with streaming support.
///
/// Includes detailed tutorials for audio format handling, streaming transcription,
/// and batch processing. See the module documentation for audio guidelines and examples.
pub mod dictation;

/// **Speech API** - Text-to-Speech synthesis and audio generation.
///
/// Features tutorials for voice synthesis, audio format configuration,
/// and speech generation. See the module documentation for voice options and usage patterns.
pub mod speech;
