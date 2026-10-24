# Unit Tests
## 📗 **`assert!` Macro**
The `assert!` macro checks whether a condition is true. If it’s false, the test fails.
```rust,ignore
fn is_positive(number: i32) -> bool {
    number > 0
}

#[test]
fn checks_that_number_is_positive() {
    assert!(is_positive(5));
}
```
Run the test with:
```bash
cargo test
```
Expected output:
```bash
running 1 test
test checks_that_number_is_positive ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## 📗 **`assert_eq!` Macro**
The `assert_eq!` macro checks whether two values are equal. If they aren’t, the test fails and Rust displays the expected and actual values.
```rust,ignore
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn adds_two_numbers() {
    assert_eq!(add(2, 3), 5);
}
```
Run the test with:
```bash
cargo test
```
Expected output:
```bash
running 1 test
test adds_two_numbers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## 📗 **`assert_ne!` Macro**
The assert_ne! macro checks that two values are not equal. If they are equal, the test fails and Rust displays the values.
```rust,ignore
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn sum_is_not_zero() {
    assert_ne!(add(2, 3), 0);
}
```
Run the test with:
```bash
cargo test
```
Expected output:
```bash
running 1 test
test sum_is_not_zero ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## 📗 **Running Tests in Parallel**
By default, `cargo test` runs tests in parallel using multiple threads. Tests that share resources or depend on execution order may behave unpredictably.

```rust,ignore
fn is_positive(number: i32) -> bool {
    number > 0
}

fn is_negative(number: i32) -> bool {
    number < 0
}

#[test]
fn checks_that_number_is_positive() {
    assert!(is_positive(5));
}

#[test]
fn checks_that_number_is_negative() {
    assert!(is_negative(-5));
}
```
Expected output (the order of the test names may vary):
```bash
running 2 tests
test checks_that_number_is_positive ... ok
test checks_that_number_is_negative ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
To run tests one at a time, use:
```bash
cargo test -- --test-threads=1
```
The test result is the same, but the tests run sequentially:
```bash
running 2 tests
test checks_that_number_is_negative ... ok
test checks_that_number_is_positive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 📗 **tests Module and `#[cfg(test)]` Attribute**
A **tests** module keeps unit test code organized and separate from the rest of the program.

The `#[cfg(test)]` attribute tells Rust to compile this module only when running tests. It is excluded from normal builds.
```rust,ignore
pub fn is_positive(number: i32) -> bool {
    number > 0
}

pub fn is_negative(number: i32) -> bool {
    number < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_that_number_is_positive() {
        assert!(is_positive(5));
    }

    #[test]
    fn checks_that_number_is_negative() {
        assert!(is_negative(-5));
    }
}
```
Expected output:
```bash
running 2 tests
test tests::checks_that_number_is_negative ... ok
test tests::checks_that_number_is_positive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 📗 **Test Failures**
A failed test does not stop the test suite. Rust continues running the remaining tests, then reports which ones passed or failed.
```rust,ignore
pub fn is_positive(number: i32) -> bool {
    number > 0
}

pub fn is_negative(number: i32) -> bool {
    number < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_that_number_is_positive() {
        assert!(is_positive(5));
    }

    #[test]
    fn ensure_that_number_is_positive() {
        assert!(is_positive(-5));
    }

    #[test]
    fn checks_that_number_is_negative() {
        assert!(is_negative(5));
    }

    #[test]
    fn ensure_that_number_is_negative() {
        assert!(is_negative(5));
    }
}
```
Expected output:
```bash
running 4 tests
test tests::checks_that_number_is_negative ... FAILED
test tests::checks_that_number_is_positive ... ok
test tests::ensure_that_number_is_negative ... FAILED
test tests::ensure_that_number_is_positive ... FAILED
```
We can add custom failures messages
```rust,ignore
pub fn is_positive(number: i32) -> bool {
    number > 0
}

pub fn is_negative(number: i32) -> bool {
    number < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_that_number_is_positive() {
        assert!(is_positive(5), "5 should be positive");
    }

    #[test]
    fn ensure_that_number_is_positive() {
        assert!(is_positive(-5), "-5 should be positive");
    }

    #[test]
    fn checks_that_number_is_negative() {
        assert!(is_negative(5), "5 should be negative");
    }

    #[test]
    fn ensure_that_number_is_negative() {
        assert!(is_negative(5), "5 should be negative");
    }
}
```
## 📗 **pretty_assertions crate**
[Github](https://github.com/rust-pretty-assertions/rust-pretty-assertions)

[crates.io](https://crates.io/crates/pretty_assertions)

The **Cargo.toml**
```toml
[package]
name = "unit_test"
version = "0.1.0"
edition = "2024"

[dev-dependencies]
pretty_assertions = "1.4.1"
```
The test code
```rust,ignore
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 4);
    }
}
```
Expected output:
```bash
running 1 test
test tests::adds_two_numbers ... FAILED

failures:

---- tests::adds_two_numbers stdout ----

thread 'tests::adds_two_numbers' (26185) panicked at src/lib.rs:13:9:
assertion failed: `(left == right)`

Diff < left / right > :
<5
>4
```
## 📗 **`#[should_panic]` Attribute**
`#[should_panic]` makes a test pass when it panics. Use it to check that code handles an invalid case by panicking. If the test does not panic, it fails.
```rust,ignore
pub fn is_positive(number: i32) -> bool {
    number > 0
}

pub fn is_negative(number: i32) -> bool {
    number < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn checks_that_number_is_positive() {
        assert!(is_positive(-45));
    }

    #[test]
    #[should_panic]
    fn ensure_that_number_is_positive() {
        assert!(is_positive(-5));
    }
}
```
Expected output:
```bash
running 2 tests
test tests::checks_that_number_is_positive - should panic ... ok
test tests::ensure_that_number_is_positive - should panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## 📗 **Running a Subset of Tests Attribute**
Run a single test by specifying its full name:
```bash
cargo test tests::checks_that_number_is_positive
```
Run all tests in a module by specifying the module name followed by `::`
```bash
cargo test tests::
```
## 📗 **Ignoring Tests**
Add the `#[ignore]` attribute to skip a test when running `cargo test`. Ignored tests are still compiled, but they don’t run by default.
```rust,ignore
fn is_positive(number: i32) -> bool {
    number > 0
}

fn is_negative(number: i32) -> bool {
    number < 0
}

#[test]
fn checks_that_number_is_positive() {
    assert!(is_positive(5));
}

#[test]
#[ignore]
fn checks_that_number_is_negative() {
    assert!(is_negative(-5));
}
```
Run ignored tests with:
```bash
cargo test -- --ignored
```
## 📘 **Deriving the `Eq` and `PartialEq` Trait**
- **`PartialEq`** lets you compare values using `==` and `!=`.
- **`Eq`** means a value is always equal to itself. It is used together with `PartialEq`
```rust,ignore
#[derive(Debug, Eq, PartialEq)]
struct User {
    id: u32,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn users_with_same_fields_are_equal() {
        let user1 = User {
            id: 1,
            name: String::from("Ana"),
        };

        let user2 = User {
            id: 1,
            name: String::from("Ana"),
        };

        assert_eq!(user1, user2);
    }

    #[test]
    fn users_with_different_ids_are_not_equal() {
        let user1 = User {
            id: 1,
            name: String::from("Ana"),
        };

        let user2 = User {
            id: 2,
            name: String::from("Ana"),
        };

        assert_ne!(user1, user2);
    }
}
```
Expected output:
```bash
running 2 tests
test tests::users_with_different_ids_are_not_equal ... ok
test tests::users_with_same_fields_are_equal ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## 📘 **`Result` Enum in Tests**
A test can return `Result<(), String>`. Return `Ok(())` when it passes, or `Err(...)` when it fails. Rust treats an `Err` result as a failed test.
```rust,ignore
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_returns_ok() -> Result<(), String> {
        assert_eq!(divide(10, 2)?, 5);
        Ok(())
    }

    #[test]
    fn division_returns_err() {
        assert_eq!(divide(10, 0), Err(String::from("cannot divide by zero")));
    }
}
```
Expected output:
```bash
running 2 tests
test tests::division_returns_err ... ok
test tests::division_returns_ok ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests unit_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```