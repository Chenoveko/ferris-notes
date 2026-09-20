# ⏳ Lifetimes

[Visualize Ownership and Lifetimes with Rust Owl](https://github.com/cordx56/rustowl)

## 📗 **Rust's Borrowing Rules**

1. You can have multiple immutable references at the same time -> many `&T` ✅
2. You can have only one mutable reference at a time -> One `&mut T` ✅
3. You cannot have mutable and immutable references active at the same time -> `&T` + `&mut T` active ❌
4. A reference cannot outlive the data it points to -> reference outlives data ❌
5. References must always point to valid data -> Dangling References ❌

## 📘 **Concrete Lifetimes for Values and References**
The **lifetime** of a value refers to how long it is valid within the code. More technical, a value's **lifetime** is the time during which it exists at a particular memory address.

A **concrete lifetimes** means the actual period during which a specific value or reference is valid and can be used

Lifetimes are often connected to scopes, but they do not necessarily have to be connected (scope != lifetime)

The borrow checker is the part of the Rust compiler that validates that all borrows are valid

**Concrete lifetimes for values** refers to how long the owned value exist
```rust
# fn main() {
{
    let dog = String::from("Watson");
    println!("{dog}");
    // The concrete lifetime of 'dog' ends at the end of this scope
} // 'dog' is dropped here.
# }
```
**Concrete lifetimes for references** refers to how long the borrow of that value exist
```rust
# fn main() {
let mut text = String::from("hello");

let reference = &text; // The concrete lifetime starts here

// Suppose that is the last use of 'reference'
println!("Value of 'text' using the reference: {}", reference); 

// The concrete lifetime of the borrow ends here, even though the variable `reference` 
// is still inside the scope of `main`
// Thanks to NLL, Rust knows that the borrow has ended
# }
```

## 📘 **Non-Lexical Lifetimes (NLL)**
- Lexical lifetimes last until the end of the enclosing block.
- Non-lexical lifetimes end when a reference is no longer used.
- The borrow checker determines a reference's lifetime based on its
      last use rather than requiring it to remain valid until block end.
- NLL was introduced through [RFC 2094](https://rust-lang.github.io/rfcs/2094-nll.html)

**Before NLL**
```rust,ignore
fn main() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..]; // <-+ lifetime
    capitalize(slice);         //   | This is the last use of the reference 'slice'
    data.push('d'); // ERROR!  //   | Here was an error because of Rust borrowing rule 3
    data.push('e'); // ERROR!  //   |
    data.push('f'); // ERROR!  //   |
} // <------------------------------+
```
To solve the problem, developers have to do this changes
```rust
# fn capitalize(slice: &mut [char]) {
  #  for ch in slice {
   #     * ch = ch.to_ascii_uppercase();
    # }
# } 
fn main() {
    let mut data = vec!['a', 'b', 'c'];
    {
        let slice = &mut data[..]; // <-+ lifetime
        capitalize(slice);         //   |
    } // <------------------------------+
    data.push('d'); // OK
    data.push('e'); // OK
    data.push('f'); // OK
    println!("{:?}", data)
}
```
**After NLL**
```rust
# fn capitalize(slice: &mut [char]) {
  #  for ch in slice {
   #     * ch = ch.to_ascii_uppercase();
    # }
# } 
fn main() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..]; // <-+ lifetime                 
    capitalize(slice);         //   |
    // <----------------------------+
    data.push('d'); // OK  
    data.push('e'); // OK 
    data.push('f'); // OK  
    println!("{:?}", data)
} 
```
## 📘 **Invalid Lifetimes**
**Dangling References**
```rust,ignore
let cities: [String; 3] = [String::from("London"), 
    String::from("New York"), String::from("Barcelona")];

let fav_cities: &[String] = &cities[..2];
    
drop(cities);
    
println!("Fav cities: {:?}", fav_cities); // This is a dangling reference
```
Functions cannot return references to owned values or parameters, because creates **dangling references**
```rust,ignore
fn create() -> i32 {
    let age: i32 = 100;
    &age // Dangling reference
}

fn create_slice(items: Vec<i32>) -> &[i32] {
    &item // Dangling reference
}
```

## 📘 **References as Function Parameters**
To safely return a reference from a function, the referenced data must outlive the function call. In practice, the reference usually comes from data passed as an argument.
```rust
# fn main() {
// &[String] -> Slice to some collection type of Strings (vector or array) 
fn select_first_two_elements(item: &[String]) -> &[String] {
    &item[..2]
}

let cities: [String; 3] = [String::from("London"),
    String::from("New York"), String::from("Barcelona")];

println!("{:?}", select_first_two_elements(&cities));
# }
```

## 📙 **Generic Lifetimes**
Review of **Generics**:
- A generic is a placeholder for a future type
- Generics add flexibility by not hardcoding a exact type
- Code can use a variety of types in place of the generic    

Generic  Lifetimes vs Concrete Lifetimes
- A **concrete lifetimes** is the region of the code that a value exists in the program (the time it lives in its memory address)
- A **generic lifetime** is more abstract. It is a hypothetical lifetime, a non-specific lifetime, a future lifetime that can vary
- We can annotate generic lifetimes in cour code. This enables functions that are flexible enough to handle varying lifetimes -> **Lifetimes Annotations**

Lifetimes Annotations:
- A lifetimes annotation is a name or label for a lifetime
- Lifetimes annotations don't change the reference's lifetime. They don't affect the logic in any way
- A lifetime annotation is a piece of metadata that we provide to the borrow checker so that it can validate that references are valid  

```rust,ignore
fn select_first_two_elements<'a>(item: &'a [String]) -> &'a [String] {
    &item[..2]
}
```
```text
'a
│
├── item: &'a [String]
│   └── input reference lives for 'a
│
└── return: &'a [String]
    └── output reference also lives for 'a
    └── The returned slice cannot outlive `item`.

input reference ───────────────► output reference
 &'a [String]                    &'a [String]
```
## 📙 **Lifetimes Elision Rules**
Elision is the act of omitting something. Lifetime elision means omitting generic lifetime annotations in situations where the borrow checker can infer the lifetime relationship automatically

### 1ª Elision Rule
The compiler assigns a lifetime to each parameter that is a reference
```rust,ignore
fn my_awesome_function<'a, 'b>(first: &'a i32, second: &'b i32, third: i32) {};
```
### 2ª Elision Rule
If there is **one** reference parameter and the return value is a reference, the borrow checker will infer that their lifetimes are related
```rust,ignore
fn my_incredible_function<'a>(value: &'a i32) -> &'a i32 {
    value
};
```

### 3ª Elision Rule
In am method definition, if there are multiple reference parameters but one of them is `self`, the borrow checker will assume the lifetime of the instance is connected to the lifetime of the return value
```rust,ignore
struct DentisyAppointment {
    doctor: String
}

impl DentisyAppointment {
    fn book<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_time: &'c str) -> &'a str {
        println!("You are booked from {} to {} with doctor {}",check_in_time, check_out_time, self.doctor);
        &self.doctor
    }
}
```

## 📙 **`static` Lifetime**
The `static` lifetimes enable us to return a reference to data that is declared within a function, so the static lifetime is used with a reference that  we can guarantee will exist for the whole program.

For example, string literals has a `static` lifetime because the text is hardcoded in the final binary at compile time, so it lives in memory for the entire duration of the program
```rust,ignore
fn say_hello() -> &'static str {
    "Hello"
}
```

Constants are available and defined at compile time and similarly have a `static` lifetime, available for the duration of the whole program
```rust,ignore
const COUNT: i32 = 400;

fn value() -> &'static i32 {
    &COUNT
}
```


## 📕 **Multiple References Parameters**
When a function has multiple reference parameters, the lifetime elision rules may be insufficient to determine the lifetime of the returned reference.

The compiler cannot know which input reference will be returned, so we must explicitly describe the relationship between the input lifetimes and the output lifetime.
### One Shared Lifetime
In this example, both parameters share the same lifetime parameter, `'a`. 

`'a` represents the period during which both input references are simultaneously valid.
```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando: String = "Orlando".to_string();
    let san_francisco: String = "San Francisco".to_string();

    println!("{}", longest(&orlando, &san_francisco));
}
```
### Different Lifetimes
The two input references do not need to live for the same amount of time. The lifetime of the returned reference is limited by the shorter lifetime.
```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando: String = "Orlando".to_string();
    {
        let san_francisco: String = "San Francisco".to_string();
        println!("{}", longest(&orlando, &san_francisco));
    }
}
```
### Independent Lifetimes
This works because the function always returns `first`, whose lifetime is `'a`. The returned reference does not depend on the lifetime of `_second`.
```rust
// The underscore in _second indicates that the parameter is intentionally unused
fn longest<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
    first
}

fn main() {
    let orlando: String = "Orlando".to_string();
    let result = {
        let san_francisco: String = "San Francisco".to_string();
        longest(&orlando, &san_francisco)
    };
    println!("{}", result);
}
```


## 📕 **Lifetimes in Structs**
When a struct stores a reference, we must specify a lifetime parameter for that reference.
```rust
#[derive(Debug)]
struct TrainSystem<'a> {
    // 'name' reference must remain valid for at least as long as the 
    // TrainSystem instance
    name: &'a str
}

fn main() {
    let name = "NJ Transit".to_string();

    let nj_transit = TrainSystem { name: &name };

    println!("{:?}", nj_transit);
}
```
A struct can store references with different lifetimes. In that case, we use multiple lifetime parameters.
```rust
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str
}

fn main() {
    let from: String = "Portland".to_string();                  // <-+ lifetime
                                                                //   |
    let plan: &str = {                                          //   |
        let to: String = "Bangor".to_string();  // <-+ lifetime //   |
                                                //   |          //   |
        let travel_plan = TravelPlan {          //   |          //   |
            from: &from,                        //   |          //   |
            to: &to                             //   |          //   |
        };                                      //   |          //   |
                                                //   |          //   |
        travel_plan.from                        //   |          //   |
    };  // <-----------------------------------------+          //   |                                              
                                                                //   |
    println!("{}", plan);                                       //   |
} // <---------------------------------------------------------------+
```