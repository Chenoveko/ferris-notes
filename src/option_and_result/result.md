# 🎯 **`Result enum`**
The `Result enum` models the outcome of an evaluation that can produce either a success or an error
- `Result::Ok(T)` -> Indicates a success. It stores an associated piece of data of **generic** type `T`
- `Result::Err(E)` -> Indicates an error. It stores an associated piece of data of **generic** type `E`

```rust,ignore
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`Result<T, E>` implements **traits** such as `Debug`, `Clone`, and `Copy` when `T` and `E` implements the corresponding **trait**.
```rust
# fn main() {
let ok: Result<i8, &str> = Result::Ok(8);
let disaster: Result<i8, &str> = Result::Err("Disaster");
println!("ok: {:?} and disaster: {:?}", ok, disaster);
# }
```
A real example of `Result enum` is using the `parse` **method**
```rust
# fn main() {
let text: &str = "50";
println!("text as number: {:?}", text.parse::<i8>());
println!("text as bool: {:?}", text.parse::<bool>());
# }
```

## 📘 **`as_ref` method**
`unwrap`, `expect` and `unwrap_or` are **methods** designed to consume the Result. This means that in their definition, they take `self` by value, taking **ownership** of the variable. 
```rust,ignore
fn unwrap(self) -> T
fn expect(self, message: &str) -> T
fn unwrap_or(self, default: T) -> T
```
After calling one of these methods, the original `Result` can no longer be
used because it has been moved:
```rust,ignore
let result: Result<String, String> = Ok(String::from("Success"));

let value = result.unwrap();

// `result` cannot be used here because it was moved.
```
The `as_ref()` **method** allows us to **borrow** the contents of the `Result` before
calling `unwrap`, `expect` or `unwrap_or`. It converts:
```rust,ignore
Result<T, E> -> Result<&T, &E>
```
Therefore, the original `Result` remains available:
```rust
let result: Result<String, String> = Ok(String::from("Success"));

let value: &String = result.as_ref().unwrap();

println!("{value}");
println!("{result:?}"); // The Result can still be used
```
## 📗 **`is_ok` and `is_err` method**
This **methods** make a **borrow** automatically, they don't need `as_ref()`
```rust
# fn main() {
let result: Result<String, String> = Ok(String::from("Success"));

let value: &String = result.as_ref().unwrap();

println!("result Ok?: {}", result.is_ok());
println!("result Err?: {}", result.is_err());

println!("{result:?}"); // The Result can still be used
# }
```
## 📗 **`unwrap` method**
- The `unwrap` **method** attempts to extract the associated data out of the `Ok` **variant**
- It assumes that the `Result` contains `Ok`.
- If the `Result` contains `Err`, `unwrap` causes the program to panic.
- It can make code shorter and easier to write, but it is not always the safest
  way to handle errors.
- Use it when failure is impossible, when writing examples or tests, or when a
  panic is acceptable.
```rust
# fn main() {
let text: &str = "50";

let text_parsed_as_i8 = text.parse::<i8>(); // Result<i8, std::num::ParseIntError>
let text_parsed_as_bool = text.parse::<bool>(); // Result<bool, std::str::ParseBoolError>

println!("Unwrap text parse as i8: {}", text_parsed_as_i8.as_ref().unwrap());
// println!("Unwrap text parse as i8: {}", text_parsed_as_bool.as_ref().unwrap()); -> panic! ❌
# }
```
## 📗 **`expect` method**
- The `expect` **method** is identical to `unwrap` but it allow us to customize the error message
- If the is `Err` **variant**, the program will fail at runtime, displayin the custom error
```rust
# fn main() {
let text: &str = "50";

let text_parsed_as_i8 = text.parse::<i8>(); // Result<i8, std::num::ParseIntError>
let text_parsed_as_bool = text.parse::<bool>(); // Result<bool, std::str::ParseBoolError>

println!("Expect text parse as i8: {}", text_parsed_as_i8.as_ref().expect("Unable to parse"));
// println!("Expect text parse as bool: {}", text_parsed_as_bool.as_ref().expect("Unable to parse")); -> panic! ❌
# }
```
## 📗 **`unwrap_or` method**
- Similar to `unwrap` **method**, but mandates an argument which represents the fallback value
- If the is `Ok` **variant**, `unwrap_or` returns the value inside `Ok`.
- If the is `Err` **variant**, the program will not panic, because there is a fallback value
```rust
# fn main() {
let text: &str = "50";

let text_parsed_as_i8 = text.parse::<i8>(); // Result<i8, std::num::ParseIntError>
let text_parsed_as_bool = text.parse::<bool>(); // Result<bool, std::str::ParseBoolError>

// `as_ref()` changes the value type from `i8` to `&i8`,so the fallback value must also be a reference: `&12`
println!("Expect text parse as i8: {}", text_parsed_as_i8.as_ref().unwrap_or(&12));

// `as_ref()` changes the value type from `bool` to `&bool`, so the fallback value must also be a reference: `&true`
println!("Expect text parse as bool: {}", text_parsed_as_bool.as_ref().unwrap_or(&true));
# }
```
## 📗 **`match` with `Result enum`**
```rust
# fn main() {
let text: &str = "50";

let text_parsed_as_i8 = text.parse::<i8>(); // Result<i8, std::num::ParseIntError>
let text_parsed_as_bool = text.parse::<bool>(); // Result<bool, std::str::ParseBoolError>

match text_parsed_as_i8 {
    Ok(value) => println!("Value: {}", value),
    Err(message) => println!("Error message: {}", message),
}

match text_parsed_as_bool {
    Ok(value) => println!("Value: {}", value),
    Err(message) => println!("Error message: {}", message),
}
# }
```


## 📘  **Return `Result enum` from a function**
```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by 0".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

fn main() {
    println!("divide by 0: {:?}", divide(20.0, 0.0));
    println!("divide ok: {:?}", divide(20.0, 12.0));

    match divide(20.0, 1.0) {
        Ok(calculation) => println!("calculation: {}", calculation),
        Err(message) => println!("Error message: {}", message),
    }

    match divide(20.0, 0.0) {
        Ok(calculation) => println!("calculation: {}", calculation),
        Err(message) => println!("Error message: {}", message),
    }
}
```


## 📘  **`Result` Variants and the Prelude**
The **Rust prelude** is a collection of commonly used names that Rust automatically brings into scope in every program. The `Ok` and `Err` variants of `Result<T, E>` are included in the prelude, so we can use them directly without writing `Result::Ok` or `Result::Err`.

```rust,ignore
match divide(20.0, 1.0) {
    Result::Ok(calculation) => println!("calculation: {}", calculation),
    Result::Err(message) => println!("Error message: {}", message),
}
```
Using the variants imported by the **Rust prelude**
```rust,ignore
match divide(20.0, 1.0) {
    Ok(calculation) => println!("calculation: {}", calculation),
    Err(message) => println!("Error message: {}", message),
}
```

## 📙 **`if let` Construct**
Instead of using `match` statement, if I'm only interested in `Ok` **variant** use the `if let` **contruct** 
```rust
# fn main() {
let text: &str = "50";

let parsed_number = text.parse::<i32>();

if let Ok(number) = parsed_number {
    println!("The number is {number}");
}
# }
```
You can also check the `Err` **variant**:
```rust
# fn main() {
let text: &str = "hello";

let parsed_bool = text.parse::<bool>();

if let Err(error) = parsed_bool {
    println!("Parsing failed: {error}");
}
# }
```
`if let` executes the block only when the pattern matches.

## 📙 **`let ... else` Construct**
Use `let ... else` when you need an `Ok` value to continue:
```rust
fn print_number(text: &str) {
    let Ok(number) = text.parse::<i32>() else {
        println!("The text is not a valid number");
        return;
    };

    // `number` is available from this point onward.
    println!("The number is {number}");
}

fn main() {
    print_number("50");
    print_number("hello");
}
```
If the `Result` contains `Err`, the `else` block runs and must exit the current **scope**.

## 📙 **`while let` Construct**
`while let` repeatedly checks whether a value matches a pattern. With `Result`, it is useful when you want to keep processing values while they are `Ok`.

With an **iterator** of `Result` values, it can be used to process values while the
**iterator** returns `Some(Ok(value))`.

The `next()` method returns:
```rust,ignore
Option<Result<T, E>>
```
Example processing only successful values:
```rust
# fn main() {
let results: Vec<Result<i32, &str>> = vec![
        Ok(10),
        Ok(20),
        Ok(30),
    ];

// `into_iter()` converts the vector into an iterator and takes ownership of the vector
let mut iter = results.into_iter(); 

while let Some(Ok(number)) = iter.next() {
    println!("Processing number: {number}");
}
# }
```
Handling both `Ok` and `Err`
```rust
# fn main() {
let results: Vec<Result<i32, &str>> = vec![
        Ok(10),
        Err("Invalid value"),
        Ok(30),
    ];

let mut iter = results.into_iter(); // iterator

while let Some(result) = iter.next() {
    match result {
        Ok(number) => println!("Processing number: {number}"),
        Err(error) => println!("An error occurred: {error}"),
    }
}
# }
```

