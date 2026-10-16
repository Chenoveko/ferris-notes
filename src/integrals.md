# Integrals 

## Single-Variable Integration

### Newton–Cotes Rules
Based on Newton-Gregory interpolation
- Rectangle Rule — degree 0
- Trapezoidal Rule — degree 1
- Simpson’s Rule — degree 2


#### Rectangle Rule
```rust
use std::time::Instant;

fn rectangle<F>(function: F, a: f64, b: f64, n: usize,) -> f64
where
    F: Fn(f64) -> f64
{
    let h: f64 = (b - a) / n as f64;
    let mut value: f64 = 0.0;
    for i in 0..n {
        value += h * function(a + h * i as f64);
    }
    value
}

fn main() {
    let function = |x: f64| x.powi(4) - 2.0 * x + 1.0;

    let start = Instant::now();
    let value: f64 = rectangle(function, 0.0, 2.0, 10);
    let end = start.elapsed();

    println!("Approximate integration: {}", value);
    println!("Rectangle Rule took {} microseconds.", end.as_micros());
}
```

#### Trapezoidal Rule
```rust
use std::time::Instant;

fn trap<F>(function: F, a: f64, b: f64, n: usize,) -> f64
where
    F: Fn(f64) -> f64
{
    let h: f64 = (b - a) / n as f64;
    let mut value: f64 = (function(a) + function(b))/ 2.0;
    for i in 0..n {
        value += function(a + h * i as f64);
    }
    value * h
}

fn main() {
    let function = |x: f64| x.powi(4) - 2.0 * x + 1.0;

    let start = Instant::now();
    let value: f64 = trap(function, 0.0, 2.0, 10);
    let end = start.elapsed();

    println!("Approximate integration: {}", value);
    println!("Trapezoidal Rule took {} microseconds.", end.as_micros());
}
```
#### Simpson’s Rule
```rust
use std::time::Instant;

fn simpson<F>(function: F, a: f64, b: f64, n: usize,) -> f64
where
    F: Fn(f64) -> f64
{
    assert!(n > 0 && n % 2 == 0, "n must be even and greater than 0");

    let h: f64 = (b - a) / n as f64;
    let mut value: f64 = function(a) + function(b);
    
    for i in 1..n {
        let weight: f64 = if i % 2 == 0 { 2.0 } else { 4.0 };
        value += weight * function(a + i as f64 * h);
    }

    value * h / 3.0
}

fn main() {
    let function = |x: f64| x.powi(4) - 2.0 * x + 1.0;

    let start = Instant::now();
    let value: f64 = simpson(function, 0.0, 2.0, 10);
    let end = start.elapsed();

    println!("Approximate integration: {}", value);
    println!("Simpson Rule took {} microseconds.", end.as_micros());
}
```

### Romberg's Method 

### Gaussian Quadrature
```rust
use std::time::Instant;

fn gauss_legendre(a: f64, b: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    assert!(n > 0, "n must be greater than 0");

    let mut points = vec![0.0; n];
    let mut weights = vec![0.0; n];
    let midpoint = (a + b) / 2.0;
    let half_length = (b - a) / 2.0;

    for i in 0..n.div_ceil(2) {
        // Initial guess for the i-th root
        let mut x = (std::f64::consts::PI * (i as f64 + 0.75)
            / (n as f64 + 0.5))
            .cos();

        // Newton's method for finding the root of P_n(x)
        let derivative = loop {
            let mut p0 = 1.0;
            let mut p1 = x;

            for k in 2..=n {
                let kf = k as f64;
                let p = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                p0 = p1;
                p1 = p;
            }

            let (pn, pnm1) = if n == 1 { (x, 1.0) } else { (p1, p0) };
            let dpn = n as f64 * (x * pn - pnm1) / (x * x - 1.0);
            let next_x = x - pn / dpn;

            if (next_x - x).abs() < 1e-14 {
                x = next_x;

                // Recompute P'_n at the converged root for the weight.
                let mut p0 = 1.0;
                let mut p1 = x;
                for k in 2..=n {
                    let kf = k as f64;
                    let p = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                    p0 = p1;
                    p1 = p;
                }

                let (pn, pnm1) = if n == 1 { (x, 1.0) } else { (p1, p0) };
                break n as f64 * (x * pn - pnm1) / (x * x - 1.0);
            }

            x = next_x;
        };

        let weight = 2.0 / ((1.0 - x * x) * derivative * derivative);

        points[i] = midpoint - half_length * x;
        points[n - 1 - i] = midpoint + half_length * x;
        weights[i] = half_length * weight;
        weights[n - 1 - i] = half_length * weight;
    }

    (points, weights)
}

fn gauss_quad<F>(function: F, a: f64, b: f64, n: usize,) -> f64
where
    F: Fn(f64) -> f64
{
    assert!(n > 0, "n must be greater than 0");

    let (points, weights) = gauss_legendre(a, b, n);

    let mut value: f64 = 0.0;
    for i in 0..n {
        value += weights[i] * function(points[i]);
    }

    value 
}

fn main() {
    let function = |x: f64| x.powi(4) - 2.0 * x + 1.0;

    let start = Instant::now();
    let value: f64 = gauss_quad(function, 0.0, 2.0, 10);
    let end = start.elapsed();

    println!("Approximate integration: {}", value);
    println!("Gaussian Quadrature took {} microseconds.", end.as_micros());
}
```

## Infinite-Range Integrals

## Multiple Integrals

### Double Integrals over Rectangular Regions
**Gravitational Attraction of a Metal Sheet**
```rust
use std::time::Instant;

fn gauss_legendre(a: f64, b: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    assert!(n > 0, "n must be greater than 0");

    let mut points = vec![0.0; n];
    let mut weights = vec![0.0; n];
    let midpoint = (a + b) / 2.0;
    let half_length = (b - a) / 2.0;

    for i in 0..n.div_ceil(2) {
        // Initial guess for the i-th root
        let mut x = (std::f64::consts::PI * (i as f64 + 0.75)
            / (n as f64 + 0.5))
            .cos();

        // Newton's method for finding the root of P_n(x)
        let derivative = loop {
            let mut p0 = 1.0;
            let mut p1 = x;

            for k in 2..=n {
                let kf = k as f64;
                let p = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                p0 = p1;
                p1 = p;
            }

            let (pn, pnm1) = if n == 1 { (x, 1.0) } else { (p1, p0) };
            let dpn = n as f64 * (x * pn - pnm1) / (x * x - 1.0);
            let next_x = x - pn / dpn;

            if (next_x - x).abs() < 1e-14 {
                x = next_x;

                // Recompute P'_n at the converged root for the weight.
                let mut p0 = 1.0;
                let mut p1 = x;
                for k in 2..=n {
                    let kf = k as f64;
                    let p = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                    p0 = p1;
                    p1 = p;
                }

                let (pn, pnm1) = if n == 1 { (x, 1.0) } else { (p1, p0) };
                break n as f64 * (x * pn - pnm1) / (x * x - 1.0);
            }

            x = next_x;
        };

        let weight = 2.0 / ((1.0 - x * x) * derivative * derivative);

        points[i] = midpoint - half_length * x;
        points[n - 1 - i] = midpoint + half_length * x;
        weights[i] = half_length * weight;
        weights[n - 1 - i] = half_length * weight;
    }

    (points, weights)
}

fn fz_sheet(z: f64) -> f64 {
    let (points, weights) = gauss_legendre(-L/2.0, L/2.0, N);

    let mut value: f64 = 0.0;
    for i in 0..N {
        for j in 0..N {
            let r_squared = points[i].powi(2) + points[j].powi(2) + z.powi(2);
            value += weights[i] * weights[j] / r_squared.powf(1.5);
        }
    }
    value * G * RHO * z 
}


// Problem Constants
const L: f64 = 10.0; // m
const G: f64 = 6.67430e-11; // m³ kg⁻¹ s⁻²
const N: usize = 10; 
const RHO: f64 = 100.0; // Kg m⁻²

fn main() {
    let start = Instant::now();

    let z_min: f64 = 0.0;
    let z_max: f64 = 1.0;
    let step: f64 = (z_max - z_min) / (N - 1) as f64;
    let z_points: Vec<f64> = (0..N)
        .map(|i| z_min + i as f64 * step)
        .collect();

    let fz_values: Vec<f64> = z_points
        .iter()
        .map(|&z| fz_sheet(z))
        .collect();

    for (&z, &fz) in z_points.iter().zip(&fz_values) {
        println!("z = {z:.4}, Fz = {fz:.2e}");
    }
  
    let end = start.elapsed();
    println!("Problem took {} microseconds.", end.as_micros());
}
```





### Double Integrals over Nonrectangular Regions

