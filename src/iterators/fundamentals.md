# Fundamentals
## 📗 **Manual Iteration**
To iterate means to perform repeteadly. Iteration is repeating the same operation on a sequence of items, one item at a time.

Rust provides several ways to iterate over a collection.

**Using `loop`**

The `loop` keyword continually executed a block until we force termination with the `break` keyword
```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let mut current_index = 0;
let final_index = numbers.len() - 1;

loop {
    if current_index > final_index {
        break;
    }

    println!("{}", numbers[current_index]);
    current_index += 1;
}
# }
```

**Using `while`**

A `while` loop executes its block while a condition is true.
```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let mut current_index = 0;
let final_index = numbers.len() - 1;

while current_index <= final_index {
    println!("{}", numbers[current_index]);
    current_index += 1;
}
# }
```

**Using `for`**

A `for` loop provides a simpler way to iterate over a collection. The `for` loops takes ownserhip
```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

for number in numbers {
    println!("{}", number);
}
# }
```
The `for` loops takes ownserhip so if we want to keep using the vector, iterate over a reference.
```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

for number in &numbers {
    println!("{}", number);
}
# }
```

## 📘 **Trait Iterator**
Official documentation: [Trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

The `Iterator` trait represents an object that can produce elements one at a time.
## 📘 **Trait IntoIterator**
Official documentation: [Trait IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)

The `IntoIterator` trait defines how a value can be converted into an iterator. The main difference is:

- `Iterator` represents the iterator itself.
- `IntoIterator` represents something that can be converted into an iterator.

Many types implement IntoIterator, including: vectors, arrays, hashmaps, hashsets, ranges, Option enum, Result enum ...

Fro example with vectors, the type implements the `IntoIterator trait`, which means that a vector has the `into_iter` method that will return a type that implements the `Iterator trait` 
```rust
# fn main() {
let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Move of ownership from 'my_vec' to 'my_iterator'
let my_iterator = my_vec.into_iter();

// The iterator implements the Debug trait
println!("{:?}", my_iterator);
# }
```
## 📘 **Trait FromIterator**
Official documentation: [Trait FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)

The `FromIterator` trait defines how an iterator can be converted into a type
```rust
# use std::ops::Range;
# fn main() {
let numbers: Range<i32> = 1..11; 

let result: Vec<i32> = Vec::from_iter(numbers);

println!("{:?}", result);
# }
```

## 📘 **Exhausting the Iterator**
Iterators in Rust are lazy, which means that they do not automatically start iteration when they are created. The iterator remains unactivated  until another method is called or an operation occurs that begins the actual iteration process. The process of completing the iteration is called **using**,**consuming**, or **exhausting** the iterator. The iterator can only be used once before it is exhausted, so iteration cannot occur again after iterator has been exhausted. Exhausted means that the iterator has yielded all of its elements advanced to its end, and there are no more items to be extracted from it.

```rust
# fn main() {
let my_vec = vec![1, 2, 3];

let mut my_iterator = my_vec.into_iter();

println!("{:?}", my_iterator.next());
println!("{:?}", my_iterator.next());
println!("{:?}", my_iterator.next());
println!("{:?}", my_iterator.next()); 
println!("{:?}", my_iterator.next());
# }
```
## 📘 **Create an Iterator**
### **`into_iter` Method**
A `for` loop automatically uses the `IntoIterator` trait. 
```rust
# fn main() {
let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let mut my_iterator = my_vec.into_iter();

// for loop takes ownership of the iterator
for number in my_iterator {
    println!("{}", number);
}
# }
```
In the followin case that we saw before, Rust creates the iterator automatically
With `for` loop:
```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

for number in numbers {
    println!("{}", number);
}

// We can not acces numbers because the for loop takes ownserhip
# }
```
### **`iter` Method**
The `iter` method will create an iterator that yields immutable references to te collection's elements.
```rust
# fn main() {
let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let my_iterator = my_vec.iter();

for number in my_iterator {
    println!("{}", number);
}

println!("{:?}", my_vec);
# }
```
Is the same as this
```rust
# fn main() {
let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

for number in &my_vec {
    println!("{}", number);
}

println!("{:?}", my_vec);
# }
```

### **`iter_mut` Method**
The `iter` method will create an iterator that yields mutable references to the collection elements.
```rust
# fn main() {
let mut flavors = [
    String::from("Chocolate"),
    String::from("Vanilla"),
    String::from("Banana"),
];

let my_iterator = flavors.iter_mut();

for flavor in my_iterator {
    flavor.push_str(" Ice Cream");
}

println!("{:?}", flavors);
# }
```
Is the same as this
```rust
# fn main() {
let mut flavors = [
    String::from("Chocolate"),
    String::from("Vanilla"),
    String::from("Banana"),
];

for flavor in &mut flavors {
    flavor.push_str(" Ice Cream");
}

println!("{:?}", flavors);
# }
```
To derefrence we have to be more explicit in Rust using iterators
```rust
# fn main() {
let mut flavors = [
    String::from("Chocolate"),
    String::from("Vanilla"),
    String::from("Banana"),
];

let my_iterator = flavors.iter_mut();

for flavor in my_iterator {
    *flavor = flavor.to_lowercase();
}

println!("{:?}", flavors);
# }
```
Same with the shortcut
```rust
# fn main() {
let mut flavors = [
    String::from("Chocolate"),
    String::from("Vanilla"),
    String::from("Banana"),
];

for flavor in &mut flavors {
    *flavor = flavor.to_lowercase();
}

println!("{:?}", flavors);
# }
```

### Summary
Thre are three syntax options to create an iterator:
- **Taking Ownserhip** 
    - `for value in collection`
    - `for value in collection.into_iter()` 
- **Immutable References**
    - `for value in &collection`
    - `for value in collection.iter()` 
- **Mutable References**
    - `for value in &mut collection`
    - `for value in collection.iter_mut()` 

## 📘 **`for_each` Method**
The `for_each` method applies a consistent operation to every iterator element. It accepts a closure that determines what should happen to each element. The `for_each` method consumes the iterator.
```rust
# fn main() {
let text = "Sally sells sea shells by the sea shore";

let words = text.split_whitespace(); 

words.for_each(|word| { println!("{word}"); });

// println!("{:?}", words); ❌
# }
```

## 📘 **`HashMap` Iteration**
The iteration orden in `HashMap` is not guaranteed.

**Taking Ownership**
```rust
# use std::collections::HashMap;
# fn main() {
let mut to_dos = HashMap::new();
to_dos.insert("Pick up groceries", false);
to_dos.insert("Study Rust", true);
to_dos.insert("Sleep", false);
to_dos.insert("Claen the house", false);

for (key, value) in to_dos {
    println!("Tasks: {} | Compelte: {}", key, value);
}
# }
```

**Immutable References**
```rust
# use std::collections::HashMap;
# fn main() {
let mut to_dos = HashMap::new();
to_dos.insert("Pick up groceries", false);
to_dos.insert("Study Rust", true);
to_dos.insert("Sleep", false);
to_dos.insert("Claen the house", false);

for (key, value) in &to_dos {
    println!("Tasks: {} | Compelte: {}", key, value);
}

println!("{:?}", to_dos);
# }
```

**Mutable References**
```rust
# use std::collections::HashMap;
# fn main() {
let mut to_dos = HashMap::new();
to_dos.insert("Pick up groceries", false);
to_dos.insert("Study Rust", true);
to_dos.insert("Sleep", false);
to_dos.insert("Claen the house", false);

for (key, value) in &mut to_dos {
    *value = false;
    println!("Tasks: {} | Compelte: {}", key, value);
}

println!("{:?}", to_dos);
# }
```

## 📙 **`String` Iteration**
A `String` stores text as a sequence of UTF-8 encoded bytes. A character does not always occupy a single byte in memory. For example, ASCII characters usually occupy one byte, while emojis may occupy several bytes.

Because of this, Rust allows us to iterate over a `String` in two different ways: over its raw bytes or over its Unicode characters.

The `chars()` method returns an iterator over the string’s Unicode scalar values, while the `bytes()` method returns an iterator over its raw UTF-8 bytes.

`String` does not provide inherent `iter()` or `iter_mut()` methods. However, it can be consumed with `into_iter()`

**Iterating over bytes**
```rust
# fn main() {
let seafood = "Oyster🦪";

for byte in seafood.bytes() {
    println!("{}", byte);
}

println!("{}", seafood);
# }
```
**Iterating over characters**
```rust
# fn main() {
let seafood = "Oyster🦪";

for character in seafood.chars() {
    println!("{}", character);
}

println!("{}", seafood);
# }
```
**Counting bytes and characters**
```rust
# fn main() {
let seafood = "Oyster🦪";

let byte_count = seafood.bytes().count(); 
let character_count = seafood.chars().count(); 

println!("Bytes: {byte_count}"); 
println!("Characters: {character_count}");
# }
```
**Iterator length**

The `len()` method returns the number of remaining items in an iterator
```rust
# fn main() {
let seafood = "Oyster🦪";

let mut iterator = seafood.bytes();

println!("Remaining bytes: {}", iterator.len()); 
iterator.next(); 
println!("Remaining bytes: {}", iterator.len());
# }
```
Unlike `count()`, the `len()` method does not consume the iterator. It only checks how many items remain.
