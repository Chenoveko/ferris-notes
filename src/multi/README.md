# 🧵 **Multithreading**

Multithreading means running multiple threads within one program. Each thread can work independently, or coordinate with other threads to share work and resources.

Before writing code, it’s important to understand these concepts:
- Concurrency
- Parallelism
- Race Conditions


## **Concurrency**

Concurrency means making progress on multiple tasks over the same period of time. Tasks may take turns on a single CPU core, or run at the same time on multiple cores.

**Example:** An application downloads a file while still responding to user input.

```text
Request A → ███░░░███
Request B → ░███░░░██
Request C → ░░███░░░█
```

## **Parallelism**

Parallelism means executing multiple tasks at the same time, usually on different CPU cores.

**Example:** A program splits a large calculation into parts and computes them on several cores.

> **Key difference:** Concurrency is about managing multiple tasks; parallelism is about doing multiple tasks at the same time.

```text
Core 1 → Task A ███████
Core 2 → Task B ███████
Core 3 → Task C ███████
```

## **Data Races**

A data race happens when threads access the same memory at the same time, at least one modifies it, and access is not synchronized.

Rust prevents data races in safe code through ownership, borrowing, and thread-safe types such as `Mutex<T>` and `Arc<T>`. The compiler checks these rules at compile time.