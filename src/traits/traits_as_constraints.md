# Traits as Constraints
## 📙 **Traits for Function Parameter Constraints**
Traits can be used to restrict function parameters to types that implement specific behavior.

### **`impl Trait` Syntax**
The `impl Trait` syntax allows a function to accept any type that implements a specific trait.
```rust,ignore
fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    // entity can be a mutable reference to any type that implements Accommodation
    entity.book(guest, 1);
}
```
### **Generic Trait Bound Syntax**
The same function can be written using the **Trait Bound Syntax**. This syntax uses a **generic** `type` restricted by a trait bound:
```rust,ignore
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    // 'T' can represent any type that implements Accommodation
    entity.book(guest, 1);
}
```
### **`where` Clause Syntax**
A `where` clause provides an alternative syntax for writing trait bounds. It is especially useful when a function has multiple or complex constraints.
```rust,ignore
fn book_for_one_night<T>(entity: &mut T, guest: &str)
where
    T: Accommodation
{
    // 'T' can represent any type that implements Accommodation
    entity.book(guest, 1);
}
```
The `where` clause separates the generic parameters from their trait constraints, making the function easier to read.
### **Multiple Trait Bounds**
A function parameter can require a type to implement multiple traits.

**Using `impl Trait`**
```rust,ignore
fn describe_and_book(entity: &mut (impl Accommodation + Description), guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}
```
**Using Trait Bound Syntax**
```rust,ignore
fn describe_and_book<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}
```
**Using a `where` clause**
```rust,ignore
fn describe_and_book<T>(entity: &mut T, guest: &str) 
where
    T: Accommodation
{
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}
```
## 📕  **Traits for Function Return Values Constraints**
Traits can also constrain the return type of a function. In this case we  only have one possible syntax. This is the `impl Trait`.
```rust,ignore
// The function chooses the concrete return type
fn choose_best_place() -> impl Accommodation {
    Hotel::new("Luxury") // The concrete type is visible here
    // The caller see the signature of the funcion, so only know that the return type implements Accomodation
}

...

// The caller can see that the return type implements `Accommodation`,
// but cannot see the exact concrete type in the function signature
let place = choose_best_place(); 
```
**Trait Bound Syntax** and `where` clause are not valid, because with this syntax the caller choose the return type `T`
```rust,ignore
fn choose_best_place<T: Accommodation>() -> T {
    // The function return any type T that implements Accommodation
    // but we return Hotel
    Hotel::new("Luxury") 
}

...

// The caller choose Hotel type
let place_1: Hotel = choose_best_place(); 

// The caller choose AirBnB type. Rust does not know how to create an arbitrary T
let place_2: Airbnb = choose_best_place(); 
```
```rust,ignore
fn choose_best_place<T>() -> T
where
    T: Accommodation
{
    Hotel::new("Luxury")
}

...

let place_1: Hotel = choose_best_place(); 
let place_2: Airbnb = choose_best_place();
```
Therefore, use `impl Trait` when a function returns a specific concrete type but hides that type from its callers in the function signature.

## 📕 **Trait Bounds on `impl` Blocks**
A trait bound can be applied to an entire `impl` block. This means that the methods inside the block are only available when the generic type satisfies the specified constraint.
```rust
# use std::collections::HashMap;
# use std::fmt::Display;
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("This is the default description")
    }
}

struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

// This implementation is available for every type T
impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name: name,
            reservations: HashMap::new()
        }
    }
}

// These methods are only available when T implements Display trait
impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("🏨 Hotel: {}", self.name)
    }
}

// The Accommodation trait is implemented for every Hotel<T>
impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} night(s).", guest, nights);
    }
}

// Description requires T to implement Display because
// the name is included in the returned String.
impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("This is the hotel named {}.", self.name)
    }
}

fn main(){
    let mut palace: Hotel<&str> = Hotel::new("Hotel Palace");

    println!("{}", palace.summarize());
    println!("{}", palace.get_description());

    palace.book("Alice", 3);
    palace.book("Bob", 2);
}
```