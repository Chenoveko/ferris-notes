# ⚡ **Asynchronous**

[JSONPlaceholder](https://jsonplaceholder.typicode.com/)

[DummyJSON](https://dummyjson.com)

[Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## 📘 **Async/Await**
An `async fn` returns a **future**. A future does not do work until it is polled by an async runtime. The `.await` keyword waits for a future to make progress, yielding control to the runtime while it is waiting.
```rust,ignore
use tokio::time::{sleep,Duration};

async fn greetings(id: u8) {
    println!("Task {} initiated", id);
    sleep(Duration::from_millis(500)).await;
    println!("Task {} done", id);
}

#[tokio::main]
async fn main() {
    println!("Asynchronus main initiated!!!");

    greetings(1).await;
    greetings(2).await;
}
```
Here, the calls run sequentially: `greetings(2)` starts after `greetings(1)` finishes.
## 📘 **Futures and Concurrency**
You can wait for multiple futures together with `tokio::join!`. They make progress concurrently, but `join!` does not create separate Tokio tasks.
```rust,ignore
use tokio::time::{sleep,Duration};

async fn greetings(id: u8) {
    println!("Task {} initiated", id);
    sleep(Duration::from_millis(500)).await;
    println!("Task {} done", id);
}

#[tokio::main]
async fn main() {
    println!("Async main started");

    // Sequential execution
    greetings(1).await;
    greetings(2).await;

    // Concurrent execution in the same task
    let (first, second) = tokio::join!(
        greetings(3),
        greetings(4),
    );

    println!("All work finished");
}
```
`join!` returns a tuple containing each future’s output. In this example, both functions return `()`, so the tuple values are unused. You can write `tokio::join!(...)` without assigning the result if you do not need it.
## 📘 **Async Runtimes**
Rust provides the `async` and `await` language features, but an async runtime is needed to drive futures forward. Tokio is a runtime that schedules async tasks and provides async tools such as timers, networking, and synchronization.

The `#[tokio::main]` macro sets up a Tokio runtime and runs the `async main` function on it. Tokio’s default runtime uses multiple worker threads; you can configure runtime features in your `Cargo.toml`.

## 📕 **Communication Between Async Functions**
An `mpsc` channel sends values from a producer to a consumer. A oneshot channel sends a single value, which is useful for a shutdown signal.
```rust,ignore
use tokio::{
    sync::{mpsc, oneshot},
    time::{sleep, Duration},
};

async fn producer(tx: mpsc::Sender<i8>, mut shutdown_rx: oneshot::Receiver<()>) {
    for i in 1..=10 {
        // Check whether a shutdown signal has arrived.
        match shutdown_rx.try_recv() {
            Ok(()) => {
                println!("Producer: stop signal received");
                break;
            }
            Err(oneshot::error::TryRecvError::Closed) => {
                println!("Producer: shutdown channel closed");
                break;
            }
            Err(oneshot::error::TryRecvError::Empty) => {}
        }

        if tx.send(i).await.is_err() {
            println!("Producer: receiver was dropped");
            break;
        }

        println!("Producer sent: {}", i);
        sleep(Duration::from_millis(200)).await;
    }

    println!("Producer: Closing Channel");
}

async fn consumer(mut rx: mpsc::Receiver<i8>) {
    while let Some(value) = rx.recv().await {
        println!("Consumer received: {}", value);
        sleep(Duration::from_millis(300)).await;
    }

    println!("Consumer: channel closed");
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(5);

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    let prod_handle = tokio::spawn(producer(tx, shutdown_rx));
    let cons_handle = tokio::spawn(consumer(rx));

    tokio::spawn(async move {
        sleep(Duration::from_secs(2));
        let _ = shutdown_tx.send(());
        println!("Shutdown signal sent");
    });

    prod_handle.await.unwrap();
    cons_handle.await.unwrap();
}
```
**What is happening?**
- `mpsc::channel(5)` creates a channel that can buffer up to five messages. The producer sends values; the consumer receives them. If the buffer fills, `send().await` waits for space.
- `oneshot::channel()` creates a channel for one message. Here, that message tells the producer to stop.
- `try_recv()` checks the shutdown channel without waiting. It returns Empty if no signal has arrived yet.
- `sleep(...).await` pauses the async task while allowing Tokio to schedule other tasks.
- When the producer finishes, its Sender is dropped. The consumer drains any buffered values, then `recv().await` returns `None` and the loop ends.

## 📕 **Async File System Operations**
Tokio provides async file operations through `tokio::fs`. These operations can yield while waiting, allowing other async tasks to make progress.
```rust,ignore
use tokio::{fs, io};

async fn file_io_example() -> io::Result<()> {
    let path = "async_text.txt";
    let content = "Hello from tokio async I/O";

    fs::write(path, content).await?;
    println!("File write with {}", content);

    let text = fs::read_to_string(path).await?;
    println!("File read with {}", text);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = file_io_example().await {
        eprintln!("File I/O error: {error}");
    }
}
```
