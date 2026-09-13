# 🎯 **`Option enum`**
The `Option enum` models a scenario whree a type could be a valid value or nothing at all
- `Option::None` -> Represents an absent value
- `Option::Some(T)` -> Represents a present value

```rust,ignore
enum Option<T> {
    None,
    Some(T),
}
```
`Option<T>` implements **traits** such as `Debug`, `Clone`, and `Copy` when `T` implements the corresponding **trait**.
```rust
# fn main() {
let some: Option<i32> = Option::Some(5);
println!("Option<i32> Some: {:?}", some);

let some = Option::<f32>::Some(5.0);
println!("Option<f32> Some: {:?}", some);
  
let none: Option<i32> = Option::None;
println!("Option<i32> None: {:?}", none);
# }
```
A real example of `Option enum` is using the `get` **method** on an array
```rust
# fn main() {
let musical_instruments: [String; 3] = [
    String::from("Guitar"),
    String::from("Drums"),
    String::from("Bass"),
];

let bass: Option<&String> = musical_instruments.get(2);
let invalid_instrument: Option<&String> = musical_instruments.get(3);

println!("Bass: {:?}", bass);
println!("Invalid Instrument: {:?}", invalid_instrument);
# }
```

## 📗 **`unwrap` method**
- The `unwrap` **method** attempts to extract the associated data out of the `Some` **variant**
- `unwrap` always assume that there is something to `unwrap`
- use for developers to write faster and easier code
- is not the safest approach

```rust,editable
fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(3);

    println!("Bass unwrapped: {}", bass.unwrap());
    // println!("Invalid Instrument: {}", invalid_instrument.unwrap()); -> panic! ❌
}
```

## 📗 **`expect` method**
- The `expect` **method** is identical to `unwrap` but it allow us to customize the error message
- If the is `None` **variant**, the program will fail at runtime, displayin the custom error

```rust,editable
fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(3);

    println!("Bass expected: {}", bass.expect("Unable to retrieve musical instrument"));
    // println!("Invalid Instrument: {}", invalid_instrument.expect("Unable to retrieve musical instrument")); -> panic! ❌
}
```

## 📗 **`unwrap_or` method**
- Similar to `unwrap` **method**, but mandates an argument which represents the fallback value
- If the is `None` **variant**, the program will not fail, because there is a fallback value

```rust
fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];
    let default = String::from("Default");

    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(3);

    println!("Bass unwrap_or: {}", bass.unwrap_or(&default));
    println!("Invalid Instrument unwrap_or: {}", invalid_instrument.unwrap_or(&default));
}
```
## 📗 **`match` with `Option enum`**
```rust
# fn main() {
let musical_instruments: [String; 3] = [
    String::from("Guitar"),
    String::from("Drums"),
    String::from("Bass"),
];

let bass: Option<&String> = musical_instruments.get(2);
match bass {
    Option::Some(instrument) => println!("Playing the instrument {} in my band", instrument),
    Option::None => println!("Singing with my voice"),
}

match musical_instruments.get(3) {
    Option::Some(instrument) => println!("Playing the instrument {} in my band", instrument),
    Option::None => println!("Singing with my voice"),
}
# }
```

Refactoring with a **function** that implements the `match` statement

```rust
fn get_musical_instrument(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => {
            println!("Playing the instrument {} in my band", instrument)
        }
        Option::None => println!("Singing with my voice"),
    }
}

fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(3);

    get_musical_instrument(bass);
    get_musical_instrument(invalid_instrument);
}
```


## 📘  **Return `Option enum` from a function**
```rust
# fn main() {
fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
            Option::Some(false)
    } else {
            Option::None
    }
}

match is_item_in_stock(true, true) {
    Some(true) => println!("Item is available"),
    Some(false) => println!("Item is falsable"),
    None => println!("Item is not available"),
}
# }
```


## 📘  **`Option` Variants and the Prelude**
The **Rust prelude** is a collection of commonly used names that Rust automatically brings into scope in every program. The `Some` and `None` variants of `Option<T>` are included in the prelude, so we can use them directly without writing `Option::Some` or `Option::None`.

```rust,ignore
match is_item_in_stock(true, true) {
    Option::Some(true) => println!("Item is available"),
    Option::Some(false) => println!("Item is falsable"),
    Option::None => println!("Item is not available"),
}
```
Using the variants imported by the **Rust prelude**
```rust,ignore
match is_item_in_stock(true, true) {
    Some(true) => println!("Item is available"),
    Some(false) => println!("Item is falsable"),
    None => println!("Item is not available"),
}
```


## 📙 **`if let` Construct**
Instead of using `match` statement, if I'm only interested in `Some` **variant** use the `if let` **contruct** 
```rust
# fn main() {
let musical_instruments: [String; 3] = [
    String::from("Guitar"),
    String::from("Drums"),
    String::from("Bass"),
];

if let Some(instrument) = musical_instruments.get(2) {
    println!("Playing {}", instrument);
}

if let Some(instrument) = musical_instruments.get(3) {
    println!("Playing {}", instrument);
}

if let None = musical_instruments.get(3) {
    println!("Playing nothing!");
}
# }
```

## 📙 **`let ... else` Construct**
Instead of using `match` statement, if I'm only interested in `Some` **variant** use the `let ... else` **contruct** 
```rust
fn play_instrument(instrument: Option<&String>) {
    // We need an instrument to continue
    let Some(instrument) = instrument else {
        println!("No instrument available");
        return;
    };

    // `instrument` is available from this point
    println!("Playing {}", instrument);
}

fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    play_instrument(musical_instruments.get(2));
    play_instrument(musical_instruments.get(3));
}
```

## 📙 **`while let` Construct**
`while let` repeatedly checks whether a value matches a pattern. With `Option`, it is useful when you want to keep processing values while they are `Some`.

The `pop` **method** removes and returns the last element from a vector. It returns
`Some(value)` when an element is available and `None` when the vector is empty.

Example with `if let` **construct**
```rust
# fn main() {
let mut sauces = vec!["Mayonaise", "Ketchup", "BBQ"];

// 'if let' executes the block only if .pop() successfully returns a value (Some).
if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce)
}

if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce)
}
if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce)
}
# }
```
A cleaner approach with `while let` **Construct**
```rust
# fn main() {
let mut sauces = vec!["Mayonaise", "Ketchup", "BBQ"];

// 'while let' loops as long as .pop() keeps returning Some(value).
// It automatically stops when the vector is empty and .pop() returns None.
while let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce);
}
# }
```