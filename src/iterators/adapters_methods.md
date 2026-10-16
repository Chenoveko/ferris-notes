# Iterator Adapter Methods
We can invoke methods on the iterator to transform it into another iterator. This methods are called **adapter methods** or just **adapters**. An **adapter method** is one that transform an iterator into another iterator based on some logic. The adapter are like a pipeline of transformations, starting with the original iterator and then arriving to the finañ desired iterator. The adapters consume the original iterator, the use it as the bases for creating a brand new iterator. 

## 📗 **`take` Method**
```rust
# use std::ops::Range;
# fn main() {
// Range already implements the Iterator trait
let numbers: Range<i32> = 1..51; 

for number in numbers.take(15) {
    println!("{}", number);
}
# }
```

## 📗 **`rev` Method**
```rust
# use std::ops::Range;
# fn main() {
let numbers: Range<i32> = 1..51; 

for number in numbers.rev().take(15) {
    println!("{}", number);
}
# }
```

## 📗 **`skip` Method**
```rust
# use std::ops::Range;
# fn main() {
let numbers: Range<i32> = 1..51; 

for number in numbers.skip(5).take(15) {
    println!("{}", number);
}
# }
```

## 📗 **`step_by` Method**

```rust
# use std::ops::Range;
# fn main() {
let numbers: Range<i32> = 1..51; 

for number in numbers.skip(5).take(15).step_by(2) {
    println!("{}", number);
}
# }
```
## 📗 **`lines` Method**
The `lines` method returns an iterator of the string's individual lines
```rust
# fn main() {
let text: String = String::from("
Chapter 1
Chapter 2
Chapter 3
Chapter 4
Chapter 5
");

for line in text.lines() {
    println!("{}", line);
}
# }
```
## 📘 **`map` Method**
The `map` method applies a closure to each item in an iterator to arrive at a new iterator of values

```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let my_iterator = numbers.iter();

let squares = my_iterator.map(|x: &i32| x.pow(2));

// 'squares' is lazy
println!("{:?}", squares);

// The `for` loop consumes the `squares` iterator.
for number in squares {
    println!("Squares: {}", number);
}
# }
```
We can concatenate several adapter methods 
```rust
# fn main() {
let names = [
    String::from("Jimmy"),
    String::from("Cleveland"),
    String::from("Boris")
];

let name_lengths = names
    .iter() // [Strings; 3] -> Iterator<&String>
    .map(|name| name.to_lowercase()) // Iterator<&String> -> Iterator<String>
    .map(|name| name.replace("i", "@@")) // Iterator<String> -> Iterator<String>
    .map(|name| name.len()) // Iterator<String> -> Iterator<usize>
    .collect::<Vec<usize>>(); // Iterator<usize> -> Vec<usize>

println!("{:?}", name_lengths);
# }
```

## 📘 **`filter` Method**
The `filter` method extracts a subset of values that satisfy a condition. Pass a closure that returns true for the elements to keep and false for the elements to exclude.
```rust
# fn main() {
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// The array is copied because i32 implements the Copy trait.
let evens: Vec<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .filter(|number| number % 2 == 0) // Iterator<i32> -> Iterator<i32>
    .collect(); // Iterator<i32> -> Vec<i32>

println!("Evens Numbers: {:?}", evens);
# }
```
## 📗 **`zip` Method**
The `zip` method combines two iterators together based on their elements having the same index positions. We esentially merge two iterators together into a rband new iterator and when we iterate over that brand new iterator, we're going to receive a tuple consisting of the respective elementsat each shared index position. 

```rust
# fn main() {
let first_names = ["Casey", "Robert", "Cargo", "Dan"];
let last_names = ["Johnson", "Smith", "Rustman"];

for (first_name, last_name) in first_names.iter().zip(last_names) {
    println!("{} {}", first_name, last_name);
}

let complete_names: Vec<String> = first_names
    .iter() // [&str; 3] -> Iterator<&str>
    .zip(last_names) // Iterator<&str> -> Iterator<(&str, &str) 
    .map(|(first_name, last_name)| format!("{} {}", first_name, last_name)) // Iterator<(&str, &str) -> Iterator<String>
    .collect(); // Iterator<String> -> Vec<String> 

println!("{:?}", complete_names);
# }
```

## 📙 **`copied` and `cloned` Methods**
The `copied` method converts an iterator from storing `&T` elements to `T` elements. It makes a copy of each `T` element. The `T` data type must implement the `Copy` trait.
```rust
# fn main() {
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// The array is copied because i32 implements the Copy trait.
let evens: Vec<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .filter(|number| number % 2 == 0) // Iterator<i32> -> Iterator<i32>
    .collect(); // Iterator<i32> -> Vec<i32>
println!("Evens Numbers: {:?}", evens);

let evens: Vec<&i32> = numbers
    .iter() // &[i32; 10] -> Iterator<&i32>
    .filter(|number: &&i32| **number % 2 == 0) // Iterator<&i32> -> Iterator<&i32>
    .collect(); // Iterator<&i32> -> Vec<&i32>
println!("Evens Numbers: {:?}", evens);

let evens: Vec<i32> = numbers
    .iter() // &[i32; 10] -> Iterator<&i32>
    .filter(|&&number| number % 2 == 0) // Iterator<&i32> -> Iterator<&i32>
    .copied() // Iterator<&i32> -> Iterator<i32>
    .collect(); // Iterator<&i32> -> Vec<&i32>
println!("Evens Numbers: {:?}", evens);
# }
```
The `cloned` method similarly converts an iterator from storing `&T` elements to `T` elements. It makes a clone of each `T` element. The `T` data type must implement the `Clone` trait.
```rust
# fn main() {
let teas: [String; 3] = [
    String::from("Hot Earl Grey"),
    String::from("Iced Green"),
    String::from("Hot Matcha"),
];

let more_teas: Vec<String> = teas
    .iter() // &[String; 3] -> Iterator<&String>
    .cloned() // Iterator<&String> -> Iterator<String>
    .collect(); // Iterator<String> -> Vec<String>

println!("{:?}", more_teas);
# }
```
## 📙 **`filter_map` Method**
The `filter_map` method both filters and transform a subset of elements from an iterator.
```rust
# fn main() {
let stocks: [String; 6] = [
    String::from("nvda"),
    String::from(""),
    String::from("aapl"),
    String::from("msft"),
    String::from(""),
    String::from("goog")
];

let capitalized_stocks: Vec<String> = stocks
    .iter() // &[String; 6] -> Iterator<&String>
    .filter(|stock| !stock.is_empty()) // Iterator<&String> -> Iterator<&&String>
    .map(|stock| stock.to_uppercase())
    .collect(); // Iterator<String> -> Vec<String>

println!("{:?}", capitalized_stocks);
# }
```
With `filter_map` we simplify the code 
```rust
# fn main() {
let stocks: [String; 6] = [
    String::from("nvda"),
    String::from(""),
    String::from("aapl"),
    String::from("msft"),
    String::from(""),
    String::from("goog")
];

let capitalized_stocks: Vec<String> = stocks
    .iter() // &[String; 6] -> Iterator<&String>
    .filter_map(|stock| {
        if stock.is_empty() {
            None
        } else {
            Some(stock.to_uppercase())
        }
    }) // Iterator<&String> -> Iterator<String>
    .collect(); // Iterator<String> -> Vec<String>

println!("{:?}", capitalized_stocks);
# }
```
## 📙 **`flatten` Method**
The `flatten` method return an iterator that flattens nested data structures
```rust
# fn main() {
let spreadsheet = vec![
    [1, 2, 3, 4],
    [10, 20, 30, 40],
    [100, 200, 300, 400],
];

let values: Vec<usize> = spreadsheet
    .into_iter() 
    .flatten()
    .collect(); 

println!("{:?}", values);
# }
```
## 📙 **`flat_map` Method**
The `flat_map` method combines `flatten` and `map` methods
```rust
# fn main() {
let attendees = [
    "Bon, Mary, Kevin",
    "Mike, Freddie",
    "John, Liam",
];

let attendees : Vec<&str> = attendees
    .iter() 
    .map(|group| group.split(", "))
    .flatten()
    .collect(); 

println!("{:?}", attendees);
# }
```

```rust
# fn main() {
let attendees = [
    "Bon, Mary, Kevin",
    "Mike, Freddie",
    "John, Liam",
];

let attendees : Vec<&str> = attendees
    .iter() 
    .flat_map(|group| group.split(", "))
    .collect(); 

println!("{:?}", attendees);
# }
```

## 📙 **`enumerate` Method**
The `enumerate` method transforms an iterator so that it yields the index position along with the current element

```rust
# fn main() {
let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers",
    "John", "Dan"];

let winners: Vec<&str> = applicants 
    .into_iter() // Vec<&str> -> Iterator<&str>
    .enumerate() // Iterator<&str> -> Iterator<(usize,&str)>
    .filter(|(index, _)| index % 3 == 0) // Iterator<(usize,&str)> -> Iterator<(usize,&str)>
    .map(|(_, applicant)| applicant) // Iterator<(usize,&str) -> Iterator<&str>
    .collect(); // Iterator<&str> -> Vec<&str>

println!("{:?}", winners);
# }
```
