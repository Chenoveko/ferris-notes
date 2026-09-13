# **Scalar Data Types**

## 📗 **Integers** (Signed and Unsigned)
| Length | Signed | Range | Unsigned | Range |
|--------|--------|-------|----------|-------|
| 8-bit | `i8` | -128 to 127 | `u8` | 0 to 255 |
| 16-bit | `i16` | -32,768 to 32,767 | `u16` | 0 to 65,535 |
| 32-bit | `i32` | -2³¹ to 2³¹ − 1 | `u32` | 0 to 2³² − 1 |
| 64-bit | `i64` | -2⁶³ to 2⁶³ − 1 | `u64` | 0 to 2⁶⁴ − 1 |
| 128-bit | `i128` | -2¹²⁷ to 2¹²⁷ − 1 | `u128` | 0 to 2¹²⁸ − 1 |
| Architecture-dependent | `isize` | -2ⁿ⁻¹ to 2ⁿ⁻¹ − 1 | `usize` | 0 to 2ⁿ − 1 |
```rust,noplayground
# fn main() {
let eight_bit_signed: i8 = -128; 
let sixteen_bit_signed: i16 = 32_500; // Using _ as visual separator for numbers
let days: usize = 55;
let years: isize = -15_000;
# }
```
## 📗 **String Literals**
Values written directly in the source code and known at compile time.
```rust
# fn main() {
println!("Hello world"); // String literal 
println!("Dear Emily,\nHow have you been?"); // \n -> New line
println!("\tOnce upon a time"); // \t -> Tabulation
println!("Juliet said \"I love you Romeo\""); // \" -> Double quote
let filepath = "C:\\My Documents\\new\\videos"; // \\ -> Backslash
println!("{filepath}");
// Raw string -> Escape sequences are not processed
// Backslashes can be written directly without escaping them
let raw_filepath = r"C:\My Documents\new\videos";
println!("{raw_filepath}");
# }
```

## 📗 **Methods**
A method is a function that lives on a value. It's an action we can ask the value to execut
```rust
# fn main() {
let value: i32 = -15;
println!("ABS Value {}", value.abs());
println!("Pow 2 Value {}", value.pow(2));
let empty_space = "                         my content                 ";
println!("{}", empty_space.trim());
# }
```

## 📗 **Floating-Point**
| Length | Type | Approximate range | Precision |
|--------|------|-------------------|-----------|
| 32-bit | `f32` | ±1.18 × 10⁻³⁸ to ±3.40 × 10³⁸ | ~6–9 decimal digits |
| 64-bit | `f64` | ±2.23 × 10⁻³⁰⁸ to ±1.80 × 10³⁰⁸ | ~15–17 decimal digits |

Formatting controls how a value is displayed:
```rust
# fn main() {
let pi: f64 = 3.1415932312313131;
println!("The current value of pi is {}", pi);
println!("The current value of pi floored is {}", pi.floor());
println!("The current value of pi ceiling is {}", pi.ceil());
println!("The current value of pi rounded is {}", pi.round());
println!("The current value of pi formatted is {:.3}", pi);
# }
```

## 📘 **Casting**
Casting is the process of converting a value from one type to another using the **as** keyword
- Value must fit within the constraints of the new assigned type
- Casting to a smaller or incompatible numeric type may lose information
```rust
# fn main() {
let miles_away: i32 = 50;
let miles_away_i8 = miles_away as i8;
let miles_away_u8 = miles_away as u8;
let miles_away_f32 = miles_away as f32;
println!("Miles Away as i32 {}", miles_away);
println!("Miles Away as i8 {}", miles_away_i8);
println!("Miles Away as u8 {}", miles_away_u8);
println!("Miles Away as f32 {:.2}", miles_away_f32);
# }
```

## 📗 **Numeric Operations**
Basic math operators
```rust
# fn main() {
let addition = 5 + 4;
let subtraction = 10 - 6;
let multiplication = 3 * 4;
let floor_division = 5 / 3; // Floor division -> integer divide by integer
let float_division = 5.0 / 3.0; // Decimal division
let modulo = 7 % 2; // remainder operator
println!(
    "Addition: {}, Subtraction: {}, Multiplication: {},
    Floor Division: {}, Float Division: {:.2}, Modulo: {}",
    addition, subtraction, multiplication, floor_division, float_division, modulo
);
# }
```
Augmented Assignment Operators
```rust
# fn main() {
let mut year = 2026;
year += 1; // Equivalent to: year = year + 1;
println!("Next year: {}", year); // 2027
year -= 1; // Equivalent to: year = year - 1;
println!("Current year: {}", year); // 2026
year *= 2;
println!("Future year: {}", year); // 4052
year /= 2;
println!("Actual year: {}", year); // 2026
// Also x %= 5;
# }
```

## 📗 **Booleans**
| Operator | Description | Example |
|----------|-------------|---------|
| `!` | NOT / Inversion | `!true` |
| `&&` | AND | `true && false` |
| `\|\|` | OR | `true \|\| false` |
| `==` | Equality | `"Coke" == "Pepsi"` |
| `!=` | Inequality | `"Coke" != "Pepsi"` |
| `<` | Less than | `age < 35` |
| `>` | Greater than | `age > 18` |
| `<=` | Less than or equal to | `age <= 35` |
| `>=` | Greater than or equal to | `age >= 18` |
```rust
# fn main() {
let is_handsome = true;
let is_silly = false;
println!("Handsome: {}, Silly: {}", is_handsome, is_silly);
let age: i32 = 21;
let is_young = age < 35;
println!("Young: {}", is_young);
println!("Age positive: {}, Age negative: {}", age.is_positive(), age.is_negative());
let is_true = true;
println!("Boolean inversion: {}", !is_true);
println!("Equality operator: {}", "Coke" == "Pepsi");
println!("Inequality operator: {}", "Coke" != "Pepsi");
println!("AND operator: {}", true && false);
println!("OR operator: {}", true || false);
# }
```

## 📗 **Characters**
- Represents a single unicode character
- unicode is a computing standard for the representation of text for most of the world's writing system
- Use single quotes
```rust
# fn main() {
let first_initial: char = 'C';
let crab: char = '🦀';
println!("First Initial: {} | Crab Emoji: {}", first_initial, crab);
println!("Character methods");
println!("{}, {}", first_initial.is_alphabetic(), crab.is_alphabetic());
println!("{}, {}", first_initial.is_uppercase(), crab.is_uppercase());
println!("{}, {}", first_initial.is_lowercase(), crab.is_lowercase());
# }
```