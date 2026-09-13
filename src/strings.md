# 🔗 Strings
## 📗 **Strings in Rust**
A string is a piece of text or alternatively, a sequence of characters. In Rust we have different types of string:
- **String**
  - Dynamic piece of text stored on the heap at runtime
  - Owned string
  - Can grow/shrink and change
  - Owns its heap data
  - Its value lives on the heap
  - Its stack entry contains three pieces of data:
    - Pointer/reference
    - Length
    - Capacity
```rust,ignore
// Creates an empty String
let empty_string: String = String::new(); 

// Creates a String from a string literal
let pirate: String = "Jack Sparrow".to_string(); 

// Creates a String from a string literal
let action_hero: String = String::from("Arnold Schwarzenegger"); 
```
- **Reference to a String (`&String`)** -> Reference to a heap-allocated `String`.
```rust,ignore
let action_hero_reference: &String = &action_hero;
```
- **String Slice (`&str`)**
  - Reference to text in the memory where the binary file is loaded.
  - Has fixed content.
```rust,ignore
// String slice containing "Arnold"
let first_name: &str = &action_hero[0..6]; 

// String slice containing "Schwarzenegger"
let last_name: &str = &action_hero[7..]; 

// String slice containing the entire String, this is still &str, not &String
let full_name: &str = &action_hero[..]; 
```
- **String Literal (`&str`)**
  - Hardcoded, read-only piece of text encoded in the binary
  - Embedded directly into the binary executable
  - Known at compile time
  - References the hardcoded text
```rust,ignore
let food: &str = "pasta"; 
```

## 📗 **Concatenation**
**Appending a string slice with `push_str`**

The `push_str` **method** appends a string slice (`&str`) to the end of a `String`.
```rust
# fn main() {
let mut action_hero: String = String::from("Arnold");

println!("Action Hero Incomplete: {}", action_hero);

action_hero.push_str(" Schwarzenegger");

println!("Action Hero Complete: {}", action_hero);
# }
```
`push_str` **borrows** the string slice, so it does not take **ownership** of it.

**Appending a character with `push`**

The `push` **method** appends a single character (`char`) to the end of a `String`
```rust
# fn main() {
let mut action_hero: String = String::from("Arnold");

println!("Action Hero Incomplete: {}", action_hero);

action_hero.push('🦸');

println!("Action Hero Complete: {}", action_hero);
# }
```

**Concatenating with `add` method**

The **method** takes **ownership** of the first `String` and **borrows** the second string as a string slice:
```rust
use std::ops::Add;

fn main() {
    let first_name: String = String::from("Graydon");
    let last_name: String = String::from(" Hoare");

    let rust_creator: String = first_name.add(&last_name);

    println!("Rust Creator: {}", rust_creator);

    // first_name can no longer be used because its ownership was moved into the new string
}
```
The `+` calls the `add` **method**. 
```rust
# fn main() {
let first_name: String = String::from("Graydon");
let last_name: String = String::from("Hoare");

let rust_creator: String = first_name + &last_name; 

println!("Rust Creator: {}", rust_creator);
// first_name can no longer be used because its ownership was moved into the new string
# }
```
## 📗 **`format!` Macro**
The `format!` **macro** works similarly to `println!`, but it returns a formatted `String` instead of printing it.
- It creates a new `String`.
- It supports interpolation and formatting.
- The values passed to it are borrowed, so their ownership is not moved.
- It is one of the clearest ways to concatenate multiple strings.
```rust
# fn main() {
let first_name: String = String::from("Graydon");
let last_name: String = String::from("Hoare");

let rust_creator: String = format!("{} {}", first_name, last_name);

println!("Rust Creator Formatted: {}", rust_creator);
# }
```
## 📗 **`trim`, `trim_start` and `trim_end`  Methods**
These **methods** return a string slice with whitespace removed.
- `trim` removes whitespace from the beginning and the end.
- `trim_start` removes whitespace from the beginning.
- `trim_end` removes whitespace from the end.
```rust
# fn main() {
let music_genres: &str = "         Rock, Jazz, Blues, Classical, Pop         ";

println!("Trimmed Music Genres: {}", music_genres.trim());

println!("Trimmed Music Genres at beginning: {}", music_genres.trim_start());

println!("Trimmed Music Genres at end: {}", music_genres.trim_end());
# }
```
These **methods** do not modify the original string. They return a borrowed `&str`.
## 📗 **`to_uppercase` Method**
The `to_uppercase` **method** returns a new `String` containing the text converted to uppercase.
```rust
# fn main() {
let music_genres: &str = "Rock, Jazz, Blues, Classical, Pop";

println!("Uppercase Music Genres: {}", music_genres.to_uppercase());
# }
```
## 📗 **`to_lowercase` Method**
The `to_lowercase` **method** returns a new `String` containing the text converted to lowercase.
```rust
# fn main() {
let music_genres: &str = "Rock, Jazz, Blues, Classical, Pop";

println!("Uppercase Music Genres: {}", music_genres.to_lowercase());
# }
```
## 📗 **`replace` Method**
The `replace` **method** replaces all occurrences of a pattern with another string.

It returns a new `String` and does not modify the original string.
```rust
# fn main() {
let music_genres: &str = "Rock, Jazz, Blues, Classical, Pop";

println!("Replaced Music Genres: {}", music_genres.replace("Jazz", "Hip Hop"));
# }
```

## 📘 **`split` Method**
The `split` **method** divides a string into an **iterator** of substrings based on a delimiter.
```rust
# fn main() {
let music_genres: &str = "Rock, Jazz, Blues, Classical, Pop";

let music_genres_split: Vec<&str> = music_genres.split(", ") // => Iterator
    .collect(); // => Vec<&str>

println!("Split Music Genres: {:?}", music_genres_split);
# }
```
The `split` method returns an **iterator**. The collect method converts that **iterator** into a `Vec<&str>`.

The resulting string slices borrow from `music_genres`, so they do not own their text.
## 📗 **Collecting User Input with `read_line` Method**
The `read_line` **method** reads a line from standard input and appends it to a mutable String.
```rust
use std::io::stdin;

fn main() {
    let mut name: String = String::new();

    println!("Please enter your name: ");

    match stdin().read_line(&mut name) {
        Ok(_) => println!("Hello {}!", name.trim()),
        Err(error) => println!("Error reading input: {}", error)
    }
}
```
`read_line` includes the newline character entered by the user. We use `trim()` to remove the newline and any surrounding whitespace.