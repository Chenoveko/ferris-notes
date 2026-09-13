# 🗺️ HashMap
- A `HashMap<K, V>` is a collection type that stores data as key-value pairs, similar to a dictionary in Python.
- Each key must be unique, but different keys can have the same value.
- `HashMap` are stored on the heap because their size is not known at compile time, and they can grow and shrink dynamically
- `HashMap` are implemented using a hashing algorithm, which allows for fast lookups and insertions of key-value pairs
- `HashMap` are not ordered, meaning that the order of the key-value pairs is not guaranteed to be the same as the order in which they were inserted
- `HashMap` implement the `Debug` **trait**, which allows for easy printing of the contents of a hashmap using the {:?} format specifier
- The `HashMap` type is not included in the **prelude**, so we must import it from `std::collections`

## 📗 **Creating a `HashMap`**
`HashMap::new()` creates an empty `HashMap`. Rust needs to know the type of its elements, either through a type annotation or the turbofish operator.
```rust
use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f32> = HashMap::new();

    menu.insert(String::from("Burger"), 5.99);
    menu.insert(String::from("Fries"), 2.99);
    menu.insert(String::from("Nuggets"), 2.99);

    println!("Menu: {:?}", menu);
}
```
We can specify the key and value types using the turbofish operator:
```rust
use std::collections::HashMap;

fn main() {
    let mut country_capitals = HashMap::<&str, &str>::new();

    country_capitals.insert("USA", "Washington D.C.");
    country_capitals.insert("Canada", "Ottawa");
    country_capitals.insert("France", "Paris");
    
    println!("Country Capitals: {:?}", country_capitals);
    println!("Country Capitals Length: {:?}", country_capitals.len());
}
```
We can use the `HashMap::from()` to create a `HashMap`containing initial values.
```rust
use std::collections::HashMap;

fn main() {
    let data: [(&str, i32); 3] = [
        ("Bobby", 7),
        ("Alice", 8),
        ("John", 9)
    ];

    let years_at_company: HashMap<&str, i32> = HashMap::from(data);
    
    println!("Years at company before remove: {:?}", years_at_company);
}
```

## 📗 **`remove` method**
The `remove` **method** deletes a key-value pair using its key

It returns an `Option<V>`:
- `Some(value)` if the key existed.
- `None` if the key was not found.
```rust
use std::collections::HashMap;

fn main() {
    let data: [(&str, i32); 3] = [
        ("Bobby", 7),
        ("Alice", 8),
        ("John", 9)
    ];

    let mut years_at_company: HashMap<&str, i32> = HashMap::from(data);
    
    println!("Years at company before remove: {:?}", years_at_company);

    match years_at_company.remove("Alice") {
        Some(years) => {
            println!("Years at company after remove: {:?}", years_at_company);
            println!("Alice spend {} at the company", years);
        }
        None => println!("Alice not found in the hashmap"),
    }
}
```

## 📗 **Ownership and Borrowing**
When a `HashMap` stores owned values such as `String`, **ownership** moves into the `HashMap`.
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<String, String> = HashMap::new();

    let latte: String = String::from("Latte");
    let whole_milk: String = String::from("Whole Milk");

    // Ownership moves from each variable into the hashmap
    coffee_pairings.insert(latte, whole_milk);

    println!("Coffee Pairings <String, String>: {:?}", coffee_pairings);

    // latte and whole_milk can no longer be used because their ownership was moved into the vector
}
```
A `HashMap` can also store references instead of owning the data.
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let capuccino: String = String::from("Capuccino");
    let soy_milk: &str = "Soy Milk";

    // &String -> &str automatically
    coffee_pairings.insert(&capuccino, soy_milk);

    println!("Coffee Pairings <String, String>: {:?}", coffee_pairings);

    // coffee_pairings HashMap does not own cappuccino
    println!("Capuccino: {}", capuccino);
}
```
## 📗 **Access a Value by Key Using Indexing**
We can access a value using its key and the indexing operator. Indexing returns a reference to the value. However, it **panics** at runtime if the key does not exist.
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let capuccino: String = String::from("Capuccino");
    let soy_milk: &str = "Soy Milk";

    // &String -> &str automatically
    coffee_pairings.insert(&capuccino, soy_milk);

    println!("Coffee Pairings <String, String>: {:?}", coffee_pairings);

    println!("Coffee Pairings Capuccino Value: {:?}", coffee_pairings["Capuccino"]);
}
```

## 📗 **Access a Value Using `get` Method**
The `get` **method** attempts to access a value using its key.

It returns an `Option<&V>`:
- `Some(&value)` if the key exists.
- `None` if the key does not exist.
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let capuccino: String = String::from("Capuccino");
    let soy_milk: &str = "Soy Milk";

    // &String -> &str automatically
    coffee_pairings.insert(&capuccino, soy_milk);

    println!("Coffee Pairings <String, String>: {:?}", coffee_pairings);

    match coffee_pairings.get("Capuccino") {
        Some(value) => println!("Capuccino found in the hashmap: {}", value),
        None => println!("Capuccino not found in the hashmap"),
    }

    println!("Coffee Pairings Capuccino Value other option: {:?}",
        coffee_pairings.get("Capuccino") // => Option<&&str>
        .copied() // => Option<&str>
        .unwrap_or("Not Found"));
}
```
Using `get` is safer than indexing because it does not **panic** when the key is missing.

## 📗 **Overwriting a Value with an Existing Key**
If the key already exists in the `HashMap`, the new value will overwrite the existing value
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    coffee_pairings.insert("Capuccino", "Soy Milk");

    println!("Coffee Pairings before overwrite: {:?}", coffee_pairings);

    coffee_pairings.insert("Capuccino", "Almond Milk");

    println!("Coffee Pairings after overwrite: {:?}", coffee_pairings);
}
```
The `insert` method returns an `Option` containing the old value if the key already existed.
```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    coffee_pairings.insert("Capuccino", "Soy Milk");

    println!("Coffee Pairings before overwrite: {:?}", coffee_pairings);

    match coffee_pairings.insert("Capuccino", "Almond Milk") {
        Some(value) => println!("Your old value in the hashmap: {}", value),
        None => println!("New entry in HashMap"),
    }

    println!("Coffee Pairings after overwrite: {:?}", coffee_pairings);
}
```
## 📗 **`entry` method**
The `entry` **method** accepts a key and returns an `Entry` enum.

The `Entry enum` has two **variants**:
- `Occupied`: the key already exists.
- `Vacant`: the key does not exist.

```rust
use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    coffee_pairings.insert("Capuccino", "Soy Milk");
    coffee_pairings.insert("Flat White", "Almond Milk");

    println!("Coffee Pairings before entry: {:?}", coffee_pairings);

    coffee_pairings.entry("Capuccino").or_insert("Whole Milk");
    coffee_pairings.entry("Latte").or_insert("Oat Milk");

    println!("Coffee Pairings after entry: {:?}", coffee_pairings);
}
```
`or_insert` inserts the value only if the key does not already exist.