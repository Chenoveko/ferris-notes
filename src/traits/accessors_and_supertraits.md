# **Accessors Traits & Supertraits**

## 📗 **Getters in `traits`**
A **getter** methods is a method that retrieves a piece of data. It "gets" a piece of state.
```rust
trait UserAccessors {
    fn name(&self) -> &str;
    fn age(&self) -> u32;
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl UserAccessors for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let user = User::new(String::from("Alice"), 25);

    println!("Name: {}", user.name());
    println!("Age: {}", user.age());
}
```

## 📗 **Setters in `traits`**
A **setter** methods is a method that writes a piece of data. It "sets" a piece of state.
```rust
trait UserAccessors {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> u32;

    // Setters
    fn set_name(&mut self, name: String);
    fn set_age(&mut self, age: u32);
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl UserAccessors for User {
    // Getters
    fn name(&self) -> &str {
        &self.name
    }
    fn age(&self) -> u32 {
        self.age
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let mut user = User::new(String::from("Alice"), 25);

     // Using getters
    println!("Name: {}", user.name());
    println!("Age: {}", user.age());

    // Using setters
    user.set_name(String::from("Bob"));
    user.set_age(30);

    println!("Updated name: {}", user.name());
    println!("Updated age: {}", user.age());
}
```

## 📘 **`traits` Inheritance with `supertrait`**
A **supertrait** is a trait from which another trait inherits functionality. The parent is called the **supertrait** and the child is called the **subtrait**
```rust
trait Animal {
    fn name(&self) -> &str;

    fn eat(&self) {
        println!("{} is eating", self.name());
    }
}

// Dog requires every implementing type to also implement Animal
// Animal is a supertrait and Dog is a subtrait
trait Dog: Animal {
    fn bark(&self);
}

struct Labrador {
    name: String,
}

impl Animal for Labrador {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Dog for Labrador {
    fn bark(&self) {
        println!("{} says: Woof!", self.name());
    }
}

fn main() {
    let dog = Labrador { name: String::from("Max") };

    dog.eat();
    dog.bark();
}
```