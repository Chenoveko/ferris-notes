# 🔎 [regex](https://crates.io/crates/regex)
A **regular expression** (RegEx) is a search pattern for text

## 📗 **`find` Method**
`find` searches the text for the first match. It returns `Some(Match)` if it finds one, or `None` if there is no match. The match includes the matched text and its byte range in the original string.
```rust
# use regex::Regex;
# fn main() {
let re = Regex::new(r"ue").unwrap();
let text = "my movie queue";

match re.find(text) {
    Some(data) => println!("{data:?}"),
    None => println!("No matches found")
}
# }
```
## 📗 **`find_iter` Method**
`find_iter` searches the text and returns an iterator over all non-overlapping matches.
```rust
# use regex::Regex;
# fn main() {
let re = Regex::new(r"ue").unwrap();
let text = "my movie queue";

for data in re.find_iter(text) {
    println!("{} {} {}", data.start(), data.end(), data.as_str());
}
# }
```

## 📗 **Searching for Digits**
```rust
# use regex::Regex;
# fn main() {
let re = Regex::new(r"[0-9]").unwrap();
let text = "My ZIP code is 90210. I am very rich";

for data in re.find_iter(text) {
    println!("{:?}", data);
}
# }
```
## 📗 **Searching for Alphanumerics**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[A-Za-z0-9_]").unwrap();
let text = "My ZIP code is 90210. I am very rich";

for data in re.find_iter(text) {
    println!("{:?}", data);
}
# }
```
For non alphanumerics
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[^A-Za-z0-9_]").unwrap();
let text = "My ZIP code is 90210. I am very rich";

for data in re.find_iter(text) {
    println!("{:?}", data);
}
# }
```

## 📗 **Searching for Whitespaces**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[\t\n\r ]").unwrap();
let text = "Hello there!\nHow are you?";

for data in re.find_iter(text) {
    println!("{data:?}");
}
# }
```

## 📗 **Word Boundaries**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"(?-u)\bZIP\b").unwrap();
let text = "My ZIP code is 90210";

for data in re.find_iter(text) {
    println!("{}", data.as_str());
}
# }
```

## 📗 **Dot Metacharacter**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"c.t").unwrap();
let text = "cat cot cut";

for data in re.find_iter(text) {
    println!("{}", data.as_str());
}
# }
```

## 📗 **Square Brackets with Characters**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[aeiou]").unwrap();
let text = "regex";

for data in re.find_iter(text) {
    println!("{}", data.as_str());
}
# }
```

## 📗 **Number of Matches**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[aeiou]").unwrap();
let text = "regex";

let count = re.find_iter(text).count();
println!("Number of matches: {count}");
# }
```
## 📗 **OR Logic**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"cat|dog").unwrap();
let text = "I have a cat and a dog";

for data in re.find_iter(text) {
    println!("{}", data.as_str());
}
# }
```
## 📗 **Anchors**
```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"^Hello$").unwrap();

println!("{}", re.is_match("Hello"));     // true
println!("{}", re.is_match("Hello there")); // false
# }
```

## 📗 **Capture Groups**

```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
let text = "Date: 2026-10-21";

if let Some(caps) = re.captures(text) {
    println!("Full match: {}", &caps[0]);
    println!("Year: {}", &caps[1]);
    println!("Month: {}", &caps[2]);
    println!("Day: {}", &caps[3]);
}
# }
```
## 📗 **`replace_all` Method**

```rust
# use regex::Regex;
#
# fn main() {
let re = Regex::new(r"[0-9]+").unwrap();
let text = "Order 42 contains 3 items";

let result = re.replace_all(text, "#");
println!("{result}");
# }
```
