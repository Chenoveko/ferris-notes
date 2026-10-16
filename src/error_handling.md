# 🚨 Error Handling

**Error handling** is the process of dealing with potential errors from operations that can go wrong.

Two general categories of errors:
- **Recoverable errors** -> we can define code to handle

- **Unrecoverable errors** -> cause the program to be unable to proceed. Example:

```rust,ignore
None.unwrap()
```

The **backtrace** is the list of files and functions that were running at the point that the error ocurred

## 📗 **`panic!` Macro**
The `panic!` macro stops the program when it encounters an unrecoverable error. It prints a panic message and, by default, a backtrace may be available.
```rust,ignore
println!("Starting the program...");
panic!("Something went wrong");
// This line is never reached.
// println!("This will not be printed");
```
Rust is actually going to clean up the stack entries and any kind of data thas has been created within the program, and that just ensures that there is no data corruption or memory corruption from leftover data.


## 📗 **`process` Module**

```rust
use std::process;

fn main() {
    // Return 0 if there is no error, 1 something went wrong
    process::exit(0); 
}
```

## 📗 **Standard Error**
The `eprintln!` macro prints messages to the standard error, in comparison to `println!` which prints messages to the standard output
```rust,ignore
# fn main() {
println!("Standard Output (stdout)")
eprintln!("Standard Error (stderr)")
# }
```

## 📗 **Opening a File with `fs` Module (File System)**

```rust,ignore
# fn main() {
let file = match File::open("story.txt") {
    Ok(file) => file,
    Err(error) => {
        eprintln!("Something went wrong reading the file. The error was {:?}", error);
        process::exit(1);
    }
};

println!("File opened successfully: {:?}", file);
# }
```

## 📗 **Asking the User for Input**

```rust,ignore
# fn main() {
println!("Enter the name of the file: ");

let mut input: String = String::new();

let user_requested_file = stdin().read_line(&mut input);

if let Err(error) = user_requested_file {
    eprintln!("Something went wrong collecting the user input. The error was {:?}", error);
    process::exit(1);
}

let mut file = match File::open(&input.trim()) {
    Ok(file) => file,
    Err(error) => {
        eprintln!("Something went wrong reading the file. The error was {:?}", error);
        process::exit(1);
    }
};

println!("File opened successfully with user input: {:?}", file);
# }
```

## 📗 **Reading File's Content**

```rust,ignore
# fn main() {
let mut file_content: String = String::new();

let read_operation = file.read_to_string(&mut file_content);

if let Err(error) = read_operation {
    eprintln!("Something went wrong reading the file content. The error was {:?}", error);
    process::exit(1);
}

println!("File Content: {}", file_content);
# }
```

## 📙 **Try Operator `?` with `Result` Enum**
Rust uses the `Result<T, E>` enum for operations that can succeed or fail:
- `Ok(value)` means the operation succeeded and contains a value of type `T`.
- `Err(error)` means the operation failed and contains an error of type `E`.

You can handle a `Result` with `match`, `if let`, or the `?` operator:
- `match` -> lets you handle `Ok` and `Err` separately
- `if let Err(error)` -> ets you handle only the error case
- `?` -> returns the error to the caller, or unwraps the success value so the program can continue.

Handling the `Result` with `match`:
```rust,ignore
use std::fs::File;
use std::process;

fn main() {
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {error}");
            process::exit(1);
        }
    };

    // Use `file` here.
}
```
Handling the `Result` with `?`:
```rust,ignore
use std::fs;
use std::io;

fn read_file() -> io::Result<String> {
    let contents = fs::read_to_string("story.txt")?;
    Ok(contents)
}

fn main() {
    match read_file() {
        Ok(contents) => println!("{contents}"),
        Err(error) => eprintln!("Error reading the file: {error}"),
    }
}
```
If `read_to_string` succeeds, `?` gives you the file contents, and the function returns them as `Ok(contents)`.

If it fails, `?` returns the error from `read_file` immediately. The `match` in `main` then handles that error and prints a message.


## 📙 Try Operator `?` with the `Option` Enum

Rust uses the `Option<T>` enum for values that may or may not be present:

- `Some(value)` means a value is present.
- `None` means there is no value.

The `?` operator works with `Option` too:

- If the value is `Some(value)`, `?` unwraps it and lets the function continue with `value`.
- If the value is `None`, `?` returns `None` from the function immediately.

The function using `?` must return an `Option` type.

Handling `Option` with `match`
```rust,ignore
fn find_name() -> Option<String> {
    Some(String::from("Ada"))
}

fn get_name() -> Option<String> {
    let name = match find_name() {
        Some(name) => name,
        None => return None,
    };

    Some(name)
}
```
Handling `Option` with `?`

```rust,ignore
fn find_name() -> Option<String> {
    Some(String::from("Ada"))
}

fn get_name() -> Option<String> {
    let name = find_name()?;
    Some(name)
}
```
If `find_name()` returns `Some(name)`, `?` extracts the name, and `get_name()` returns it inside `Some(name)`. If `find_name()` returns `None`, `?` makes `get_name()` return `None` immediately.