# Direct Methods

## Gaussian Elimination


```rust,ignore
use ndarray::{array, concatenate, Axis, Array1, Array2};

fn gaussian_elim(a: &Array2<f64>, b: &Array1<f64>) -> Option<Array1<f64>> {
    // Number of rows
    let n = b.len();

    // Augmented Matrix [A | b]
    let mut ab = concatenate![Axis(1), a.view(), b.view().insert_axis(Axis(1))];

    // Get triangular matrix -> Elimination process
    for m in 0..n {
        let div = ab[[m, m]];
        if div == 0.0 {
            return None;
        }

        for j in 0..=n {
            ab[[m, j]] /= div;
        }

        for i in (m + 1)..n {
            let mult = ab[[i, m]];
            for j in 0..=n {
                ab[[i, j]] -= mult * ab[[m, j]];
            }
        }
    }

    // Back substitution
    let mut x = Array1::<f64>::zeros(n);
    for m in (0..n).rev() {
        x[m] = ab[[m, n]];
        for i in (m + 1)..n {
            x[m] -= ab[[m, i]] * x[i];
        }
    }

    Some(x)
}

fn main() {
    let a: Array2<f64> = array![
        [2.0, 1.0, 4.0, 1.0],
        [3.0, 4.0, -1.0, 1.0],
        [1.0, -4.0, 1.0, 5.0],
        [2.0, -2.0, 1.0, 3.0],
    ];

    let b: Array1<f64> = array![-4.0, 3.0, 9.0, 7.0];

    println!("A =\n{:.2}\n", a);
    println!("B =\n{:.2}\n", b);

    match gaussian_elim(&a, &b) {
        Some(x) => {
            // Check solution ->  A·x
            let ax = a.dot(&x);
            println!("Solution x =\n{:.2}\n", x);
            println!("A·x =\n{:.2}\n", ax);

        },
        None => println!("The system has no unique solution.")
    }
}
```

## Pivoting Method

## LU Factorization

## Cholesky Factorization

## [ndarray-linalg](https://crates.io/crates/ndarray-linalg)
