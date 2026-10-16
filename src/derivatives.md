# Derivatives with Finite-Difference Methods:

## First-Order Derivatives

### Forward Difference - 1 point

```rust
fn forward_diff<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (function(x + step) - function(x)) / step
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| forward_diff(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Backward Difference - 1 point

```rust
fn backward_diff<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (function(x) - function(x - step)) / step
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| backward_diff(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```
### Forward Difference - 3 points

```rust
fn forward_diff_3<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (-3.0 * function(x) + 4.0 * function(x + step) - function(x + 2.0 * step)) / (2.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| forward_diff_3(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Backward Difference - 3 points

```rust
fn backward_diff_3<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (3.0 * function(x) - 4.0 * function(x - step) + function(x - 2.0 * step)) / (2.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| backward_diff_3(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Central Difference - 3 points
```rust
fn central_diff<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (function(x + step) - function(x - step)) / (2.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| central_diff(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Forward Difference - 5 points

```rust
fn forward_diff_5<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (-25.0 * function(x)
        + 48.0 * function(x + step)
        - 36.0 * function(x + 2.0 * step)
        + 16.0 * function(x + 3.0 * step)
        - 3.0 * function(x + 4.0 * step))
        / (12.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| forward_diff_5(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Backward Difference - 5 points

```rust
fn backward_diff_5<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (25.0 * function(x)
        - 48.0 * function(x - step)
        + 36.0 * function(x - 2.0 * step)
        - 16.0 * function(x - 3.0 * step)
        + 3.0 * function(x - 4.0 * step))
        / (12.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| backward_diff_5(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

### Central Difference - 5 points
```rust
fn central_diff_5<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (function(x - 2.0 * step) - 8.0 * function(x - step) + 8.0 * function(x + step) - function(x + 2.0 * step)) / (12.0 * step)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| 1.0 / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| central_diff_5(function, x, 1e-8))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```
## Second-Order Derivatives

### Central Difference - 3 points
```rust
fn central_diff<F>(function: F, x: f64, step: f64) -> f64
where
    F: Fn(f64) -> f64
{
    (function(x - step) - 2.0* function(x) + function(x + step)) / step.powi(2)
}

fn main() {
    let function = |x: f64| 1.0 + 0.25 * (4.0 * x).tanh();
    let derivative = |x: f64| - 8.0 * (4.0 * x).tanh() / (4.0 * x).cosh().powi(2);

    let points: Vec<f64> = (0..10)
        .map(|i| -1.0 + i as f64 * (2.0 / 9.0))
        .collect();
    
    let diff_points: Vec<f64> = points
        .iter()
        .map(|&x| central_diff(function, x, 1e-4))
        .collect();
    
    for (&x, &numerical) in points.iter().zip(&diff_points) {
        let exact = derivative(x);
        let relative_error = ((numerical - exact) / exact).abs();
        println!(
            "x = {:.2} | numerical = {:.6} | exact = {:.6} | relative error = {:.2e}",
            x, numerical, exact, relative_error
        );
    }
}
```

## Higher-Order Derivatives
