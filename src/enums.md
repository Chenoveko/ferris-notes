# 🔀 Enums
An `enum` is a type that represents a set of possible values. Each possible value is called a **variant**. We use **PascalCase** to define enums and it's variants

Rust has 3 kinds of **variants** in `enums`:
- **Unit Variant**: stores no associated data
- **Tuple Variant**: stores associated data by position
- **Struct Variant**: stores associated data in named fields

## 📗 **Unit Variant Enums**
```rust
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

fn main() {
    // Instance enum
    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", first_card);
    println!("{:?}", second_card);
}
```

## 📗 **Tuple Variant Enums**

```rust
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal(String, String)
}
fn main() {
    let visa: PaymentMethodType = PaymentMethodType::CreditCard(String::from("0034-4582"));
    let mastercard: PaymentMethodType = PaymentMethodType::DebitCard(String::from("1234-5678"));
    let paypal: PaymentMethodType = PaymentMethodType::Paypal(String::from("bob@gmail.com"), String::from("password"));
    println!("{:?} {:?} {:?}", visa, mastercard, paypal);
}
```

## 📗 **Struct Variant Enum**
```rust
#[derive(Debug)]
enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Paypal { username: String, password: String },
    Cash
}

fn main() {
    let paypal: PaymentMethod = PaymentMethod::Paypal{
        username: String::from("bob@gmail.com"), 
        password: String::from("1234")
    };
    println!("{:?}", paypal);
}
```
## 📗 **Matching with Enums**
```rust
enum OperatingSytem {
    Windows,
    MacOS,
    Linux
}

fn years_since_release(os: OperatingSytem) -> u32 {
    match os {
        OperatingSytem::Windows => 39,
        OperatingSytem::MacOS => 23,
        OperatingSytem::Linux => 34
    }
}

fn main() {
    let my_computer = OperatingSytem::MacOS;
    let age: u32 = years_since_release(my_computer);
    println!("My computer's os is {} years old", age);
}
```

```rust
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => println!("Running the laundry with cold temperature"),
        LaundryCycle::Hot { temperature } => println!("Running the laundry with a temperature of {} degrees", temperature),
        LaundryCycle::Delicate(fabric_type) => println!("Running the laundry with a delicate cyle for {}", fabric_type)
    }
}

fn main() {
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
}
```
## 📗 **Nesting Enums in Enums**
```rust
#[derive(Debug)]
enum Beans {
    Pinto,
    Black
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

#[derive(Debug)]
enum RestaurtantItem {
    Burrito { meat: Meat, beans: Beans},
    Bowl(Meat),
    VeganPlate
}

fn main() {
    let lunch: RestaurtantItem = RestaurtantItem::Burrito{ meat: Meat::Steak, beans: Beans::Pinto };
    let dinner: RestaurtantItem = RestaurtantItem::Bowl(Meat::Chicken);
    println!("Lunch: {:?}", lunch);
    println!("Dinner: {:?}", dinner);
}
```

## 📘 **Adding Behavior to the Enum**
```rust
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => println!("Running the laundry with cold temperature"),
            LaundryCycle::Hot { temperature } => println!("Running the laundry with a temperature of {} degrees", temperature),
            LaundryCycle::Delicate(fabric_type) => println!("Running the laundry with a delicate cyle for {}", fabric_type)
        }
    }
}

fn main() {
    let my_laundry_cycle: LaundryCycle = LaundryCycle::Delicate(String::from("Silk"));
    my_laundry_cycle.wash_laundry();
}
```

```rust
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered
}

impl OnlineOrderStatus{
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed  => println!("Your item is being prepped for shipment"),
            OnlineOrderStatus::Delivered => println!("Your item has been delivered"),
            other_status => println!("Your item is {:?}", other_status)
        }
    }   
}

fn main() {
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();
}
```

```rust
enum Milk {
    LowFat(u32),
    Whole
}

impl Milk{
    fn drink(&self) {  
        match self {
            Milk::LowFat(2) => println!("Delicious, 2% milk is my favourite!"),
            Milk::LowFat(percent) => println!("You've got the lowfat {} percent version!", percent),
                Milk::Whole => println!("Whole milk 🐂")
        }
    }   
}

fn main() {
    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();
}
```
## 📘 **`if let` Construct**
- The `if let` construct provides a concise way to match a specific enum variant without using a full match expression.
- The code inside the `if let` block is executed only if the value matches the specified enum variant.
- If the `enum` variant contains associated data, `if let` can extract that data into variables that are available inside the block.
- Declare the hardcoded `enum` **variant** on the left-hand side of `=`. Declare the dynamic value on the right-hand side
- An optional `else` block can be used to handle all cases where the pattern does not match.
- Use `if let` when you are interested in one specific pattern and want to ignore or handle all other cases with else.
```rust
enum GoatMilk {
    LowFat(u32),
    Whole,
    NonDairy { kind: String }
}

fn main() {
    let milk = GoatMilk::LowFat(2);
    let my_beverage: GoatMilk = GoatMilk::Whole;

    // if match variant -> do something

    if let GoatMilk::Whole = my_beverage {
        println!("You have whole goat milk 🐐")
    }

    if let GoatMilk::LowFat(percent) = milk {
        // if variant match, we acces the value of the variable -> 'percent' only exist in this block
        println!("Low fat: {}%", percent); 
    }

    let my_beverage: GoatMilk = GoatMilk::NonDairy{
        kind: String::from("Oat")
    };

    if let GoatMilk::NonDairy { kind } = my_beverage {
        // if variant match, we acces the value of the variable -> 'kind' only exist in this block
        println!("Your beverage is {} milk", kind)
    } else {
        // if variant not match
        println!("Other variant")
    }
}
```
**Key idea**: `if let` is essentially syntactic sugar for a `match` when you only care about one pattern. **Syntactic sugar** means syntax that makes code easier or shorter to write without adding new functionality.

## 📘 **`let ... else` Construct**
- The `let...else` construct attempts to match a value against a specific `enum` **variant** and extract its associated data.
- If the value matches the **variant**, the extracted variables are declared and remain available in the code that follows.
- If the value does not match the **variant**, the `else` block is executed.
- The `else` block must diverge, meaning that it must exit the current flow using something like `return`, `break`, `continue`, or `panic!`.
- Declare the hardcoded `enum` **variant** on the left-hand side of `=` and the value being checked on the right-hand side.
- Use `let...else` when you expect one specific pattern to be the valid case and want to **exit early** if it does not match.

```rust
enum GoatMilk {
    LowFat(u32),
    Whole,
    NonDairy { kind: String }
}

fn print_low_fat_info(milk: GoatMilk) {
    // We need LowFat milk to continue
    let GoatMilk::LowFat(percent) = milk else {
        println!("This beverage is not low-fat milk.");
        return;
    };

    // If we reach this point, Rust guarantees that `percent` exists
    println!("Low-fat milk detected!");
    println!("Fat percentage: {}%", percent);

    if percent <= 2 {
        println!("This milk is very low in fat.");
    }
}
fn main() {
    let my_beverage: GoatMilk = GoatMilk::Whole;

    print_low_fat_info(my_beverage);
}
```

**Key idea**: `let ... else` is useful for **early exists**. It extracts the data you need when the pattern matches; otherwise, the else block exits the current flow.