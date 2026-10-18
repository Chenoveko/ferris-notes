# 🚀 Optimization

## 📗 Simple Rust Optimizations Rules

1. Stop allocating Memory you don't need -> Borrow instead fo clone **≈30% improved**
2. Avoid Repeating Work **≈12% improved**
3. Use every core you have -> Parallelize work **≈7 faster**
4. Use `swap_remove` on vectors when order dos not matter **≈7000 faster**

## 📗 Performance Lifecycle
> “Whe sould forget about small efficiencies, say about 97% of the time: Premature optimization is the root of all evil. Yet whe sould not pass up our opportunities in that critical 3%”  
> — Donald Knuth

1. **Measuring Performance**
2. **Isolate Bottlenecks**
3. **Optimizing Code**

**What to measure?**
- 💾 Memory (heap, stack)
- 💿 CPU time and hot paths
- 🛜 IO bottlenecks (disk, network)
- 📈 Throughput (e.g. requests per second)
- ⏳ Latency (e.g. response time ,tail latency)
- 🛠️ Compile time

**How to measure sync code?**
- [Hyperfine](https://github.com/sharkdp/hyperfine)
- [Flamegraph](https://github.com/brendangregg/flamegraph)
- [Dhat](https://github.com/nnethercote/dhat-rs)

**How to measure async code?**
- [Tracing](https://github.com/tokio-rs/tracing)
- [Tokio Console](https://github.com/tokio-rs/console)
- [Oha](https://github.com/hatoo/oha)


## 📗 Optimization Techniques
- Generics over dynamic dispatch
- Inline Critical Functions
- Using the CoW (Copy on Write) smart pointer
- Using libraries to parallelize work
    - [Rayon](https://github.com/rayon-rs/rayon) -> data-parallelism library
    - [Dashmap](https://github.com/xacrimon/dashmap) -> concurrent HashMap that lets multiple threads read and update entries safely.
    - [Crossbeam](https://github.com/crossbeam-rs/crossbeam) -> tools for threads, channels, and synchronization.
    - [MPI (Message Passing Interface)](https://github.com/rsmpi/rsmpi) -> parallel computing across multiple processes or machines, often used in HPC.
    - [ndarray](https://docs.rs/ndarray/latest/ndarray/) -> n-dimensional arrays and numerical operations, similar to NumPy.
    - [nalgebra](https://www.nalgebra.rs/) -> vectors, matrices, and linear algebra.
    - [argmin](https://docs.rs/argmin/latest/argmin/) -> numerical optimization algorithms.
