# Variables and Mutability 

## 📗 Variables
```rust
# fn main() {
let apples = 50; // Rust infers the type from the value
let oranges: i32 = 25;
let _fruits = apples + oranges; // '_' prefix suppresses the unused-variable warning
# }
```
## 📗 Interpolation using Curly Braces
Interpolation means inserting a value into a string. Rust provides three forms of interpolation:
1. Sequential placeholders
2. Variable capture
3. Positional arguments
```rust
# fn main() {
let apples = 50; 
let oranges = 25;
// Sequential Placeholders
println!("My garden has {} apples.", apples);
// Variable Capture 
println!("My garden has {apples} apples.");
// Positional arguments
println!(
        "My garden has {0} apples and {1} oranges. I can't believe I have {0} apples.",
        apples, oranges
);
# }
```
## 📗 Mutable and Immutable Variables
Variables are immutable by default. Value can change, type can't change
```rust
# fn main() {
let mut gym_reps = 10; 
println!("I plan to do {} reps.", gym_reps);
gym_reps = 12;
println!("Now I plan to do {} reps.", gym_reps);
# }
```
## 📗 Variable Shadowing
Variable shadowing refers to declaring a new variable with the same name. The new variable shadows the previous one and can have a different type.
```rust
# fn main() {
let grams_of_protein: &str = "100.345";
println!("Grams of protein in string type {}", grams_of_protein);
let grams_of_protein: f64 = 100.345;
println!("Grams of protein in float type {}", grams_of_protein);
let mut grams_of_protein: i32 = 100;
println!("Grams of protein in integer type {}", grams_of_protein);
grams_of_protein = 105;
println!("New grams of protein in integer type {}", grams_of_protein);
# }
```

## 📗 Scope
The scope is the boundary or region of code where a variable is valid
```rust
# fn main() {
let macchiato_price = 4.99;
// Block -> area between an opening curly brace and a closing curly brace
{
    let capuccino_price = 5.99;
    println!("Capuccino price -> {}$", capuccino_price);
    println!("Macchiato price -> {}$", macchiato_price);
} // cappuccino_price is out of scope here.
// println!("Capuccino price -> {}$", capuccino_price); ❌ capuccino_price out of scope
# }
```

## 📗 Type Aliases
Is an alternate name that we can assign to an existing type. Type aliases can also be declared at the top of the file, outside main, so they can be used by other functions within the module.
```rust
# fn main() {
type Meters = u32;
let mile_race_length: Meters = 1600;
println!("The race is {} miles long.", mile_race_length);
# }
```

## 📗 Compiler Attributes
A compiler attribute is metadata that provides instructions or information to the compiler. They can control compiler behavior, configure lints, mark tests, enable conditional compilation, etc. Attributes use the syntax `#[...]` for **outer attributes** or `#![...]`for **inner attributes**
    

### Outer Attributes
Applies to the item that comes immediately after it
```rust,noplayground
# fn main() {
#[allow(unused_variables)] // Applies to what comes NEXT
let x = 10; // The attribute applies only to `x`
# }
```

### Inner Attributes
Applies to the container where the attribute is written. If placed at the top of the file, it applies to the entire crate.
```rust,noplayground
# fn main() {
#![allow(unused_variables)] // Applies to what CONTAINS it
# }
```

## 📗 Constants
A constant is a name assigned to a value that cannot change. It's value must be known at compile time. Constants require an explicit type declaration.

Constants vs. immutable variables:
- Variables declared with let are scoped to the block where they are declared.
- Constants can be declared in any scope, including the global scope.
- Constants use const and require an explicit type.
```rust,noplayground
# fn main() {
const TAX_RATE: f32 = 0.5;
# }
```