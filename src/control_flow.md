# Control Flow

## if / else if /else
### As a Statement
```rust,editable
fn main() {
    let season: &str = "winter";
    if season == "summer" {
        println!("School is out!")
    } else if season == "winter" {
        println!("Winter is coming!")
    } else if season == "fall" {
        println!("Leaves falling!")
    } else if season == "spring" {
        println!("Lots of rain!")
    } else {
        println!("Unknown season")
    }
}
```

### As an Expression
Rust does not have a ternary operator like C or JavaScript 
```text
condition ? value_if_true : value_if_false
```
In Rust, an if expression provides the same functionality:
```rust,editable
fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {}", result);
}

fn main() {
    even_or_odd(17);
}
```
## match
- A pattern or arm is one possible option to compare the match value against
- match must cover every possible scenario
- Every match arm must return a value of the same type
- Match arms are checked from top to bottom in order

### As a Statement
```rust,editable
fn main() {
    let evaluation = true;
    match evaluation {
        true => println!("The evaluation is true"),
        false => println!("The evaluation is false")
    }
}
```

```rust,editable
fn main() {
    let number = 8;
    match number {
        2 | 4 | 6 | 8 => println!("{} is even", number),
        1 | 3 | 5 => println!("{} is odd", number),
        _ => println!("Unknown")
    }
}
```

```rust,editable
fn main() {
    let number = 8;
    match number {
        // x captures the value of number
        x if x % 2 == 0 => println!("{} is even", x),
        x if x % 2 != 0 => println!("{} is odd", x),
        _ => unreachable!(), // Macro for an arm that should never be reached
    }
}
```

### As an Expression
```rust,editable
fn main() {
    let evaluation = true;
    let value: i32 = match evaluation {
        true => 20,
        false => 40,
    };
    println!("The value is {}", value)
}
```

```rust,editable
fn main() {
    let evaluation = true;
    let value: i32 = match evaluation {
        true => 20,
        false => 40,
    };
    println!("The value is {}", value)
}
```

## Refactor if / else if / else Statement with match Statement
Refactor means to restructure or improve existing code without altering its design
```rust,editable
fn main() {
    let season: &str = "winter";

    if season == "summer" {
        println!("School is out!")
    } else if season == "winter" {
        println!("Winter is coming!")
    } else if season == "fall" {
        println!("Leaves falling!")
    } else if season == "spring" {
        println!("Lots of rain!")
    } else {
        println!("Unknown season");
    }

    match season {
        "summer" => println!("School is out!"),
        "winter" => println!("Winter is coming!"),
        "fall" => println!("Leaves falling!"),
        "spring" => println!("Lots of rain!"),
        _ => println!("Unknown season"), // cath-all pattern
    }
}
```

## Loops

### loop, break and continue
```rust,editable
fn main() {
    let mut seconds: i8 = 21;
    println!("Loop iteration!");
    loop {
        if seconds <= 0 {
            println!("Blastoff!!");
            break;
        }
        if seconds % 2 == 0 {
            println!("{} seconds (even number), skipping 3 seconds...", seconds);
            seconds -= 3;
            continue;
        }
        println!("{} seconds to blastoff...", seconds);
        seconds -= 1;
    }
}
```

### while loop
```rust,editable
fn main() {
    let mut seconds: i8 = 21;
    println!("Loop iteration!");
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{} seconds (even number), skipping 3 seconds...", seconds);
            seconds -= 3;
            continue;
        }
        println!("{} seconds to blastoff...", seconds);
        seconds -= 1;
    }
    println!("Blastoff!!");
}
```

## Recursion
Recursion is when a function calls itself. The base case is the condition that stops the recursion
```rust,editable
fn countdown(seconds: i8) {
        if seconds == 0 {
            println!("Blastoff!!") // Base case
        } else {
            println!("{} seconds to blastoff...", seconds);
            countdown(seconds - 1);
        } 
    }

fn main() {
    countdown(5);
}
```
