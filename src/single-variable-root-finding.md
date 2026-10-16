# Single-Variable Root Finding

## Bisection Method

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let eps: f64 = 1e-6;
let function = |x: f64| x.exp() - 2.0;

let mut a: f64 = -2.0;
let mut b: f64 = 2.0;

let mut f_a = function(a);
let f_b = function(b);

let mut it = 0;
let mut m = (a + b) / 2.0;

while b - a > eps {
    m = a + (b - a) / 2.0;
    let f_m = function(m);

    it += 1;

    if f_a.signum() == f_m.signum() {
        a = m;
        f_a = f_m;
    } else {
        b = m;
    }
}

let end = start.elapsed();

println!("Approximate root: {} | Iterations: {}", m, it);
println!("Bisection Method took {} microseconds.", end.as_micros());
# }
```

## Newton–Raphson Method and Extensions

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let eps: f64 = 1e-6;
let function = |x: f64| x.exp() - 2.0;
let derivative = |x: f64| x.exp();

// Initial estimation
let mut x_1: f64 = 2.0; 

let mut it = 0;
while function(x_1).abs() > eps {
    let x_2 = x_1 - function(x_1)/derivative(x_1);
    x_1 = x_2;
    it += 1;
}

let end = start.elapsed();

println!("Approximate root: {} | Iterations: {}", x_1, it);
println!("Newton–Raphson Method took {} microseconds.", end.as_micros());
# }
```

### Secant Method

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let eps: f64 = 1e-6;
let function = |x: f64| x.exp() - 2.0;

// Initial estimations
let mut x_0: f64 = -2.0;
let mut x_1: f64 = 2.0;

let mut it = 0;
while function(x_1).abs() > eps {
    let f_0 = function(x_0);
    let f_1 = function(x_1);

    let denominator = f_1 - f_0;

    if denominator.abs() < f64::EPSILON {
        panic!("The secant method cannot continue: denominator is too small.");
    }

    let x_2 = x_1 - f_1 * (x_1 - x_0) / denominator;

    x_0 = x_1;
    x_1 = x_2;
    it += 1;
}

let end = start.elapsed();

println!("Approximate root: {} | Iterations: {}", x_1, it);
println!("Secant Method took {} microseconds.", end.as_micros());
# }
```

### False Position Method

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let eps: f64 = 1e-6;
let function = |x: f64| x.exp() - 2.0;

// Initial interval: function(x_0) and function(x_1) must have opposite signs
let mut x_0: f64 = -2.0;
let mut x_1: f64 = 2.0;
let mut f_0 = function(x_0);
let mut f_1 = function(x_1);

if f_0 * f_1 > 0.0 {
    panic!("The initial interval must bracket a root.");
}

let mut it = 0;
let mut x_2: f64;

loop {
    let denominator = f_1 - f_0;

    if denominator.abs() < f64::EPSILON {
        panic!("False Position cannot continue: denominator is too small.");
    }

    x_2 = x_1 - f_1 * (x_1 - x_0) / denominator;
    let f_2 = function(x_2);
    it += 1;

    if f_2.abs() <= eps {
        break;
    }

    // Keep the endpoints on opposite sides of the root.
    if f_0 * f_2 < 0.0 {
        x_1 = x_2;
        f_1 = f_2;
    } else {
        x_0 = x_2;
        f_0 = f_2;
    }
}

let end = start.elapsed();

println!("Approximate root: {} | Iterations: {}", x_2, it);
println!("False Position Method took {} microseconds.", end.as_micros());
# }
```

## Fixed-Point 

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let eps: f64 = 1e-6;
let mut err: f64 = 1.0;
let function = |x: f64| (x - x.powi(2)).asin() * 0.5;

let mut x_0: f64 = 1e-4;

let mut it = 0;
while err > eps {
    let x_1 = function(x_0);
    err = (x_1 - x_0).abs();
    it += 1;
    x_0 = x_1;
}

let end = start.elapsed();

println!("Approximate root: {:e} | Iterations: {}", x_0, it);
println!("Fixed-Point Method took {} microseconds.", end.as_micros());
# }
```

## Simple Benchmark: Bisection vs Newton vs Secant vs False Position

```rust
# use std::time::Instant;
fn bisection<F>(function: F, mut a: f64, mut b: f64, eps: f64) -> (f64, usize)
where
    F: Fn(f64) -> f64,
{
    let mut f_a = function(a);
    let f_b = function(b);
    
    let mut it = 0;
    while b - a > eps {
        let m = a + (b - a) / 2.0;
        let f_m = function(m);
        it += 1;
        if f_a.signum() == f_m.signum() {
            a = m;
            f_a = f_m;
        } else {
            b = m;
        }
    }
    (a + (b - a) / 2.0, it)
}

fn newton_raphson<F, D>(function: F, derivative: D, mut x_1: f64, eps: f64,) -> (f64, usize)
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut it = 0;
    while function(x_1).abs() > eps {
        let x_2 = x_1 - function(x_1)/derivative(x_1);
        x_1 = x_2;
        it += 1;
    }
    (x_1, it)
}

fn secant<F>(function: F, mut x_0: f64, mut x_1: f64, eps: f64) -> (f64, usize)
where
    F: Fn(f64) -> f64,
{
    let mut it = 0;
    while function(x_1).abs() > eps {
        let f_0 = function(x_0);
        let f_1 = function(x_1);

        let denominator = f_1 - f_0;

        if denominator.abs() < f64::EPSILON {
            panic!("The secant method cannot continue: denominator is too small.");
        }

        let x_2 = x_1 - f_1 * (x_1 - x_0) / denominator;

        x_0 = x_1;
        x_1 = x_2;
        it += 1;
    }
    (x_1, it)
}

fn false_position<F>(function: F, mut x_0: f64, mut x_1: f64, eps: f64) -> (f64, usize)
where
    F: Fn(f64) -> f64,
{
    let mut f_0 = function(x_0);
    let mut f_1 = function(x_1);

    if f_0 * f_1 > 0.0 {
        panic!("The initial interval must bracket a root.");
    }

    let mut it = 0;
    let mut x_2: f64;

    loop {
        let denominator = f_1 - f_0;

        if denominator.abs() < f64::EPSILON {
            panic!("False Position cannot continue: denominator is too small.");
        }

        x_2 = x_1 - f_1 * (x_1 - x_0) / denominator;
        let f_2 = function(x_2);
        it += 1;

        if f_2.abs() <= eps {
            break;
        }

        // Keep the endpoints on opposite sides of the root.
        if f_0 * f_2 < 0.0 {
            x_1 = x_2;
            f_1 = f_2;
        } else {
            x_0 = x_2;
            f_0 = f_2;
        }
    }
    (x_2, it)
}

fn main() {
    let function = |x: f64| x.exp() - 2.0;
    let derivative = |x: f64| x.exp();

    // Bisection Method Benchmark
    let start = Instant::now();
    for _ in 0..100_000 {
        bisection(function, -2.0, 2.0, 1e-6);
    }
    let bisection_time = start.elapsed();
    println!("Bisection Method Benchmark {} microseconds.", bisection_time.as_micros());

    // Newton–Raphson Method Benchmark
    let start = Instant::now();
    for _ in 0..100_000 {
        newton_raphson(function, derivative, 2.0, 1e-6);
    }
    let newton_time = start.elapsed();
    println!("Newton–Raphson Method Benchmark {} microseconds.", newton_time.as_micros());

    // Secant Method Benchmark
    let start = Instant::now();
    for _ in 0..100_000 {
        secant(function, -2.0, 2.0, 1e-6);
    }
    let secant_time = start.elapsed();
    println!("Secant Method Benchmark {} microseconds.", secant_time.as_micros());

    // False Position Method Benchmark
    let start = Instant::now();
    for _ in 0..100_000 {
        false_position(function, -2.0, 2.0, 1e-6);
    }
    let false_time = start.elapsed();
    println!("False Position Method Benchmark {} microseconds.", false_time.as_micros());

}
```
