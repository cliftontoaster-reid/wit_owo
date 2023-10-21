# wit_owo

This crate has a goal to simplify the access to the free Natural Language software wit.ai.
Therefore, the solution found was creating a united library to manage all the API endpoints that this service provides.

If you are new to the Rust language, we recommend you starting with this example,
before trying to play around with the amazing language Rust is.

```rust
use wit_owo::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let owo = Client::new(&dotenv::var("WIT_AI").unwrap_or(env::var("WIT_AI").expect("For testing a .env must have WIT_AI set, a backup archive is located here https://github.com/cliftontoaster-reid/wit_owo/blob/master/owo/wit_ai.zip")));

  let uwu = owo
    .message("OwO what's this", DynamicEntities::default())
    .await
    .unwrap();
}
```

Note that this example only works with the following [wit.ai backup](https://github.com/cliftontoaster-reid/wit_owo/blob/e16df111153b8278746007a817bd0843222dd263/owo/wit_ai.zip).

---

If by any means, someone would like to help us with this project actively, we would love to work with others.
Thought we think that corresponding to plan together features and more would be better for everyone.
In that spirit for anyone willing to help, please contact the repertory's owner.

Anyway, we hope a great day for anyone reading this.

<img src="https://pbs.twimg.com/media/EbxGWPSVcAADMPw?format=png&name=900x900" alt="Toaster with heart" style="width:200px;float: right;"/>

---

Licences: MIT OR Apache-2.0 according to your preferences.
