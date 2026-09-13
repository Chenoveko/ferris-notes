# ➡ Vectors
- A **vector** is a collection type for storing homogeneous elements in order
- A **vector** is similar to an array but it can grow and shrink in size
- Each element is assigned an index position reflecting its place in line. The index starts counting at 0
- Rust's vector type is `Vec`. It has one **generic** which represents the type of the stored elements
- `Vec<T>` is include in Rust prelude
- Implement `Debug` **trait** (not `Display` **trait**)
- If we know the starter values, we can use the macro `vec!` to create the vector

## 📗 **Creating a Vector**
`Vec::new()` creates an empty vector. Rust needs to know the type of its elements, either through a type annotation or the turbofish operator.
```rust
# fn main() {
let steven_spielberg_movies: Vec<&str> = Vec::new();
let pizza_diameters = Vec::<i8>::new();

println!("{steven_spielberg_movies:?}");
println!("{pizza_diameters:?}");
# }
```
We can use the `vec![]` macro to create a vector containing initial values.
```rust
# fn main() {
let pizza_diameters: Vec<i8> = vec![25, 35]; 

println!("{pizza_diameters:?}");
# }
```
## 📗 **`push` method**
Appends an element to the end of the **vector**
```rust
# fn main() {
let mut pizza_diameters: Vec<i8> = vec![25, 35]; 

println!("Pizza diameters: {:?}", pizza_diameters);

pizza_diameters.push(45);

println!("Pizza diameters after push: {:?}", pizza_diameters);
# }
```

## 📗 **`insert` method**
Inserts an element at a specific index. The existing elements are shifted to the right. It panics at runtime if the index is greater than the vector's length.
```rust
# fn main() {
let mut pizza_diameters: Vec<i8> = vec![25, 35]; 

println!("Pizza diameters: {:?}", pizza_diameters);

pizza_diameters.insert(0, 56);

println!("Pizza diameters after insert: {:?}", pizza_diameters);
# }
```
`insert` panics at runtime if the index is greater than the vector's length.
## 📗 **`pop` method**

Attempts to remove the last element from the **vector**. It return an `Option<T>`
- `Some(value)` if the vector contained an element.
- `None` if the vector was empty.
```rust
# fn main() {
let mut pizza_diameters: Vec<i8> = vec![25, 35]; 

println!("Pizza diameters: {:?}", pizza_diameters);

pizza_diameters.pop();

println!("Pizza diameters after pop: {:?}", pizza_diameters);
# }
```

## 📗 **`remove` method**
Removes and returns an element at a specific index. The elements after it are shifted to the left. It panics at runtime if the index is invalid.

```rust
# fn main() {
let mut pizza_diameters: Vec<i8> = vec![25, 35]; 

println!("Pizza diameters: {:?}", pizza_diameters);

pizza_diameters.remove(0);

println!("Pizza diameters after remove: {:?}", pizza_diameters);
# }
```
## 📗 **Ownership with Vectors**
When we create a **vector** with owned values such as `String`, ownership of those values moves into the vector.
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

// Ownership moves from each variable into the vector
let pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara]; 

// pepperoni, barbecue, diavola, and carbonara can no longer be used because 
// their ownership was moved into the vector

println!("Pizza toppings: {:?}", pizza_toppings);
# }
```
## 📗 **Reading Vector Elements using indexing**
Indexing accesses an element through a reference. If the element implements the `Copy` **trait**, such as `i32`, Rust copies the value when it is assigned to another variable.
```rust
# fn main() {
let pizza_diameters: Vec<i32> = vec![25, 35, 45, 55];

// Rust create a full copy of the element from the vector
let value: i32 = pizza_diameters[2]; 

// A slice is a reference to part of the vector, not a copy
let pizza_slice: &[i32] = &pizza_diameters[1..3]; 

println!("Value of pizza diameter[2]: {} | Pizza diameters vector: {:?}", value, pizza_diameters);
println!("Pizza diameter slice: {:?}", pizza_slice);
# }
```
Types such as `String` do not implement the `Copy` **trait**. Therefore, we cannot move a `String` out of the vector using indexing. Instead, we borrow it using `&`
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

let pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara];

// We borrow the String instead of moving it out of the vector
let value: &String = &pizza_toppings[2]; 

// A slice is a reference to part of the vector
let pizza_slice: &[String] = &pizza_toppings[1..3]; 

println!("Reference of pizza toppings[2]: {} | Pizza toppings vector: {:?}", value, pizza_toppings);
println!("Pizza toppings slice: {:?}", pizza_slice);
# }
```
It is usually more idiomatic to borrow a string as `&str`:
```rust,ignore
let value: &str = &pizza_toppings[2];
```
## 📗 **Reading Vector Elements using `get` method**
The `get` **method** extracts a vector element by index position. It returns an `Option` enum. The `Some` **variant** will store a reference to the value
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

let pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara];

match pizza_toppings.get(2)  {
    Some(topping) => println!("The topping is {}", topping),
    None => println!("No value at the index position")
}
# }
```

## 📗 **Writing Vector Elements using indexing**
A **vector** must be **mutable** to modify its elements. When we assign a new value using indexing, the existing value at that position is replaced.
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

let mut pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara];

println!("Pizza toppings before writing: {:?}", pizza_toppings);

// The String at index 0 is replaced
pizza_toppings[0] = String::from("Hawaiana");

println!("Pizza toppings after writing: {:?}", pizza_toppings);
# }
```
We can also **borrow** an element mutably and modify its existing value.
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

let mut pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara];
println!("Pizza toppings before writing: {:?}", pizza_toppings);

// We create a mutable reference to the String at index 0
let target_topping: &mut String = &mut pizza_toppings[0];

// We modify the existing String
target_topping.push_str(" (Pineapple)");

println!("Pizza toppings after writing with mut reference: {:?}", pizza_toppings);
# }
```
## 📗 **Writing Vector Elements using `get_mut` method**
The `get_mut` **method** attempts to return a mutable reference to an element at a specific index.

It returns an `Option<&mut T>`:
- `Some(&mut value)` if the index is valid.
- `None` if the index is out of bounds.
```rust
# fn main() {
let pepperoni: String = String::from("Pepperoni");
let barbacue: String = String::from("Barbacue");
let diavola: String = String::from("Diavola");
let carbonara: String = String::from("Carbonara");

let mut pizza_toppings: Vec<String> = vec![pepperoni, barbacue, diavola, carbonara];
println!("Pizza toppings before writing: {:?}", pizza_toppings);

if let Some(target_topping) = pizza_toppings.get_mut(0) {
    target_topping.push_str(" with pineapple");
}

println!("Pizza toppings after writing: {pizza_toppings:?}");
# }
```
`get_mut` is safer than indexing because it does not **panic** if the index is invalid.
## 📗 **Vector Capacity**
A vector has two important properties:
- **Length**: the number of elements currently stored in the vector.
- **Capacity**: the amount of space currently allocated for elements.

Capacity is not the maximum number of elements that a vector can contain. When the vector needs more space, Rust automatically allocates a larger memory region and may move the existing elements to that new location.

The exact capacity growth strategy is implementation-dependent. It often grows by more than the required amount to make future insertions more efficient.

```rust
# fn main() {
// Create a vector with an initial capacity of 4
let mut seasons: Vec<&str> = Vec::with_capacity(4);

println!("Legth: {} | Capacity: {} | Seasons Vector: {:?} | Memory Address: {:p}", seasons.len(), seasons.capacity(), seasons, seasons.as_ptr());

// Add four elements to the vector
seasons.push("Summer");
seasons.push("Fall");
seasons.push("Winter");
seasons.push("Spring");

println!("Legth: {} | Capacity: {} | Seasons Vector: {:?} | Memory Address: {:p}", seasons.len(), seasons.capacity(), seasons, seasons.as_ptr());

// The vector needs more capacity for this element
seasons.push("Other");

println!("Legth: {} | Capacity: {} | Seasons Vector: {:?} | Memory Address: {:p}", seasons.len(), seasons.capacity(), seasons, seasons.as_ptr());
# }
```
When the fifth element is added, the capacity increases from 4 to 8. This value is only an example; the exact growth amount is not guaranteed.

The memory address may stay the same if the allocator can extend the existing allocation in place. Therefore, a changed capacity does not necessarily mean that the address shown by `as_ptr()` will change.