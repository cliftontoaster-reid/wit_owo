//! # Wit.owo
//!
//! In light of the widely recognized influence of individuals within the furry community on the digital landscape, it comes as no surprise that an endeavor to enhance the Wit.ai library is in progress. What distinguishes this initiative from its predecessors? It isn't necessarily a matter of superior intellect, as I, for one, do not lay claim to exceptional intelligence. Rather, the distinction lies in the intention to comprehensively implement a diverse array of functions, a facet that certain existing libraries like 'witty' and 'wit_ai' have yet to fully embrace.
//!
//! You can begin with this example. It's currently up-to-date, and if I happen to forget to change it from time to time, you have my permission to raise an issue on GitHub with the following remark
//!
//! > You stinky furry, you forgot to change the example, stupid toaster!
//!
//! Rest assured, I won't take offense, it's really as a joke.
//!
//! The upcoming venture seeks to encompass a multitude of functionalities, spanning from text-to-speech to speech-to-text conversion, acknowledging the extensive spectrum of capabilities that this domain encompasses. The aspiration is to provide a holistic solution that caters to a wide array of requirements. Anticipate forthcoming enhancements designed to facilitate your digital experiences; for the time being, dedicated efforts are underway. Until we reconvene, I wish you a productive and enjoyable journey.
//!
//! ```
//! use wit_owo::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!   dotenv::dotenv().ok();
//!   let owo = owo_whats_this::model::client::Client::new(&dotenv::var("wit_ai").expect("For testing a .env must have wit_ai set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip"));
//!
//!   let uwu = owo
//!     .message("OwO what's this", DynamicEntities::default())
//!     .await
//!     .unwrap();
//! }
//! ```
//!
//! Please note that this example only works with the following [wit.ai backup](https://github.com/cliftontoaster-reid/wit_owo/blob/e16df111153b8278746007a817bd0843222dd263/owo/wit_ai.zip).
//!
//! ---
//!
//! Now all of you cuties,
//!
//! take care of yourselves,
//!
//! have a nice day, or night
//!
//! I don't know where you live ***YET***. Anyway bye.
//! <img src="https://pbs.twimg.com/media/EbxGWPSVcAADMPw?format=png&name=900x900" alt="Toaster with heart" style="width:200px;float: right;"/>
//!
//! Also yes, I use ChatGPT to rewrite my docs better because I'm a stupid toaster, now you all know my terrible secret.

pub mod constants;
pub mod model;
pub mod prelude;
