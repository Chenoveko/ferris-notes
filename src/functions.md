# Functions
Rust code uses **snake case** as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
```rust,editable
fn another_function() {
    println!("Another function.");
}

fn main() {
    println!("Hello, world!");

    another_function();
}
```
## 📗 Functions with Parameters
A **parameter** is a name for an expected input to a function. An **argument** is the concrete value passed in for a parameter when the function is invoked
```rust,editable
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {}", neighborhood);
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {} {} pizzas!", number, topping);
}

fn main() {
    open_store("Brooklyn");
    bake_pizza(2, "pepperoni");
}
```
## 📗 Functions with Return Values
A return value is the output of a function
```rust,editable
// Explicit return value
fn square_explicit(number: i32) -> i32 {
    return number * number;
}

// Implicit return value
fn square_implicit(number: i32) -> i32 {
    number * number
}

// Unit return (empty tuple) -> let result: () = ();
fn mistery() {}

// Mix functions
fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn main() {
    println!("Explicit Square Return: {}", square_explicit(5));
    println!("Implicit Square Return: {}", square_implicit(5));
    println!("Mistery: {:?}", mistery());
    println!("Is Even: {}", is_even(8));
    println!("Is Even: {}", is_even(9));
    println!("Alphabets: {:?}", alphabets("antonio"));
}
```

## 📘 Statements and Expressions
Statements perform an action and don't return a value
```rust,noplayground
# fn main() {
let x = 5; 
# }
```
Expressions evaluate to a value
```rust,noplayground
# fn main() {
let y = {
        // Independent execution environment
        let x = 5;
        x + 1 // No semicolon -> returned by the block
    };
# }
```
