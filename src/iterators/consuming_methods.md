# Iterator Consuming  Methods
## 📗 **`sum` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let total: i32 = numbers.iter().sum();

println!("{}", total);
# }
```

```rust
# fn main() {
let numbers: Vec<f64> = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

let total: f64 = numbers.iter().sum();

println!("{}", total);
# }
```

```rust
# fn main() {
let numbers: Vec<f64> = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

let total: f64 = numbers.iter().filter(|number| !number.is_nan()).sum();

println!("{}", total);
# }
```
## 📗 **`product` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let total: i32 = numbers.iter().product();

println!("{}", total);
# }
```
## 📗 **`max` Method**

```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let max: Option<&i32> = numbers.iter().max();

println!("{:?}", max);
# }
```


```rust
# fn main() {
let numbers: Vec<f64> = vec![1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

let max: Option<f64> = numbers.iter().filter(|number| !number.is_nan()).copied().reduce(|accumulator, current| accumulator.max(current));

println!("{:?}", max);
# }
```

## 📗 **`min` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let min: Option<&i32> = numbers.iter().min();

println!("{:?}", min);
# }
```

## 📗 **`count` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let total: usize = numbers.iter().count();

println!("{}", total);
# }
```

## 📗 **`last` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let last: Option<&i32> = numbers.iter().last();

println!("{:?}", last);
# }
```

## 📗 **`nth` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let second: Option<&i32> = numbers.iter().nth(1);

println!("{:?}", second);
# }
```

## 📗 **`nth_back` Method**
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let second_back: Option<&i32> = numbers.iter().nth_back(1);

println!("{:?}", second_back);
# }
```
## 📗 **`position` Method**

```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let position: Option<usize> = numbers.iter().position(|number| *number == 5);

println!("{:?}", position);
# }
```

## 📗 **`sort` Method**
```rust
# fn main() {
let mut numbers: Vec<i32> = vec![9, 4, 3, 2, 15, 6, 99, 32, 45, 10];

numbers.sort();

println!("{:?}", numbers);
println!("{:?}", numbers.is_sorted());
# }
```
```rust
# fn main() {
let mut exercises: Vec<&str> = vec!["squat", "Bench", "Deadlift"];

exercises.sort();

println!("{:?}", exercises);
println!("{:?}", exercises.is_sorted());
# }
```
## 📗 **`sort_by_key` Method**
```rust
#[derive(Debug)]
struct GasStation {
    name: String,
    city: String,
    price_cents: u32,
}

fn main() {
    let mut stations = vec![
        GasStation {
            name: String::from("Shell"),
            city: String::from("Madrid"),
            price_cents: 178,
        },
        GasStation {
            name: String::from("Repsol"),
            city: String::from("Toledo"),
            price_cents: 165,
        },
        GasStation {
            name: String::from("BP"),
            city: String::from("Madrid"),
            price_cents: 172,
        },
    ];

    stations.sort_by_key(|station| station.price_cents);

    println!("{:?}", stations);
}
```
## 📘 **`collect` Method**
The `collect` method exhausts the iterator and gathers the resulting values in a new collection type.
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let squares: Vec<i32> =  numbers.iter().map(|x: &i32| x.pow(2)).collect();

println!("Numbers: {:?}", numbers);
println!("Squares: {:?}", squares);
# }
```
We can use the turbofish operator
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let squares: Vec<i32> =  numbers.iter().map(|x: &i32| x.pow(2)).collect::<Vec<i32>>();

println!("Numbers: {:?}", numbers);
println!("Squares: {:?}", squares);
# }
```
Sometimes Rust can infer the type of the return value
```rust
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let squares: Vec<_> =  numbers.iter().map(|x: &i32| x.pow(2)).collect();

println!("Numbers: {:?}", numbers);
println!("Squares: {:?}", squares);
# }
```
The `collect` method can create a variety of different data types and Rust just need the specific type annotation for the compiler safety checks.
```rust
# use std::collections::HashSet;
# fn main() {
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5];

let squares: HashSet<i32> =  numbers.iter().map(|x: &i32| x.pow(2)).collect();

println!("Numbers: {:?}", numbers);
println!("Squares: {:?}", squares);
# }
```

## 📘 **`find` and `rfind` Methods**
The `find` method locate the first element for which the closure returns `true`. So not all elements, but just the very first one proceeding from the start of the iterator. The `find` method is going to stop iteration inmediately when it gets a true, and then actually exhaust the remainder of the iterator to prevent any unnecessary traversal. It returns and `Option`, because there is the possibility that no element at all evaluates to true in the closure. If there is an element that evaluates to `true`, is going to we wrapped up in the `Some` variant, if not it returns the `None` variant. 
```rust
# fn main() {
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let first_even: Option<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .find(|number| number % 2 == 0); // Iterator<i32> -> Option<i32>

let first_odd: Option<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .find(|number| number % 2 != 0); // Iterator<i32> -> Option<i32>

let nothing: Option<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .find(|number| number % 2 > 100); // Iterator<i32> -> Option<i32>

println!("First Even: {:?}", first_even);
println!("First Odd: {:?}", first_odd);
println!("Nothing: {:?}", nothing);
# }
```
The `rfind` method has the same logic as `find` but from the end (reverse find). Give the last occurence of an element that satisfies the closure condition. 
```rust
# fn main() {
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let last_even: Option<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .rfind(|number| number % 2 == 0); // Iterator<i32> -> Option<i32>

let last_odd: Option<i32> = numbers
    .into_iter() // [i32; 10] -> Iterator<i32>
    .rfind(|number| number % 2 != 0); // Iterator<i32> -> Option<i32>

println!("Last Even: {:?}", last_even);
println!("Last Odd: {:?}", last_odd);
# }
```
## 📘 **`any` and `all` Methods**
The `any` method returns `true` any all of the elements in a given iterator satisfy the closure condition
```rust
# fn main() {
let even_numbers = [2, 4, 6];

let any_even: bool = even_numbers
    .into_iter() // [i32; 3] -> Iterator<i32>
    .any(|number| number % 2 == 0); // Iterator<i32> -> bool

let any_odd: bool = even_numbers
    .into_iter() // [i32; 3] -> Iterator<i32>
    .all(|number| number % 2 != 0); // Iterator<i32> -> bool

println!("Any Even? {}", any_even);
println!("Any Odd? {}", any_odd);
# }
```
The `all` method returns `true` if all of the elements in a given iterator satisfy the closure condition
```rust
# fn main() {
let even_numbers = [2, 4, 6];

let all_even: bool = even_numbers
    .into_iter() // [i32; 3] -> Iterator<i32>
    .all(|number| number % 2 == 0); // Iterator<i32> -> bool

let all_odd: bool = even_numbers
    .into_iter() // [i32; 3] -> Iterator<i32>
    .all(|number| number % 2 != 0); // Iterator<i32> -> bool

println!("Any Even? {}", all_even);
println!("Any Odd? {}", all_odd);
# }
```
## 📙 **`partition` Method**
The `partition` method is similar to `filter` in that it accepts a closure that must return a boolean, the difference is tha he `filter` method selects for the elements that satisfy that particular closure `partition`, the elements for which the closure returns `true`. The `partition` method  groups and returns the values for which the closure returns `true` and for which the closures returns `false`.

```rust
# fn main() {
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let (evens, odds): (Vec<i32> ,Vec<i32>) = numbers
    .into_iter() // Vec<i32> -> Iterator<i32>
    .partition(|number| number % 2 == 0); // Iterator<i32> -> (Vec<i32> ,Vec<i32>)

println!("{:?}", evens);
println!("{:?}", odds);
# }
```
## 📙 **`fold` Method**
The `fold` method exhaust an iterator to build up and produce a single value at the end of the iteration.
```rust,ignore
.fold (
    starting_value,
    | accumulator, current_element | {}
)
```
```rust
# fn main() {
let earnings: [i32; 3] = [4, 7, 10];

let sum: i32 = earnings
    .iter() // [i32, 3] -> Iterator<i32>
    .fold(0, |total, current| total + current); // Iterator<i32> -> i32
 

println!("{}", sum);
# }
```

```rust
use std::collections::HashMap;
struct SupportStaff {
    day: String,
    employee: String
}
fn main() {
    let week: [SupportStaff; 3] = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Brian"),
        }, 
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Cam"),
        }, 
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Walter"),
        }, 
    ];

    let my_hashmap: HashMap<String, String> = week
        .into_iter() // [SupportStaff; 3] -> Iterator<SupportStaff>
        .fold(HashMap::new(), |mut data, entry| {
            data.insert(entry.day, entry.employee);
            data
        }); // Iterator<SupportStaff> -> HashMap<String, String>

    println!("{:?}", my_hashmap);
}
```

## 📙 **`reduce` Method**
The `reduce` method is similar to `fold` but it supplies the iterator's first element as the starter value. The `reduce` method returns an `Option` enum to account for the possibility of an empty iterator.

```rust
# fn main() {
let earnings: [i32; 3] = [4, 7, 10];

let sum: Option<i32> = earnings
    .into_iter() // [i32, 3] -> Iterator<i32>
    .reduce(|total, current| total + current); // Iterator<i32> -> Option<i32>
 
println!("{:?}", sum);
# }
```

```rust
# fn main() {
let earnings: [i32; 0] = [];

let sum: Option<i32> = earnings
    .into_iter() // [i32, 3] -> Iterator<i32>
    .reduce(|total, current| total + current); // Iterator<i32> -> Option<i32>
 
println!("{:?}", sum);
# }
```
```rust
# fn main() {
let address_portions: [String; 3] = [
    String::from("123 Elm Street"),
    String::from("Suburbia"),
    String::from("New Jersey"),
];

println!("{}", address_portions.join(", "));

let address: Option<String> = address_portions
    .into_iter() // [String; 3] -> Iterator<String>
    .reduce(|mut accumulator,  portion| {
        accumulator.push_str(", ");
        accumulator.push_str(&portion);
        accumulator
    }); // Iterator<String> -> Option<String>
 
println!("{:?}", address);
# }
```