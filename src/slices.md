# 🍞 **Slices**

- A collection/compound type is one that can hold multiple values. Arrays, tuples, and strings are collection types
- A slice is a reference to a portion/sequence of a collection type. It's a subcategory of reference
- A string slice is a reference to a sequence of characters from a string
- An array slice is a reference to a sequence of elements from an array 
- A tuple slice is a reference to a sequence of elements from a tuple
- As a reference, a slice does not take ownership of the collection

## 📗 **String Slice from a String**
A string slice (`&str`) is a borrowed view into part—or all—of a `String`. It does not copy the original data.
```rust
# fn main() {
let action_hero: String = String::from("Arnold Schwarzenegger");

// String Reference to heap allocated string
let action_hero_reference: &String = &action_hero;

// String Slice, with syntactic shortcut &action_hero[..6];
let first_name: &str = &action_hero[0..6]; 

// A slice from the beginning of the last name to the end
let last_name: &str = &action_hero[7..]; 

// A slice containing the entire String
let full_name: &str = &action_hero[..]; 
println!("Action hero: {}", action_hero_reference);
println!("Action hero first name: {}", first_name);
println!("Action hero last name: {}", last_name);
println!("Action hero full name: {}", full_name);
# }
```

##  📘 **String Slices and String Literals** 
String literal (`&str`) contains a reference to a piece of text stored in the binary executable

String literals have a `static` lifetime, which means they remain available for the entire lifetime of the program.
```rust
# fn main() {
let action_hero: &str = "Arnold Schwarzenegger";

let first_name: &str = &action_hero[0..6]; // String Slice 
let last_name: &str = &action_hero[7..]; // String Slice 

println!("Action hero: {}", action_hero);
println!("Action hero first name: {}", first_name);
println!("Action hero last name: {}", last_name);
# }
```
This following example does not create create a dangling reference because the string literal is stored in the binary executable and lives for the entire program
```rust
# fn main() {
let first_name: &str = {
        let action_hero: &str = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

println!("Action hero first name: {}", first_name);
# }
```

## 📗 **String Slice Lengths**
⚠️ String slices use **byte ranges**, not character indices. The range boundaries must correspond to valid UTF-8 character boundaries. This previous examples works because all characters are ASCII and each ASCII character uses 1 byte. 
```rust
# fn main() {
let food: &str = "pizza"; 
let food_slice: &str = &food[0..3]; 
let food_emoji: &str = "🍕"; // The pizza emoji 🍕 uses 4 bytes.
// let food_emoji_slice: &str = &food_emoji[0..2]; Panicked, not a valid UTF-8 character boundary

// The .len() method returns the number of bytes, not the number of characters.
println!("Food Length: {}", food.len());
println!("Food Slice Length: {}", food_slice.len());
println!("Food Emoji Length: {}", food_emoji.len());

// To count Unicode characters, use:
println!("Food character count: {}", food.chars().count());
println!("Food emoji character count: {}", food_emoji.chars().count());
# }
```

## 📘 **String Slices as Function Parameters**
The most versatile way to use strings in functions is with string slices

```rust
# fn main() {
fn do_hero_stuff(hero_name: &str) {
    println!("{} saves the day 🦸‍♂️", hero_name);
}

let action_hero: String = String::from("Arnold Schwarzenegger");

// Rust automatically converts &String into &strthrough deref coercion
do_hero_stuff(&action_hero); 

let another_action_hero: &str = "Sylvester Stallone";
do_hero_stuff(another_action_hero);
# }
```
`&str` is preferred over `&String` for function parameters because it is more flexible and does not require ownership or a new allocation.

Rust automatically converts &String into &str through **deref coercion**:
```rust,ignore
&String → &str
```
However, Rust does not automatically convert a string slice into a reference to a `String`:
```rust,ignore
&str ↛ &String
```
Therefore, this function is more restrictive:
```rust,ignore
fn do_hero_stuff(hero_name: &String) {
    println!("{} saves the day 🦸‍♂️", hero_name);
}
```

## 📗 **Array Slices**
```rust,noplaygroud
# fn main() {
let my_array: [i32; 6] = [1, 2, 3, 4, 5, 6];

// Reference to some chunk or portion of an array
let array_slice: &[i32] = &my_array[0..2]; 

// Reference to full array
let full_array_slice: &[i32] = &my_array[..]; 

// Reference to a 6 element of array of i32 (more restricted)
let array_ref: &[i32; 6] = &my_array; 
# }
```

## 📘 **Deref Coercion with Array Slices**

```rust
# fn main() {
fn print_array_length(reference: &[i32]) {
    println!("array len: {} ", reference.len());
}

let my_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
let array_slice: &[i32] = &my_array[0..2];
let full_array_slice: &[i32] = &my_array[..]; 
let array_ref: &[i32; 6] = &my_array; 

// Rust converts &[i32; N] to &[i32] through deref coercion.
print_array_length(&my_array); 
print_array_length(array_slice);
print_array_length(full_array_slice);
print_array_length(array_ref); 
# }
```

## 📗 **Mutable Array Slices**
Rust does not permit mutable slices of string. However Rust does permit mutable slices of arrays
```rust
# fn main() {
let mut mutable_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
let mut_array_slice: &mut [i32] = &mut mutable_array[2..4];

mut_array_slice[0] = 35;
println!("My array modified using mut slices: {:?}", mutable_array);
# }
```