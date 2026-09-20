# **Fundamentals**

## 📗 **Defining a `trait`**
A `trait` defines shared behavior that a type can implement. It acts as a contract: any type that implements the trait must provide an implementation for all required methods.
```rust,ignore
trait Accomodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}
```
## 📗 **Implementing a `trait` for a `struct`**
An `impl Trait for Type` block provides the behavior required by a trait for a specific type.
```rust
# use std::collections::HashMap;
trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
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
    // Calling trait method from other method
    fn summarize(&self) -> String {
        format!("Name: {}, {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("🏨 Welcome to {}!", self.name)
    }
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights", guest, nights);
    }
}

fn main(){
    let mut palace: Hotel = Hotel::new("Hotel Palace");

    println!("{}", palace.get_description());
    println!("{}", palace.summarize());

    palace.book("Alice", 3);
    palace.book("Bob", 2);
}
```
## 📗 **Implementing multiple `traits` for a `struct`**
A type can implement multiple traits. Each trait can describe a different aspect of the type's behavior.
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
        println!("✅ {} booked a room for {} nights.", guest, nights);
    }
}

impl Description for Hotel {}

fn main(){
    let mut palace: Hotel = Hotel::new("Hotel Palace");

    palace.book("Tom", 1);
    println!("{}", palace.get_description());
}
```

## 📗 **Default Method Implementations**
A trait can provide a default implementation for a method. Implementing types can use that implementation without writing their own version, or override it with custom behavior.
```rust
# use std::collections::HashMap;
trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
    fn default_impl(&self) -> String {
        String::from("This is the default implementation")
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
    fn get_description(&self) -> String {
        format!("🏨 Welcome to {}!", self.name)
    }
    fn book(&mut self, guest: &str, nights: u32) {
        self.reservations.insert(guest.to_string(), nights);
        println!("✅ {} booked a room for {} nights.", guest, nights);
    }
}

fn main(){
    let mut palace: Hotel = Hotel::new("Hotel Palace");

    println!("{}", palace.default_impl());
}
```
A type can override the default implementation when necessary
```rust,ignore
impl Accommodation for Hotel { 
    fn default_description(&self) -> String { 
        format!("A room at {}", self.name) 
    } 
}
```
## 📗 **Associated Constants in a `trait`**
An **associated constant** is a fixed, immutable value declared inside a trait. It can be shared by all implementations or overridden by a specific type.
```rust
trait Taxable {
    const TAX_RATE: f64 = 0.25; 
    fn tax_bill(&self) -> f64;
}

struct Income {
    amount: f64
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

struct Bonus {
    amount: f64
}

impl Taxable for Bonus {
    // Override the default TAX_RATE
    const TAX_RATE: f64 = 0.3; 

    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}


fn main() {
    let income: Income = Income { amount : 5000.05 };
    let bonus: Bonus = Bonus { amount : 5000.05 };

    // Income uses the default tax rate defined in the Taxable trait
    println!("Total tax owed for income: {:.2}", income.tax_bill());

    // Bonus overrides the default value and uses a tax rate of 0.30 instead
    println!("Total tax owed for bonus: {:.2}", bonus.tax_bill());
}
```
## 📘 **Traits with Generics**
A trait can use a generic type parameter. This allows the trait to work with different types depending on the implementation. The generic type can be used as a method parameter, a return type, or both. Therefore, different types can implement the same trait with different concrete types.
```rust
trait Taxable<T> {
    const TAX_RATE: f64 = 0.25; 
    fn tax_bill(&self) -> T;
}

struct Income {
    amount: f64
}

impl Taxable<f64> for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

struct Bonus {
    amount: f32
}

impl Taxable<f32> for Bonus {
    fn tax_bill(&self) -> f32 {
        self.amount * (Self::TAX_RATE as f32)
    }
}

fn main() {
    let income: Income = Income { amount : 5000.05 };
    let bonus: Bonus = Bonus { amount : 5000.05 };

    println!("Total tax owed for income: {:.2}", income.tax_bill());
    println!("Total tax owed for bonus: {:.2}", bonus.tax_bill());
}
```
## 📘 **Associated Types in a `trait`**
An **associated type** is a type placeholder defined inside a trait. It represents a type that each implementation must specify.
```rust,ignore
trait Taxable { 
    type Output; 
    const TAX_RATE: f64 = 0.25; 
    fn tax_bill(&self) -> Self::Output; 
}
```
An associated type is similar to a generic type parameter, but the type is defined inside the implementation rather than being supplied as part of the trait name.
```rust
trait Taxable {
    type Output;
    const TAX_RATE: f64 = 0.25; 
    fn tax_bill(&self) -> Self::Output;
}

struct Income {
    amount: f64,
}

impl Taxable for Income {
    type Output = f64;

    fn tax_bill(&self) -> Self::Output {
        self.amount * Self::TAX_RATE
    }
}

struct TaxReport {
    amount: f64,
}

impl Taxable for TaxReport {
    type Output = String;

    fn tax_bill(&self) -> Self::Output {
        let tax = self.amount * Self::TAX_RATE;

        format!("The total tax is ${:.2}", tax)
    }
}

fn main() {
    let income = Income { amount: 5000.05 };
    let report = TaxReport { amount: 5000.05 };

    println!("Income tax: {:.2}", income.tax_bill());
    println!("{}", report.tax_bill());
}
```

## 📙 **Generic Parameters vs. Associated Types**
Both generic parameters and associated types allow a trait to work with different types. The main difference is who defines the type and how many implementations are possible.

**Generic type parameters**

With a generic trait, the type is part of the trait implementation:
```rust,ignore
trait Convert<T> { 
    fn convert(&self) -> T; 
} 

impl Convert<i32> for Value { 
    // ..
} 

impl Convert<String> for Value {
    // ...
}
```
The same type can implement the trait more than once with different generic parameters, as long as the implementations do not conflict.

The type is visible when using the trait:
```rust,ignore
Convert<i32>
Convert<String>
```

**Associated types**

With an associated type, the implementation defines the type internally:
```rust,ignore
trait Convert { 
    type Output; 
    fn convert(&self) -> Self::Output; 
} 

impl Convert for Value {
    type Output = i32; 
    fn convert(&self) -> Self::Output { 
        10 
    } 
}
```
A type can only implement a trait once when associated types are used. This guarantees that the associated type is unambiguous.