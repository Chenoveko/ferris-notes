# Introduction to the Ownership Model

## Scope and Ownership
When the variable goes out of scope, the owner deallocates the memory of that value
```rust,noplayground
# fn main() {
{
    let age: i8 = 32; // "age" variable is the owner of the value
} // "age" variable goes out of scope

# }
```

## Copy Trait
The copy trait mandates that a type can be copied, which means that a full duplicate can be created. Rust's primitive data types, or the fixed size ones that we store on the stack like integers,floats... implement this trait
```rust
# fn main() {
let time = 2025;
let year = time; // Full copy of time -> 2 duplicate, separate, independent copies of the value
// 2 owners, "time" is responsible for cleaning up it's stack entry and "year" for his entry
println!("The time is {}. The year is {}", time, year);
println!("The time address is {:p}. The year address is {:p}", &time, &year); // {:p} -> formats a reference/pointer as a memory address.
# }
```
For compound data types like array and tuples, they implement the **Copy Trait** when all of their elements implement it
```rust
# fn main() {
let x: (i8, i8) = (10, 20);
let y = x; // The tuple is copied

println!("x: {:?}", x);
println!("y: {:?}", y);
println!("The 'x' address is {:p}. The 'y' address is {:p}", &x, &y); 
# }
```

## Move of Ownership
A move is the transfer of ownership from one owner to another. One owner at a time, but the owner can change
```rust,noplayground
# fn main() {
let person: String = String::from("Boris");
let genius: String = person;
# }
```
What happens during the move?
- Ownership is moved from `person` to `genius` -> `person` is no longer valid
- The `String` metadata (pointer capacity and length) is copied, but the heap data is not duplicated

![`String` Memory Schema](https://miro.medium.com/v2/1*EkcBiNFlb5d2aHgVsgUG9A.png)

## Drop Function
`drop()` deallocates the memory on the heap
```rust,noplayground
# fn main() {
let person: String = String::from("Boris");
drop(person);
// println!("{}", person);  ❌ Error
# }
```

## Clone Method
`clone()` creates a deep copy of the value, including heap data.
```rust
# fn main() {
let person: String = String::from("Boris");
let genius: String = person.clone();
println!("Person: {}, Genius: {}", person, genius);
println!("The 'person' address is {:p}. The 'genius' address is {:p}", &person, &genius); 
# }
```


