# Collecting Command Line Arguments
Rust can read command-line arguments using `env::args()`
```rust
// Use 'env' submomule
use std::env;

fn main() {
    let args = env::args();

    for arg in args {
        println!("{}", arg);
    }
}
```
## Passing Arguments with Cargo
When running the program with Cargo, use `--` to separate Cargo's arguments from your program's arguments:
```bash
cargo run -- argument1 argument2
```
Everything after `--` is passed to your program. Without `--`, Cargo may interpret those values as its own options.
For example:
```bash
cargo run -- hello world
```
The output will look similar to:
```bash
target/debug/my_program 
hello 
world
```
The first item returned by `env::args()` is the name or path of the executable. If we only want to process the user's arguments, excluding the executable name, we can use `skip(1)`
```rust
// Use 'env' submomule
use std::env;

fn main() {
    let args = env::args();

    for arg in args.skip(1) {
        println!("{}", arg);
    }
}
```
## Complete Example of CLI
Arguments
- Video File Name -> `String`
- Subtitles -> `bool`
- High Definition -> `bool`
```bash
cargo run -- rust.mp4 true false nonsense
```
```rust,ignore
use std::env;
use std::process;

struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}

fn main() {
    let settings = collect_settings();
    println!("{settings:?}");
}

fn collect_settings() -> Settings {
    let mut args = env::args().skip(1).take(3);

    let video_file = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified!");
        process::exit(1);
    });

    let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

    let subtitles = settings.next().unwrap_or(false);
    let high_definition = settings.next().unwrap_or(false);

    Settings {
        video_file,
        subtitles,
        high_definition,
    }
}
```

# Read Directory
```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{entry_name:?}\n-------------");
            //let contents = fs::read_to_string(&entry_name)?;
            //println!("{contents}");
        }
    }

    Ok(())
}
```