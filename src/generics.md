# ⚙️ **Generics**
A generic is a type parameter that allows code to work with different types while preserving type safety.

It acts as a placeholder for a future type, just as a function parameter acts as a placeholder for a future value.

## 📗 **Generics in Functions**
We can use the **turbofish** operator `::<type>` to customize the type of the generic 
```rust
# fn main() {
fn identity<T>(value: T) -> T {
    value
}  

println!("identity: {}", identity("Hello World!"));
println!("identity: {}", identity(5));
println!("identity: {}", identity(5.0));
println!("identity: {}", identity::<i8>(5)); 
# }
```
We can define multiple **generics**
```rust
# fn main() {
fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

println!("make_tuple: {:?}", make_tuple(5.0, "Tuple"));
println!("make_tuple: {:?}", make_tuple::<i8, f32>(5, 32.0));
# }
```

## 📗 **Generics in Structs**
```rust
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    let silver_chest = TreasureChest::<String> {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"),
    };
    let black_pearl = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: 134,
    };
    println!("gold_chest: {:?}", gold_chest);
    println!("silver_chest: {:?}", silver_chest);
    println!("black_pearl: {:?}", black_pearl);
}
```


## 📗 **Generics in `impl` Blocks**
We can define **methods** for any type `T` or implement **methods** for only a specific type
```rust
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

impl TreasureChest<i32> {
    fn double_value(&self) -> i32 {
        self.treasure * 2
    }
}

fn main() {
    let silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"),
    };

    let black_pearl: TreasureChest<i32> = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: 134,
    };
    
    println!("silver_chest capital captain: {}", silver_chest.capital_captain());
    println!("black_pearl double value: {}", black_pearl.double_value());
}
```

## 📗 **Generics in Enums**

```rust
#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T)
}

fn main() {
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topping::<&str>("mushroom"); 
    let onion: Cheesesteak<String> = Cheesesteak::Topping("onions".to_string()); 
    let plain: Cheesesteak<&str> = Cheesesteak::Plain;
    
    println!("mushroom: {:?}", mushroom);
    println!("onion: {:?}", onion);
    println!("plain: {:?}", plain);
}
```