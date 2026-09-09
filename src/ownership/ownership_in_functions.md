# Ownership in Functions

##  Functions without transfer of ownership
When a value whose type implements the `Copy` trait is passed to a function by value, Rust creates an implicit copy. The function receives its own copy, so ownership is not transferred from the original variable.

Receiving immutable parameters:
```rust,editable
fn print_my_value(value: i8) {
    println!("Your values is {}", value);
}

fn main() {
    let apples = 24;
    print_my_value(apples); 
}
```

Receiving mutable parameters:
```rust,editable
fn print_my_value(mut value: i8) {
    value += 1;
    println!("Your values is {}", value);
}

fn main() {
    let apples = 24;
    print_my_value(apples); 
    println!("{apples}")
}
```

## Functions with transfer of ownership
When a value whose type does not implement the `Copy` trait is passed to a function by value, ownership is moved to the function parameter. For example, `String` does not implement `Copy`, so passing a `String` to a function transfers its ownership.


Receiving immutable parameters:
```rust,editable
fn print_my_value(value: String) {
        println!("Your values is {}", value);
    }

fn main() {
    let oranges: String = String::from("Oranges");
    print_my_value(oranges); // Ownership moves from 'oranges' to the function parameter
    // println!("{}", oranges); ❌ Error: oranges has been moved

}
```

Receiving mutable parameters:
```rust,editable
fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("My meal: {}", meal);
}

fn main() {
    let burguer: String = String::from("Burguer");
    add_fries(burguer);
    // println!("{}", burger); // ❌ Error: burger was moved
}
```

## Functions returning ownership values
A function can transfer ownership by returning a value
```rust,editable
fn bake_cake() -> String {
    let cake: String = String::from("Chocolate Mousse");
    cake // Ownership moves from the function to the caller
}

fn main() {
    let cake = bake_cake();
    println!("I now have a {} cake", cake);
}
```
The caller decides whether the returned value can be mutated.
```rust,editable
fn bake_cake() -> String {
    let mut cake: String = String::from("Chocolate Mousse");
    cake // Ownership moves from the function to the caller
}

fn main() {
    let mut cake = bake_cake();
    cake.push_str(" with ice cream");
    println!("I now have a {} cake", cake);
}
```