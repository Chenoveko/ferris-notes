# 🗺️ HashSet
- A `HashSet` is a collection type that stores unique values, similar to a set in Python.
- Each value can appear only once in the `HashSet`
- Internally, `HashSet` is implemented using a `HashMap`, where each value is stored as a key.
- `HashSet` does not guarantee any particular order for its values.
- A `HashSet` can grow and shrink dynamically at runtime.
- `HashSet` implements the `Debug` **trait** when its elements implement `Debug`. This allows us to print its contents using the {:?} format specifier.
- The `HashSet` type is not included in the prelude, so we must import it from `std::collections`.


## 📗 **Creating a `HashSet`**
`HashSet::new()` creates an empty `HashSet`. Rust needs to know the type of its elements, either through a type annotation or the turbofish operator.
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    println!("Concert Queue: {:?}", concert_queue);
    println!("Concert Queue Length: {:?}", concert_queue.len());

    concert_queue.insert("Alice"); // This will not add a duplicate value

    println!("Concert Queue after adding duplicate: {:?}", concert_queue);
}
```
We can specify the key and value types using the turbofish operator:
```rust
use std::collections::HashSet;

fn main() {
    let mut books = HashSet::<String>::new();

    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    println!("Books: {:?}", books);
}
```

## 📗 **`remove` method**


```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    println!("Concert Queue before remove: {:?}", concert_queue);
    
    let removing: bool = concert_queue.remove("Bob");

    println!("Concert Queue after remove: {:?}", concert_queue);
    println!("Was Bob removed from the concert queue? {}", removing);
}
```

## 📗 **`contains` method**

```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");
    
    let contains: bool = concert_queue.contains("Charlie");

    println!("Does the concert queue contain Charlie? {}", contains);
}
```


## 📗 **`get` method**
The `get` **method** attempts to access a value using its key.

It returns an `Option<&V>`:
- `Some(&value)` if the key exists.
- `None` if the key does not exist.

```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");
    
    match concert_queue.get("Alice") {
        Some(value) => println!("Alice found in the concert queue: {}", value),
        None => println!("Alice not found in the concert queue"),
    }
}
```
## 📘 **`HashSet` Operations**

### 📘 **`union` Method**
Returns an **iterator** that contains all the unique values from both sets
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Union of concert and movie queue: {:?}", concert_queue.union(&movie_queue));
}
```
### 📘 **`intersection` Method**
Returns an **iterator** that contains all the values that are in both sets
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Intersection of concert and movie queue: {:?}", concert_queue.intersection(&movie_queue));
}
```



### 📘 **`difference` Method**
Returns an **iterator** that contains all the values that are in the first set but not in the second set
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Difference of concert and movie queue: {:?}", concert_queue.difference(&movie_queue));
}
```

### 📘 **`symmetric_difference` Method**
Returns an **iterator** that contains all the values that are in either set but not in both sets
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Symmetric Difference of concert and movie queue: {:?}", concert_queue.symmetric_difference(&movie_queue));
}
```
### 📘 **`is_disjoint` Method**
Returns true if the two sets have no values in common, false otherwise
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Are concert and movie queue disjoint? {}", concert_queue.is_disjoint(&movie_queue));
}
```
### 📘 **`is_subset` Method**
Returns true if the first set is a subset of the second set, false otherwise
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Is concert queue a subset of movie queue? {}", concert_queue.is_subset(&movie_queue));
}
```
### 📘 **`is_superset` Method**
Returns `true` if the first set is a superset of the second set, `false` otherwise
```rust
use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");

    movie_queue.insert("Bob");
    movie_queue.insert("David");
    
    println!("Is concert queue a superset of movie queue? {}", concert_queue.is_superset(&movie_queue));
}
```