# Integration Tests
Integraton tests are larger in scope than unit tests. They test a feature as a whole. 

By convention, integration tests are stored in the `tests` directory at the top level of a Cargo project:

```text
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```
Each file in tests is compiled as a separate crate. That means integration tests use your library the way an external program would: import it by its crate name and access only its public items.
```rust,ignore
// tests/integration_test.rs
use my_project::add_two;

#[test]
fn adds_two() {
    assert_eq!(add_two(2), 4);
}
```
Run unit and integration tests with:
```bash
cargo test
```
To run only the integration tests in a particular file:
```bash
cargo test --test integration_test
```

## 📗 **Documentation Tests**
Documentation tests, or doc tests, are Rust code examples written inside documentation comments. They show how to use your crate and verify that the examples compile and run.
```rust,ignore
/// Adds two to the given number.
///
/// # Examples
///
/// ```
/// let result = my_project::add_two(2);
/// assert_eq!(result, 4);
/// ```
pub fn add_two(num: i32) -> i32 {
    num + 2
}
```
Run documentation tests with:
```bash
cargo test
```
Cargo runs doc tests along with unit and integration tests. To run only the documentation tests:
```bash
cargo test --doc
```

## 📗 **`rstest` Crate**
`rstest` is a third-party crate that provides macros for writing parameterized tests and reusable test fixtures.

[rstest Github](https://github.com/la10736/rstest)

[rstest crates.io](https://crates.io/crates/rstest)
### **Parameterized tests**
Use `#[rstest]` and `#[case(...)]` to run the same test with different inputs. Each case becomes a separate test:
```rust,ignore
use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(6, 8)]
fn fibonacci_returns_expected_value(#[case] input: u32, #[case] expected: u32) {
    assert_eq!(fibonacci(input), expected);
}
```
### **Fixtures**
A fixture creates a value that tests can reuse. Mark its function with `#[fixture]`, then request it as an argument in a test marked with `#[rstest]`:
```rust,ignore
use rstest::{fixture, rstest};

#[fixture]
fn number() -> u32 {
    42
}

#[rstest]
fn number_is_answer(number: u32) {
    assert_eq!(number, 42);
}
```

## 📗 **Test Driven Development (TDD)**
- TDD is a paradigm that encourages you to write your test first
- TDD will guarantee that you have tests for your code
- TDD reduces overtesting (writing the same test multiple times)
- TDD forces you to define your contract before you write code
- TDD simplifies the complexity of both tests and the implementation

![tdd cycle](https://womanonrails.com/images/tdd-basics/tdd.gif)

