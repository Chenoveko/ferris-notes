# 🏗️ **Structs**
A `struct` (structure) is a container for related pieces of data. Similar to an object in POO. We use structs to model complex real-world types
We use **PascalCase** for Structs names and **snake_case** for fields in the struct

Unlike classes in traditional OOP languages, Rust separates data from behavior:
- `struct` defines the data.
- `impl` blocks define methods and associated functions.
- `traits` define shared behavior.

However, Rust does support several important object-oriented ideas:

- **Encapsulation** through modules and privacy.
- **Abstraction** through structs and traits.
- **Polymorphism** through traits.
- **Methods** associated with custom types.

Rust has 3 kinds of `structs`:
- **Named Field Structs**: each field has a name.
- **Tuple-Like Structs**: fields are accessed by position.
- **Unit-Like Structs**: structs without fields.

**Named-field Structs**
```rust
#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    // Creating an instance
    let mut coffee = Coffee {
        price: 2.5,
        name: String::from("Espresso"),
        is_hot: true,
    };

    println!("My coffe: {coffee:?}");

    // Accessing a field
    println!("The coffee costs ${}", coffee.price);

    // Overwrite struct fields
    coffee.name = String::from("Vegan latte");
    coffee.price = 3.10;
    coffee.is_hot = false;
    println!("My new coffe: {coffee:?}");
}
```
A `struct` is the owner of it's fields and each field is the owner of it's corresponding value

**Tuple-Like Structs**
```rust
struct Point(i32, i32);

fn main() {
    // Creating an instance
    let point = Point(10, 20);

    // Accessing a field
    println!("x: {}", point.0);
    println!("y: {}", point.1);
}
```

**Unit-like Structs**
Unit-like structs are often used as marker types or to implement a trait
```rust,ignore
struct Marker;
```

## 📘  **Adding Behavior to the Struct**
Both **methods** and **associated functions** are defined inside an `impl` block

The key difference is:
- A **method** receives `self` in some form, so it operates on a specific instance.
- An **associated** function does not receive `self`, so it is associated with the type itself rather than with a particular instance.

### 📗 **Methods**
A function defined inside an `impl` block is a method when its first parameter is `self` in some form.

In our example we are going to the define the following struct:
```rust,ignore
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}
```
There are 4 possible ways to pass `self`:
- Immutable struct value (`self` parameter takes ownership) 
```rust,ignore
fn display_song_info(self) {
    println!("Title: {}", self.title);
    println!("Release year: {}", self.release_year);
    println!("Duration: {} seconds", self.duration_secs);
}
```
- Mutable struct value (`self` parameter takes ownership, has permission to mutate) 
```rust,ignore
fn double_length(mut self) {
    let previous_length = self.duration_secs;
    self.duration_secs *= 2;
    println!("Previous length: {} seconds. New length: {} seconds", previous_length, self.duration_secs);
}
```
- Immutable reference to the struct instance (no ownership moved) 
```rust,ignore
fn display_song_info_ref(&self) {
    println!("Title: {}", self.title);
    println!("Release year: {}", self.release_year);
    println!("Duration: {} seconds", self.duration_secs);
}
```
- Mutable reference to the struct instance (no ownership moved, has permission to mutate) 
```rust,ignore
fn double_length_ref(&mut self) {
    self.duration_secs *= 2;
}
```
We can have multiple impl blocks, is totally valid
```rust,ignore
impl TaylorSwiftSong {
    fn display_title(&self) {
        println!("Title: {}", self.title);
    }
}
impl TaylorSwiftSong {
    fn display_release_year(&self) {
        println!("Release Year: {}", self.release_year);
    }
}
```
We can have **methods** with multiple paramaters
```rust,ignore
fn is_longer_than(&self, other_song: &Self) -> bool {
    self.duration_secs > other_song.duration_secs
}
```
We can have **methods** calling other methods
```rust,ignore
fn years_since_release(&self) -> u32 {
    2026 - self.release_year
}

fn display_year_since_release(&self) {
    println!("Years since release: {}", self.years_since_release());
}
```
All in one:
```rust
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl TaylorSwiftSong {
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }
    
    fn double_length(mut self) {
        let previous_length = self.duration_secs;
        self.duration_secs *= 2;
        println!("Previous length: {} seconds. New length: {} seconds", previous_length, self.duration_secs);
    }

    fn display_song_info_ref(&self) {
        println!("Title: {}", self.title);
        println!("Release year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length_ref(&mut self) {
        self.duration_secs *= 2;
    }

    fn is_longer_than(&self, other_song: &Self) -> bool {
        self.duration_secs > other_song.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2026 - self.release_year
    }

    fn display_year_since_release(&self) {
        println!("Years since release: {}", self.years_since_release());
    }
}


fn main() {
    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.display_song_info();

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.double_length();

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.display_song_info_ref();
    println!("{:?}", song);

    let mut song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.double_length_ref();
    println!("{:?}", song);

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    let other_song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Opalite"), release_year: 2016, duration_secs: 320 };
    println!("{}", song.is_longer_than(&other_song));
    song.display_year_since_release();
}
```

`Self` is an alias for the type being implemented. In this case, `Self` refers to `TaylorSwiftSong`. Therefore, these 3 method definitions are equivalent:
```rust,ignore
fn display_song_info(self: TaylorSwiftSong) {}
fn display_song_info(self: Self) {}
fn display_song_info(self) {}
```
### 📘 **Associated Functions**
Associated functions are functions that are attached to a type

Examples:
- `String::from()`
- `String::new()`
   
We often use associated functions for constructors. A **constructor** is a function that return a new instance of a type
```rust
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl TaylorSwiftSong {
    // Constructor
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self { title, release_year, duration_secs }
    }
}


fn main() {
    let blank_space: TaylorSwiftSong = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);
    println!("{:?}", blank_space);
}
```

## 📙  **Builder Pattern**
A design pattern is a recommended way to write or structure code to solve specific problems. Useful when you would otherwise require many constructors or where construction has side effects.

**Advantages**
- Separates methods for building from other methods.
- Prevents proliferation of constructors.
- Can be used for one-liner initialisation as well as more complex construction.
- When you add new fields to the target struct, you can update the builder to leave client code backwards compatible.

**Disadvantages** -> More complex than creating a struct object directly, or a simple constructor function.
```rust
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32
}

impl Computer {
    // Constructor
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self { cpu, memory, hard_drive_capacity }
    }

    // Methods
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self{
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }

}


fn main() {
    let mut laptop: Computer = Computer::new(String::from("M3 Max"), 64, 128);
    laptop  
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(86)
        .upgrade_hard_drive_capacity(256);
    println!("My upgrade laptop {:?}", laptop);
}
```
This pattern is seen more frequently in Rust (and for simpler objects) than in many other languages because Rust lacks overloading and default values for function parameters. Since you can only have a single method with a given name, having multiple constructors is less nice in Rust than in C++, Java, or others.










