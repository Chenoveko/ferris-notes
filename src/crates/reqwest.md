# 🌐 [reqwest](https://crates.io/crates/reqwest)

```rust
use std::error::Error;
use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let response = get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .text()
        .await?;

    println!("Response: {}", response);
    Ok(())
}
```
