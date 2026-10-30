# **Introduction**
A set of automated computational methods for obtaining and evaluating predictions from models and theories in physics. It includes:
- Numerical computation → successive approximation to a result, typically for few-body systems
- Simulations → statistical systems in which the initial state is not specified in detail and averages are taken over many instances
- Symbolic computation

## **Computer Arithmetic**
Computers represent floating-point numbers using a sign, a significand (also called a mantissa) and an exponent. In normalized binary form, a number can be written as

\\[ (-1)^s \times 1.b_1b_2\ldots b_p \times 2^e\\]

where:
- \\(s\\) is the sign bit
- \\(b_1b_2\ldots b_p\\) are the fraction bits
- \\(e\\)  is the exponent.

[IEE 754](https://standards.ieee.org/ieee/754/6210/) defines several floating-point formats:

| Precision | Sign bits | Exponent bits | Exponent bias | Fraction bits | Total bits |
|---|---:|---:|---:|---:|---:|
| Half | 1 | 5 | 15 | 10 | 16 |
| Single | 1 | 8 | 127 | 23 | 32 |
| Double | 1 | 11 | 1,023 | 52 | 64 |
| Quadruple | 1 | 15 | 16,383 | 112 | 128 |

```rust,ignore
# #![feature(f16)]
# fn main() {
println!("f16::MAX = {:e}", f16::MAX);
println!("f32::MAX = {:e}", f32::MAX);
println!("f64::MAX = {:e}", f64::MAX);
# }
```

## Rounding and Truncation Errors

There are two main types of errors:

- **Round-off error** → arises from the finite precision used to represent numbers in a computer.
- **Truncation error** → arises from approximating an exact mathematical process using a numerical method.
