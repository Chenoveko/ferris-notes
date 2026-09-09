# Basics of Traits
- A trait is a contract that requires that a type support one or more methods
- Traits establish consistency between types; methods that represent the same behavior have the same name
- When a type opts in to honoring a trait's requirements, we say the type implements the trait
- Types can vary the implementation but still implement the same trait
- A type can choose to opting in to implementing a trait
- A type can implement multiple traits. There are hundreds of traits available in Rust
- A trait is called an interface or protocol in other programming languages

## 📘 Display Trait
- The Display trait requires that a type can be represented as a user-friendly, readable string
- The Display trait mandates a format method that returns the string
- When we use the {} interpolation syntax, Rust relies on the format method
- Integers, floats and booleans will implement the Display trait so we are able to interpolate them with curly braces
- It is not always clear how a complex type should be represented as a piece of text
- Not all types implement the Display trait. Example: arrays, tuples and ranges
```rust
# fn main() {
println!("Display Trait for integer: {}", 42);
println!("Display Trait for floats: {}", 42.05);
println!("Display Trait for booleans: {}", true);
# }
```

## 📘 Debug Trait
- The Debug trait is used for developer-oriented representations of values
- `{:?}` -> Debug Formatting
- `{:#?}` -> Pretty-Printing Debug Formatting
- Arrays tuples and ranges implement Debug Trait
```rust
# use std::ops::Range;
# use std::ops::RangeInclusive;
# fn main() {
let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
println!("Debug Formatting for apples: {:?}", apples);
println!("Pretty-Printing Debug Formatting for apples: {:#?}", apples);
let employee: (&str, i32, &str) = ("Molly", 32, "Marketing");
println!("Employee: {:?}", employee);
let week_days: Range<i32> = 1..7; 
let week_days_inclusive: RangeInclusive<i32> = 1..=7;
println!("Week Days: {:?}", week_days);
println!("Week Days Inclusive: {:?}", week_days_inclusive);
# }
```

### Debug Macro
- Prints and returns the value of a given expression for quick and dirty debugging (for development)
- It uses the Debug Traits's format method to output several helpfull deatils about the content we pass in here
- The argument that we pass to the dbg! macro must implement the Debug Trait so that Rust can print out
- The `dbg!` macro writes its output to standard error (`stderr`).
```rust,noplayground
# fn main() {
let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
dbg!(2 + 2);
dbg!(seasons);
# }
```