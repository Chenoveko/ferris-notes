# ⚡ Asynchronous 
[JSONPlaceholder](https://jsonplaceholder.typicode.com/)

[DummyJSON](https://dummyjson.com)

## **Async/Await**
```rust
use tokio::time::{sleep,Duration};

async fn greetings(id: u8) {
    println!("Task {} initiated", id);
    sleep(Duration::from_millis(500)).await;
    println!("Task {} done", id);
}

#[tokio::main]
fn main() {
    println!("Asynchronus main initiated!!!");
    
}
```
## **Futures and Tasks**

## **Async Runtimes**

