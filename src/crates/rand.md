# 🎲 [rand](https://crates.io/crates/rand)

## 📗 **`random` Function**
`rand::random::<T>()` generates a random value of type `T`

For `f64`, the value is in the range `[0.0, 1.0)`: it can be 0.0, but is always less than 1.0.
```rust
# use rand::random;
# 
# fn main() {
println!("Random f64 betwwen 0 and 1: {}", random::<f64>());
println!("Random f64 betwwen 0 and 100: {}", random::<f64>() * 100.0);
# }
```

For unsigned (`uX`) and signed (`iX`) integer types, random generates a random integer across the type’s full range.
```rust
# use rand::random;
# 
# fn main() {
println!("Random u8: {}", random::<u8>());
println!("Random u32: {}", random::<u32>());

println!("Random i8: {}", random::<i8>());
println!("Random i32: {}", random::<i32>());
# }
```
## 📗 **`ThreadRng` Struct**
Behind the scenes, the `random` function relies on the `ThreadRng` Struct, short for Random Numer Generator (RNG). A RNG is a program that produces a sequence of random numbers (more than one). `ThreadRng` is a random number generator provided for the current thread. Import the `RngExt` trait to use methods such as random on it.
```rust
# use rand::{rng, RngExt};
# 
# fn main() {
let mut my_rng = rng();

println!("Random float: {}", my_rng.random::<f64>());

let numbers: Vec<i8> = (0..10)
    .map(|_| my_rng.random::<i8>())
    .collect();
println!("Random i8 numbers: {:?}", numbers);

println!("Random i32 in range: {}", my_rng.random_range(29..53));

println!("Random bool: {}", my_rng.random_bool(0.8));

println!("Random ratio: {}", my_rng.random_ratio(9, 13));
# }
```

## 📗 **`shuffle` Method**
`shuffle` randomly rearranges the elements in a mutable slice, changing their order in place. 
```rust
# use rand::rng;
# use rand::seq::SliceRandom;
# fn main() {
let mut my_rng = rng();

let mut candies = vec!["Kit Kat", "Twix", "Snickers", "Oreo", "Kinder"];

candies.shuffle(&mut my_rng);

println!("{:?}", candies);
# }
```