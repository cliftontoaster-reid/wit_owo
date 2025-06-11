# Wit OwO revived

The fluffiest and the most powerful client for meta's natural language understanding service is back.

Though as the few people that used this crate might have realized; I did leave this bowl of fluff to die
due to me having to do a lot of things at the same time, especially with my studies.
And for that I would like to apologize to all who used this crate and one day found out it was not
working any more, and that for so long, it did not work.

That is why for now I will try my best to, not forget and finally maintain this crate
as you deserve to have one.

## Description

The fluffiest and the most powerful client for meta's natural language understanding service, [Wit.AI](https://wit.ai).

The goal of this crate is to provide a feature-full yet easy to use client for Wit.AI's API. It is designed to be as flexible as possible, allowing you to use it in any way you see fit while making use of Rust's safety and strictness.

## Features

- **Simple** ✨: The client is designed to be as simple as possible to use, while still providing all the features of Wit.AI's API.
- **Flexible** 🤸: The client is designed to be used in many ways, allowing you to use it in any way you see fit.
- **Safe** 🛡️: The client is designed to be as safe as possible, making use when possible of strict types, as well as enums to ensure that you can't make mistakes.

## Cargo Features

This crate supports multiple feature flags to customize functionality based on your needs:

### `async` (default)

Enables asynchronous API support using the Tokio runtime. This feature provides:

- **Non-blocking operations**: All API calls return futures that can be awaited
- **Tokio integration**: Full compatibility with Tokio-based applications
- **Stream processing**: Support for async streams when handling continuous data
- **Concurrent requests**: Ability to make multiple API calls concurrently

```toml
[dependencies]
wit_owo = "1.1.5"  # async feature enabled by default
```

### `blocking`

Enables synchronous (blocking) API support for simpler use cases:

- **Synchronous operations**: Traditional blocking API calls
- **No async/await required**: Simpler code for basic use cases
- **Thread-safe**: Can be used safely across multiple threads

```toml
[dependencies]
wit_owo = { version = "1.1.5", features = ["blocking"] }
```

### Using both features

You can enable both async and blocking features simultaneously:

```toml
[dependencies]
wit_owo = { version = "1.1.5", features = ["async", "blocking"] }
```

This allows you to use both synchronous and asynchronous APIs in the same application, choosing the most appropriate one for each use case.

## Documentation & Tutorials

📚 **Comprehensive tutorials and examples are available on [docs.rs](https://docs.rs/wit_owo)**

The documentation includes detailed tutorials for all major functionality:

- **Message API Tutorial**: Step-by-step guides for natural language understanding, intent recognition, entity extraction, and dynamic entities
- **Dictation API Tutorial**: Complete examples for speech-to-text transcription, audio format handling, streaming, and batch processing
- **Speech API Tutorial**: Instructions for text-to-speech synthesis, voice configuration, and audio generation

Each tutorial includes:

- ✨ **Quick start examples** for immediate usage
- 🔧 **Advanced configuration** options and best practices
- 📖 **Real-world use cases** and practical applications
- ⚡ **Both async and blocking** code examples
- 🛡️ **Error handling** patterns and troubleshooting

Start with the [API module documentation](https://docs.rs/wit_owo/latest/wit_owo/api/) for an overview of all available tutorials.

## Legal Notice

This project is licensed under both the MIT and Apache 2.0 licences. You can find the full text of the licences in the [`LICENSE-MIT`](./LICENCE-MIT) and [`LICENSE-APACHE`](./LICENCE-APACHE) files respectively. You are therefore allowed to use this project in any way you see fit, as long as you respect the terms of the licenses you decide to align with.

Please note that I am not affiliated with Wit.AI or Meta. This project is a personal endeavour and is not officially endorsed by Wit.AI or Meta.

## Updates & Community

For the latest updates, announcements, and discussions about this crate, follow me on Mastodon:

🐘 **[@CliftonToasterReid@floofy.tech](https://floofy.tech/@CliftonToasterReid)**

Feel free to reach out if you have questions, suggestions, or just want to share how you're using the crate!

## Usage

To use this library, add the following to your `Cargo.toml`:

```toml
[dependencies]
wit_owo = { git = "https://github.com/cliftontoaster-reid/wit_owo.git" }
```

### API paths support

> - ✔️ Path supported and tested
> - ⚠️ Path supported but still being improved
> - ❌ Path not yet supported

- ❌ POST `/converse`

- ❌ POST `/event`

- ⚠️ POST `/speech`

- ✔️ GET `/message`

- ⚠️ POST `/dictation`

- ❌ POST `/synthesize`

- ❌ GET `/language`

- ❌ GET `/intents`

- ❌ POST `/intents`

- ❌ GET `/intents/:intent`

- ❌ DELETE `/intents/:intent`

- ❌ GET `/entities`

- ❌ POST `/entities`

- ❌ GET `/entities/:entity`

- ❌ PUT `/entities/:entity`

- ❌ DELETE `/entities/:entity`

- ❌ DELETE `/entities/:entity:role`

- ❌ POST `/entities/:entity/keywords`

- ❌ DELETE `/entities/:entity/keywords/:keyword`

- ❌ POST `/entities/:entity/keywords/:keyword/synonyms`

- ❌ DELETE `/entities/:entity/keywords/:keyword/synonyms/:synonym`

- ❌ GET `/traits`

- ❌ POST `/traits`

- ❌ GET `/traits/:trait`

- ❌ DELETE `/traits/:trait`

- ❌ POST `/traits/:trait/values`

- ❌ DELETE `/traits/:trait/values/:value`

- ❌ GET `/utterances`

- ❌ POST `/utterances`

- ❌ DELETE `/utterances`

- ❌ GET `/apps`

- ❌ GET `/apps/:app`

- ❌ POST `/apps`

- ❌ PUT `/apps/:app`

- ❌ DELETE `/apps/:app`

- ❌ POST `/apps/:app/client_tokens`

- ❌ GET `/apps/:app/tags`

- ❌ GET `/apps/:app/tags/:tag`

- ❌ POST `/apps/:app/tags`

- ❌ PUT `/apps/:app/tags/:tag`

- ❌ DELETE `/apps/:app/tags/:tag`

- ❌ GET `/export`

- ❌ POST `/import`

- ❌ GET `/voices`

- ❌ GET `/voices/:voice`

## Example

```rust,ignore
use dotenvy::dotenv;
use wit_owo::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
  dotenv().ok();
  let token = std::env::var("WIT_API_TOKEN").expect("WIT_API_TOKEN not set");
  let client = WitClient::new(&token);

  let message = "Hello world";
  let response = client.get_message(message).await?;

  println!("Response: {:?}", response);
  Ok(())
}
```

### Explanation

1. We import everything from `wit_owo::prelude`, which brings common types
   like `WitClient` and `ApiError` into scope.
2. We define an asynchronous `main` function using the `#[tokio::main]` macro.
   This macro sets up the Tokio runtime so we can `await` futures.
3. We read the `WIT_API_TOKEN` environment variable. This is your personal API
   token, required to authenticate with the Wit.ai service.
4. We create a new `WitClient` instance by calling `WitClient::new(&token)`.
   The client holds configuration and credentials needed for requests.
5. We prepare a text message (`"Hello world"`) and call
   `client.get_message(message).await?`. This sends a GET request to the
   `/message` endpoint and asynchronously waits for a parsed response.
6. If the request succeeds, we print out the structured response (wrapped in
   `Result::Ok`) using `println!`. If it fails, the `?` operator will
   return an `Err(ApiError)` from `main`, causing the program to exit with
   an error.

This pattern—reading a token, creating a client, making a request, then
handling the result—is the core workflow for all API calls in
