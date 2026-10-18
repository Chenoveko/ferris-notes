# Rust’s Inspirations
![rust inspritations](https://i.ytimg.com/vi/784JWR4oxOI/hq720.jpg?sqp=-oaymwEnCNAFEJQDSFryq4qpAxkIARUAAIhCGAHYAQHiAQoIGBACGAY4AUAB&rs=AOn4CLA1XeUURZucYDEiL9sSYf8yO6rgbA)
## Zero-Cost Abstractions
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/1/18/ISO_C%2B%2B_Logo.svg/1280px-ISO_C%2B%2B_Logo.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>

Hight level languages abstractions (generics, iterators, templates, collections, classes...) should not impose any runtime overhead or performance penalties compare to code using low level abstractions like loops and pointers.

In other languages like **Python**, iterators are not a zero cost abstraction, requiering additional runtime checks to ensure the integrity of the iteration process.
## Ownership
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/1/18/ISO_C%2B%2B_Logo.svg/1280px-ISO_C%2B%2B_Logo.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>

Based on the **Resource Acquisition Is Initialization** design pattern or **RAII**. Resources like allocated memory, file handles and database connections should be tied to object lifetimes. When an object is created, it acquieres resources and when the object is destroyed, those resources are released.

## Algebraic Data Types (ADT)
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/1/1c/Haskell-Logo.svg/1280px-Haskell-Logo.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>

ADT are a type system concept commonly found in functional languages like **haskell**. They allow us to create composite types using **sum types** and **product types**

Sum types, also known as variant types, let us define a type that could be one of sevral variants. For example:
```rust,ignore
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32)
}
```
Product types are similar to structures in **C**, allowing us to bundle multiple types together
```rust,ignore
struct Person {
    name: String,
    age: u32, 
    height: f32
}
```
ADT are useful becasue they allow us to model complex data structures and enforce constraints on data at the type level which can help cath errors at compile time

## Pattern  Matching
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/1/1c/Haskell-Logo.svg/1280px-Haskell-Logo.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>
Also took from functional languages, pattern matching checks a value against different patterns and runs the code for the matching case. In Rust, it is commonly used with `match`.

```rust,ignore
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
}

let shape = Shape::Circle(3.0);

match shape {
    Shape::Circle(radius) => println!("Radius: {radius}"),
    Shape::Rectangle(width, height) => println!("Area: {}", width * height),
}
```

## Polymorphism
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/c/c3/Python-logo-notext.svg/1280px-Python-logo-notext.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>

Polymorphism is one of the pilars of object-oriented languages like python. Polymorphism  is the ability of an object or function to take multiple forms or behave differently based on the context in which it is used. Some languages implement polymorphism through inheritance but rust takes a different approach using traits and generics.

Traits define a set of functions and methods that types can implement, similar to interfaces in other languages

```rust,ignore
trait Shape {
  fn area(&self) -> f64;
}

struct Rectangle {
  width: f64,
  height: f64
}

impl Shape for Rectangle {
  fn area(&self) -> f64 {
    self.width * self.height
  }
}

struct Circle {
  radius: f64
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * self.radius.powi(2)
  }
}
```
Generics enable us to write code that is abstracted over types which leads to reusable and efficient code.
```rust,ignore
trait Shape {
  fn area(&self) -> f64;
}

// ...

fn print_area<T: Shape>(shape: &T) {
  println!("Area: {}", shape.area());
}

fn main() {
  let rectangle = Rectangle { width: 5.0, height: 3.0 };
  print_area(&rectangle);

  let circle = Circle { radius: 2.5 };
  print_area(&circle);
}
```
Traits system in Rust has several advantages over classical inheritance:
- Flexibility and Composition 
- Non-invasive and Extensive
- No of Fragile Base Classs Problem
- Static Dispatch and Performance
## Asynchronous Programming
<p align="center">
  <img src="https://thumb.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/1280px-Unofficial_JavaScript_logo_2.svg.png?utm_source=es.wikipedia.org&utm_campaign=index&utm_content=thumbnail" alt="C++" width="100">
</p>

Asynchronous Programming is a programming paradigm that allows tasks to run independently and concurrently without blocking the execution of the main program

```rust,ignore
async fn get_post_data(user_id: u32, post_id: u32) -> Result<(User, Post, Vec<Comment>), Error>{
  let client = reqwest::Client::new();

  let user: User = client
      .get(&format!(
          "https://jsonplaceholder.typicode.com/users/{user_id}",
      ))
      .send().await?.json().await?;

  let post: Post = client
      .get(&format!(
          "https://jsonplaceholder.typicode.com/posts/{post_id}",
      ))
      .send().await?.json().await?;

  let comments: Vec<Comment> = client
      .get(&format!(
          "https://jsonplaceholder.typicode.com/posts/{post_id}/comments",
      ))
      .send().await?.json().await?;
}
```

## Meta Programming
<p align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/3/39/Lambda_lc.svg?utm_source=en.wikipedia.org&utm_campaign=index&utm_content=original" alt="C++" width="100">
</p>

Meta Programming is a programming technique that allows a program to manipulate or generate code during compile time or run time. 

Macros are a language feature that allow developers to define custom syntax and perform code generation or code transformation. They enable meta programming by providing a way to write code that generates or modifies other code during compilation

```rust,ignore
macro_rules! map {
    ($key:ty, $val:ty) => {
        {
            let map: HashMap<$key, $val> = HashMap::new();
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}
```
Rust macros are hygienic, which is a feature Rust took from Scheme. Hygienic macros is a concept that ensures the variables and identifiers used within a macro do not accidentally crash with the variables from the surrounding code.

```rust,ignore
macro_rules! greet {
    ($name:expr) => {
        let mut name = $name;
        println!("Hello, {}!", name);
    };
}

fn main() {
    let name = "John";
    greet!(name);
    println!("{name}");
}
```
## Package Management
<p align="center">
  <img src="https://developerhowto.com/wp-content/uploads/2018/10/node-js-npm.png" alt="C++" width="600">
</p>

In 2009 Ryan Doll revolutionated JvaScript introducing **node.js**, a JS runtime that lets you run JavaScript outside a web browser. This led to an explosion of JS Backend applications. Alongside node.js, Ryan also created a node package manager or **npm** for short, giving developers a powerfull tool to manage dependencies and share code within the node.js ecosystem. Shortly afterwards [npmjs](https://www.npmjs.com/) was created serving as the central hub for hosting and discovering public packages.

Rust took this idea and ran with it by creating [cargo](https://github.com/rust-lang/cargo) and [crates.io](https://crates.io/). Cargo is Rust official build system and package manager. It handles many tasks, such as building code,dependency management, testing, compilation, managinr Rust versions etc
<p align="center">
  <img src="https://user-images.githubusercontent.com/41265192/44623017-cfe8d880-a8c4-11e8-97c2-60397f0519b0.png" alt="C++" width="300">
</p>

