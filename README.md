# Linrow

[![Crates.io](https://img.shields.io/crates/v/linrow.svg)](https://crates.io/crates/linrow)
[![Docs.rs](https://docs.rs/linrow/badge.svg)](https://docs.rs/linrow)

A Rust library for generic matrix operations, numerical linear algebra, and exact symbolic computations.
Linrow provides fundamental data structures for matrices and rows, supporting algorithms for transforming matrices into Row Echelon Form (REF) and Reduced Row Echelon Form (RREF), solving systems of linear equations, and computing matrix inverses and pseudoinverses.
By leveraging generic scalar traits, `linrow` supports both standard floating-point types (`f32`, `f64`) for numerical efficiency and exact symbolic types (such as `num_rational::Rational64` and `num_complex::Complex`) to perform zero-loss matrix calculations without floating-point rounding errors.
It is also trivial to add support for other types, but for simplicity `num_decimal` support is planned.

## Features

- **Matrix Representation**: A `Matrix` struct built upon `Row`s, supporting dynamic dimensions.
- **Row Operations**: `Row` struct with overloaded operators for scalar multiplication/division, and row addition/subtraction.
- **Gaussian Elimination**: Algorithms to convert matrices to:
  - **Row Echelon Form (REF)**: `row_echelon()`
  - **Reduced Row Echelon Form (RREF)**: `reduced_row_echelon()`
- **Matrix-Vector Multiplication**: Supports multiplication of a `Matrix` by a `Row` (interpreted as a column vector).
- **Error Handling**: Robust error handling for invalid matrix creations.
- **Floating Point Precision**: Uses a small `EPSILON` for stable floating-point comparisons.
- **Orthonormal checking**: Checks if a basis is orthonormal
- **Inversion**: Invert matrices
- **Full complex number and fraction support**: Leverages num-rational and num-complex. Arbritrary types are also supported by simply implementing Scalar, which can easily be implemented by implementing metric and HasConj.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
linrow = "2.8.0" # Or the latest version
```

Or just use

```fish
cargo add linrow
```

Feature flags:

- Complex
- Rational

To install with complex support for instance:

```fish
cargo add linrow --features complex
```

## Usage

Here's a quick example of how to create a matrix and reduce it to its Reduced Row Echelon Form (RREF):

```rust
use linrow::matrix::Matrix;
use linrow::def_matrix;
use linrow::operation_logger::NoopLogger;
fn main() {
    // Define a system of linear equations cleanly as a matrix:
    // 1x + 2y + 3z = 9
    // 2x - 1y + 1z = 8
    // 3x + 0y - 1z = 3
    let mut matrix = def_matrix![
        [1.0,  2.0,  3.0, 9.0],
        [2.0, -1.0,  1.0, 8.0],
        [3.0,  0.0, -1.0, 3.0],
    ].unwrap();

    println!("Original Matrix:\n{}", matrix);

    // Convert the matrix to Reduced Row Echelon Form (RREF)
    matrix.reduced_row_echelon(&mut NoopLogger {});

    println!("Matrix in RREF:\n{}", matrix);

    // For this specific system, the RREF will directly give the solution:
    // x = 2, y = -1, z = 3
    // The matrix will look something like:
    // [[1, 0, 0, 2],
    //  [0, 1, 0, -1],
    //  [0, 0, 1, 3]]
}
```

With the complex feature flag set, it also now supports complex numbers. Comes built in with num-complex, but it is very easy to add your complex number implementation. Check traits.rs, especially docs for Scalar, to understand how.

```rust
fn main() {
    let c = |re: f64, im: f64| Complex::new(re, im);

    // System:
    // (1 + 1i)x + (2 - 1i)y = 5 + 1i
    // (0 + 2i)x + (1 + 3i)y = -1 + 5i
    let mut matrix = def_matrix![
        [c(1.0, 1.0), c(2.0, -1.0), c(5.0, 1.0)],
        [c(0.0, 2.0), c(1.0, 3.0), c(-1.0, 5.0)]
    ].unwrap();

    matrix.reduced_row_echelon(&mut linrow::NoopLogger {});

    println!("Complex Matrix in RREF:\n{}", matrix);
    // Solution: x = 0.25 - 1.25i, y = 1.0 + 1.5i
}
```

Likewise it also can solve systems with rational numbers, and it comes built in with num-rational. For example:

```rust

use linrow::def_matrix;
use linrow::matrix::Matrix;
use linrow::matrix::conjugate_matrix;
use linrow::operation_logger::NoopLogger;
use linrow::utils::conjugate_transpose;
use num_complex::Complex;
use num_rational::Rational64;
fn main() {
    let c = |c: i64| Complex::new(Rational64::ZERO, Rational64::new(c, 1));
    let r = |r: i64| Complex::new(Rational64::new(r, 1), Rational64::ZERO);
    let rc = |r_num: i64, c_num: i64| r(r_num) + c(c_num);
    let matrix_a = def_matrix!([rc(1, 1), r(-1)], [r(1), rc(2, -1)], [c(1), r(0)]).unwrap();
    let matrix_a_transpose = conjugate_transpose(&matrix_a.clone());
    let mut psuedo_inverse_intermediate = &matrix_a_transpose * &matrix_a;
    psuedo_inverse_intermediate
        .invert()
        .expect("Failed to invert");
    let pseudo_inverse = matrix_a * psuedo_inverse_intermediate * matrix_a_transpose;
    println!("{pseudo_inverse}");
}
```

```text
Output: "[[18/23+0i,-2/23-1/23i,6/23-7/23i],[-2/23+1/23i,22/23+0i,1/23-4/23i],[6/23+7/23i,1/23+4/23i,6/23+0i]]"
```

For more detailed examples and API documentation, please refer to the [docs.rs page](https://docs.rs/linrow).

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT - see the [LICENSE.md](LICENSE.md) file for details.
