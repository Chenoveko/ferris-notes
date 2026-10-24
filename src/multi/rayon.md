# [rayon](https://crates.io/crates/rayon)
For CPU-bound work over large collections, we don’t need to manage threads manually.

Rayon makes data parallelism much simpler. It can split work across multiple threads and manage the thread pool for us.
```rust
# use rayon::prelude::*;
# 
# fn main() {
let data: Vec<i32> = (0..1_000_000).collect();

let sum: i64 = data
    .par_iter() // Process items in parallel
    .map(|&x| x as i64 * 2)
    .sum();

println!("Sum: {sum}");
# }
```
Rayon turns sequential iterators into parallel computations using a work-stealing thread pool. The key difference is simple:
```rust,ignore
data.iter()      // Sequential
data.par_iter()  // Parallel
```
With .`par_iter()`, Rayon distributes the work across multiple threads automatically.

```rust
# use rayon::prelude::*;
# use std::time::Instant;
#
# fn main() {
let data: Vec<i32> = (0..1_000_000).collect();

// Without rayon
let start = Instant::now();

let sum_sequential: i64 = data
    .iter()
    .map(|&x| x as i64 * 2)
    .sum();

let sequential_time = start.elapsed();

// Wit rayon
let start = Instant::now();

let sum_parallel: i64 = data
    .par_iter()
    .map(|&x| x as i64 * 2)
    .sum();

let parallel_time = start.elapsed();

println!("Sequential sum: {}", sum_sequential);
println!("Sequential time: {}", sequential_time.as_millis());

println!("Rayon sum: {}", sum_parallel);
println!("Rayon time: {}", parallel_time.as_millis());
# }
```
