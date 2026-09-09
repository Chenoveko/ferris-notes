# Introduction to References and Borrowing
A reference allows the program to use a value without moving ownership. Borrowing means using something without taking ownership of it (`&` borrow operator)
```rust,noplayground
# fn main() {
let car: String = String::from("Toyota");
let car_reference: &String = &car;
println!("My car: {}", car_reference);
# }
```
More technically, a reference is a type of pointer. In Rust, a reference is guaranteed to point to a valid value for the lifetime of that reference. In comparison, a raw pointer does not have that guarantee.

## Dereference Operator 
An operator is a symbol that applies an operation to a value. To dereference means to access the data at the memory address that the reference point to. The dereference operator (`*`) operates on a reference.

Borrow operator (`&`) vs deference operator (`*`)
- The `&` operator creates a reference.
- The `*` operator dereferences a reference.
```rust
# fn main() {
let my_value: i8 = 2;
let my_reference = &my_value;
println!("{}", my_value); // i8  -> Display
println!("{}", *my_reference); // &i8 -> i8 -> Display
println!("{}", my_reference); // &i8 -> Display works through the reference
# }
```

## Introduction to String Types
- String literal -> hardcoded, read-only piece of text encodes in the binary
    - Embedded directly into the binary executable (know at compile time)
    - Has type &str -> reference to that hard coded text
- String Slice (&str) -> reference to the text in the memory that has loaded the binary file (fixed content)
- String -> dynamic piece of text stored on the heap at runtime
    - can grow/change
    - Value lives on the heap
    - Stack entry (3 pieces of data) -> pointer/reference, length  and capacity
- &String -> reference to a heap String
```rust,noplayground
# fn main() {
let food: &str = "pasta"; // String Literal 
let text: String = String::new(); // Creates an empty String
let candy: String = String::from("KitKat"); // Creates a String from a string literal
# }
```
With the String method `push_str()` we can append a string slice (&str) to a String
```rust
# fn main() {
let mut name: String = String::from("Graydon");
println!("String Metadata -> Pointer: {:p} | Length: {} | Capacity {}", name.as_ptr(), name.len(), name.capacity());
name.push_str(" Hoare");
println!("New String Metadata -> Pointer: {:p} | Length: {} | Capacity {}", name.as_ptr(), name.len(), name.capacity());
# }
```

The pointer may remain the same after the String grows if the allocator can extend the existing heap allocation in place.

However, this behavior is not **guaranteed**. If the existing allocation cannot be extended, Rust allocates a new memory block, copies the text to it, and updates the pointer.

Therefore:

- the length can change
- the capacity can change
- the pointer may remain the same or may change

## Copy Trait with References
Remember that stack types implement the Copy Trait. Refrences in Rust implement the Copy Trait as well
```rust
# fn main() {
let ice_cream: &str = "Cookies and Cream";
let ice_cream_copy = ice_cream; // Copying a reference creates another reference to the same data.
println!("My ice cream: {:p}. My ice cream copy: {:p}", ice_cream, ice_cream_copy);
# }
```
