# Single-Variable Root Finding


## Golden-Section Method

```rust
# use std::time::Instant;
# fn main() {
let start = Instant::now();

let v0: f64 = 1.0;
let sigma: f64 = 1.0;

let potential = |r: f64| v0 * ((sigma / r).powi(6) - (-r / sigma).exp());

let eps: f64 = 1e-6;
let z = (1.0 + 5.0_f64.sqrt()) / 2.0;

let mut x_1: f64 = sigma / 10.0;
let mut x_4: f64 = sigma * 10.0;
let mut x_2: f64 = x_4 - (x_4 - x_1) / z;
let mut x_3: f64 = x_1 + (x_4 - x_1) / z;

let mut f_1: f64 = potential(x_1);
let mut f_4: f64 = potential(x_2);
let mut f_2: f64 = potential(x_3);
let mut f_3: f64 = potential(x_4);

let mut it = 0;
while x_4 - x_1 > eps {
    if f_2 < f_3 {
        x_4 = x_3;
        x_3 = x_2;
        f_3 = f_2;

        x_2 = x_4 - (x_4 - x_1) / z;
        f_2 = potential(x_2);
    }
    else {
        x_1 = x_2;
        x_2 = x_3;
        f_2 = f_3;

        x_3 = x_1 + (x_4 - x_1) / z;
        f_3 = potential(x_3);
    }
    it += 1;
}

let xmin: f64 = (x_1 + x_4) / 2.0;
let end = start.elapsed();

println!("Approximate minimum: {} | Iterations: {}", xmin, it);
println!("Golden-Section Method took {} microseconds.", end.as_micros());
# }
```

## Gauss-Newton Method
