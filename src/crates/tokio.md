# 🌊 [tokio](https://crates.io/crates/tokio)
Tokio is an **asynchronous runtime** for Rust. It runs `async` tasks and provides tools for I/O, networking, timers, and synchronization.

## **Hello world with tokio**

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");

    sleep(Duration::from_secs(1)).await; // waits for the operation to finish while allowing Tokio to run other tasks

    println!("Finished");
}
```

## **Running tasks concurrently**
`tokio::spawn` starts each task on the runtime.

```rust
use tokio::time::{sleep, Duration};

async fn task(id: u8) {
    println!("Task {}: started", id);
    sleep(Duration::from_secs(2)).await;
    println!("Task {}: finished", id);
}

#[tokio::main]
async fn main() {
    let first = tokio::spawn(task(1));
    let second = tokio::spawn(task(2));

    first.await.unwrap();
    second.await.unwrap();

    println!("All tasks finished");
}
```
