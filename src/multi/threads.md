# **Threads**
[Medium: Multithreading in Rust](https://medium.com/@bektiaw/multithreading-in-rust-everything-you-need-to-know-054f97e4ab85)
## 📘 **Spawning and Joining Threads**
Use `std::thread::spawn` to start a new thread. The closure passed to `spawn` contains the code that the thread will run and returns a `JoinHandle`.

Call `handle.join()` to wait for the thread to finish. It returns a `Result`, so `unwrap()` will panic if the thread panicked.

```rust
# use std::thread;
# 
# fn main() {
let handle = thread::spawn(|| {
    println!("Hello from the thread!");
});

handle.join().unwrap();

println!("Thread finished!");
# }
```

In the example, the main and secondary threads run concurrently, so their messages may appear in different orders.
```rust
# use std::thread;
# use std::time::Duration;
# 
# fn main() {
let handle = thread::spawn(|| {
    for i in 0..5 {
        println!("Hello from secundary thread: {}", i);
        thread::sleep(Duration::from_millis(500));
    }
});

for i in 0..5 {
    println!("Hello from main thread: {}", i);
    thread::sleep(Duration::from_millis(500));
}

handle.join().unwrap();
# }
```

## 📘 **Spawning Multiple Threads**
We can create multiple threads by calling `thread::spawn()` multiple times. Each thread runs independently and can execute concurrently.
```rust
# use std::thread;
#
# fn main() {
// Store thread handles
let mut handles = vec![];

for i in 1..=10 {
    let handle = thread::spawn(move || {
        println!("Thread {i} is running");
    });
    // Adds each thread’s handle to the vector.
    handles.push(handle);
}

// Loops through all handles and waits for each thread to finish
for handle in handles {
    handle.join().unwrap();
}
# }
```

## 📘 **Handling Thread Panics**
Threads can panic just like the main thread. The difference is that a panic in a spawned thread is returned as an error when we call `join()`.

Previously, we used `.unwrap()` to keep the example simple. `.unwrap()` is fine for examples, but production code should handle thread failures intentionally.

```rust
# use std::thread;
# 
# fn main() {
let handle = thread::spawn(|| {
    panic!("Something went wrong!");
});

match handle.join() {
    Ok(_) => println!("Thread finished successfully"),
    Err(_) => println!("Thread panicked"),
}
# }
```
## 📘 **Moving Data into Threads**
A thread often needs to use data from the scope where it was created. Rust’s ownership rules prevent a spawned thread from borrowing local variables by default because the compiler cannot guarantee how long the thread will live.

```rust,ignore
let message = String::from("Hello");

let handle = thread::spawn(|| {
    println!("{message}"); // ❌ Error: borrowed value may not live long enough
});

handle.join().unwrap();
```
We can move ownership of data into the spawned thread using the `move` keyword.
```rust
# use std::thread;
# fn main() {
let message = String::from("Hello");

let handle = thread::spawn(move || { // Move ownership of `message` into the thread
    println!("{message}");
});

handle.join().unwrap();
# }
```

## 📙 **Message Passing with Channels**
Rust follows a philosophy popularized by Go and Erlang: 
> “Do not communicate by sharing memory; instead, share memory by communicating.”

Threads can communicate with each other using channels. A channel lets one thread send data while another thread receives it.

- `tx` is the sender used to send messages
- `rx` is the receiver used to receive messages
- `tx.send()` sends a value through the channel
- `rx.recv()` waits for and receives a value

```rust
# use std::sync::mpsc;
# use std::thread;
# use std::time::Duration;
# 
# fn main() {
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let mesagges = vec!["hello", "from", "the", "thread"];

    for msg in mesagges {
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
});

for rcv in rx {
    println!("Received: {}", rcv)
}
# }
```

### **Multiple Producers**
The `mpsc` (Multi Producer, Single Consumer) channel supports multiple producers, meaning multiple threads can send messages to the same receiver.

To send messages from multiple threads to a single receiver, clone the transmitter (`tx`):

```rust
# use std::sync::mpsc;
# use std::thread;
# use std::time::Duration;
# 
# fn main() {
let (tx, rx) = mpsc::channel();

// Create another sender for the first thread
let tx1 = tx.clone();

thread::spawn(move || {
    let vals = vec!["hi", "from", "thread", "one"];
    for val in vals {
        tx1.send(val).unwrap(); // Send a message
        thread::sleep(Duration::from_millis(200));
    }
});

thread::spawn(move || {
    let vals = vec!["more", "messages", "for", "you"];
    for val in vals {
        tx.send(val).unwrap(); // Send a message
        thread::sleep(Duration::from_millis(300));
    }
});

// Keep receiving until all senders are dropped
for received in rx {
    println!("Got: {received}");
}
# }
```


## 📕 **Sharing Data Between Threads**
When multiple threads need to read and modify the same data, message passing is not always ideal because it requires threads to communicate by sending data back and forth through channels, making the code more complex and adding unnecessary communication.

Shared-state concurrency is a way for multiple threads to access and modify the same data. Instead of sending data between threads through channels, threads share the data directly and use synchronization to make sure it is accessed safely.

### **`Arc<T>` and `Mutex<T>`**

Use `Arc<Mutex<T>>` when multiple threads need to share and safely update the same value:

- `Arc<T>` (Atomic Reference Counted) -> allows multiple threads to own the same value.
- `Mutex<T>` (Mutual Exclusion) -> allows only one thread at a time to access the value.
- `lock()` gives a thread access to the value. The lock is released when the guard goes out of scope.

```rust
# use std::thread;
# use std::sync::{Arc, Mutex};
# 
# fn main() {
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    // Clone the Arc so each thread can own a reference to the counter.
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

// Wait for every thread to finish before reading the counter
for handle in handles {
    handle.join().unwrap();
}   

println!("Result {}", *counter.lock().unwrap());
# }
```

### **`Mutex<T>` vs. `RwLock<T>`**

