# Trait Objects & Dynamic Dispatch
## 📕 **Trait Objects & Dynamic Dispatch**
A **trait object** is a value that can represent any type implementing a particular trait. Trait objects allow Rust to use **dynamic dispatch**, which means that the method to call is determined at runtime.

With dynamic dispatch, Rust does not need to know the concrete type at compile time. Instead, it determines the correct implementation when the method is called. This differs from **static dispatch**, where the compiler knows the exact type and method implementation during compilation.

Dynamic dispatch is generally slightly slower than static dispatch because the compiler cannot apply the same optimizations when the concrete type is unknown. However, it provides greater flexibility because different types can be treated uniformly through the same trait.

Trait objects are accessed through pointers, such as references or smart pointers:
```rust,ignore
&dyn Trait 
Box<dyn Trait>
```
A trait object can use only one main non-auto trait at a time. So if we want to implement 
```rust
# use std::collections::HashMap;
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("This is the default description")
    }
}

struct Hotel {
    name: String,
    reservations: HashMap<String, u32>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new()
        }
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights in the Hotel {}", guest, nights, self.name);
    }
}

impl Description for Hotel {}

struct AirBnB {
    owner: String,
    reservations: HashMap<String, u32>
}

impl AirBnB {
    fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            reservations: HashMap::new()
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights in the AirBnB of {}", guest, nights, self.owner);
    }
}

impl Description for AirBnB {}

fn main(){
    let hotel: Hotel = Hotel::new("Hotel Palace");
    let airbnb: AirBnB = AirBnB::new("Peter");
    
    // Trait object refer to the instances of '&hotel' and '&airbnb'
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());
}
```
A trait object can use only one main non-auto trait at a time. Therefore, this syntax is not valid:
```rust,ignore
let stays: Vec<&mut dyn (Description + Accommodation)> = vec![&mut hotel, &mut airbnb];
```
```rust
# use std::collections::HashMap;
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("This is the default description")
    }
}

struct Hotel {
    name: String,
    reservations: HashMap<String, u32>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new()
        }
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights in the Hotel {}", guest, nights, self.name);
    }
}

impl Description for Hotel {}

struct AirBnB {
    owner: String,
    reservations: HashMap<String, u32>
}

impl AirBnB {
    fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            reservations: HashMap::new()
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights in the AirBnB of {}", guest, nights, self.owner);
    }
}

impl Description for AirBnB {}

fn main(){
    let mut hotel: Hotel = Hotel::new("Hotel Palace");
    let mut airbnb: AirBnB = AirBnB::new("Peter");

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];

    stays[0].book("Tomas", 1);
    stays[1].book("Rose", 2);
}
```