# Advanced References and Borrowing

##  References

- A reference stores the memory address of a value
- Borrowing means creating a reference
- References enable the reuse of data with moving ownership

##  Immutable References

- Reference are immutable my default
- An immutable reference does not have permission to modify the original value at the memory address
- A value can have any number of immutable references. There is no risk
- Immutable references implement the copy trait. Rust will create a full copy in situations where one is needed (variable assignment, function parameters, variable inside array, etc)

Function with an immutable reference; it can read the value but cannot modify it, and it does not take ownership.
```rust,editable
fn show_my_meal(meal: &String) {
    println!("Show my meal: {}", meal);
}

fn main() {
    let breakfast: String = String::from("Chocapic");
    show_my_meal(&breakfast);
}
```

Rust permits any number of immutable references to the same value
```rust
# fn main() {
let car: String = String::from("Ferrari");
let car_ref1: &String  = &car;
let car_ref2: &String  = &car;
println!("My references to the car: {} and {}", car_ref1, car_ref2);
# }
```

##  Mutable References

- An mutable reference has permission to modify the original value at the memory address
- A value can only have one mutable reference at a time
- Mutable references do not implement the copy trait. Ownership will move on variable reassigment
- The compiler understands the references's lifetime, which is the time it is being utilized in the program. A lifetime can end before the function's scope

Function with a mutable reference; it can read and modify the value, but it does not take ownership.
```rust,editable
fn add_flour(meal: &mut String) {
    meal.push_str(" Add flour");
}

fn main() {
    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    println!("{current_meal}")
}
```

Rust only permits a single mutable references to the same value at a time
```rust,editable
fn main() {
    let mut motorbike: String = String::from("Kawasaki");
    let motorbike_ref1: &mut String  = &mut motorbike;
    motorbike_ref1.push_str(" Ninja");
    // let motorbike_ref2: &String = &motorbike; An immutable and a mutable reference cannot exist at the same time.
    // println!("My motorbike references: {}", motorbike_ref1, motorbike_ref2);

    // A new mutable reference can be created after the previous reference is no longer active.
    let motorbike_ref3: &mut String = &mut motorbike;
    motorbike_ref3.push_str(" ZX10R");
        
    println!("My motorbike: {}", motorbike);
}
```
Lifetimes of references is a complex topic cover later in the book

##  Dangling References

- A dangling references is a pointer to a memory addres that has been deallocated
- Dangling references create bugs and unpredictable behaviors in other programming languages
- The Rust compiler prevents dangling references. A reference is guaranteed to point to valid data
- The reference (the original data) must outlive the reference  

This function does not compile because `city` is dropped when the function ends
```rust,ignore
fn create_city() -> &String {
    let city: String = String::from("New York");
    &city // Return dangling reference
}
```

##  Ownership with Immutable and Mutable References
Immutable references implement the copy trait 
```rust
# fn main() {
let coffee: String = String::from("Mocha");
let coffee_ref1: &String = &coffee;
let coffee_ref2 = coffee_ref1; // Full copy of the reference
println!("Coffee refs: {} and {}", coffee_ref1, coffee_ref2);
println!("Coffee refs pointers: {:p} and {:p}", coffee_ref1, coffee_ref2);
# }
```

Mutable references do not implement the copy trait 
```rust
# fn main() {
let mut soda: String = String::from("Cola");
let soda_ref1: &mut String = &mut soda;
let soda_ref2 = soda_ref1; // Ownership of the mutable reference is moved
soda_ref2.push_str(" Zero");
println!("Soda: {}", soda);
# }
```

##  Ownership with Immutable and Mutable References of Compound Data Types
When the elements implement the `Copy` trait, the value is copied
```rust
# fn main() {
let registrations: [bool; 3] = [true, false, true];
let first_registration: bool = registrations[0]; // Bool implements the Copy trait, so the value is copied from the array

println!("First registration: {}", first_registration);
println!("Registrations: {:?}", registrations);
# }
```

When the elements don't implement the `Copy` trait, we borrow tha value
```rust
# fn main() {
let languages: [String; 3] = [String::from("Rust"), String::from("Go"), String::from("Python")];
let first_language: &String = &languages[0]; // String does not implement the Copy trait, so we borrow the value instead of moving it
println!("First language: {}", first_language);
println!("Languages: {:?}", languages);
# }
```

