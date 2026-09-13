# **Compound Data Types**

## 📘 **Arrays**
Fixed-size collection of homogeneous data (data of the same type)
```rust
# fn main() {
let numbers: [i32; 4] = [1, 2, 3, 4];
let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
println!("Length of numbers: {}, Length of apples: {}", numbers.len(), apples.len());
let currency_rates: [f32; 0] = []; // Empty array
let mut seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
let [spring, summer, fall, winter] = seasons; // Array destructuring
let [first, _, _, last] = numbers; // Array destructuring ignoring elements
println!("First: {}, Last: {}", first, last);
// Reading and writing array elements
println!("American Seasons: {} {} {} {}", seasons[0], seasons[1], seasons[2], seasons[3]);
seasons[2] = "Autumn";
println!("British Seasons: {} {} {} {}", seasons[0], seasons[1], seasons[2], seasons[3]);
# }
```

## 📘 **Tuples**
Fixed-size collection that can contain values of different types
```rust
# fn main() {
let employee: (&str, i32, &str) = ("Molly", 32, "Marketing");
// Access tuple elements by index
println!("Name: {}, Age: {}, Department: {}", employee.0, employee.1, employee.2);
let (name, age, department) = employee; // Tuple destructuring
println!("Name: {}, Age: {}, Department: {}", name, age, department);
# }
```

## 📘 **Ranges**
A range is a sequence/interval of consecutive values. We have to improt using the **standard library**
- `use std::ops::Range;`
- `use std::ops::RangeInclusive;`
```rust
# use std::ops::Range;
# use std::ops::RangeInclusive;
# fn main() {
let week_days: Range<i32> = 1..7; // Upper value excluded
let week_days_inclusive: RangeInclusive<i32> = 1..=7; // Upper value included
let letters: Range<char> = 'b'..'f';
println!("Week Days: {:?}", week_days);
println!("Week Days Inclusive: {:?}", week_days_inclusive);
// Iterate over a range
for number in week_days {
    println!("{number}")
}
for letter in letters {
    println!("{letter}")
}
# }
```
