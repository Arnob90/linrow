# matrix-solver-lib

[![Crates.io](https://img.shields.io/crates/v/linrow.svg)](https://crates.io/crates/linrow)
[![Docs.rs](https://docs.rs/linrow/badge.svg)](https://docs.rs/linrow)

A Rust library for performing various matrix operations, including solving systems of linear equations through row reduction. This crate, named `linrow`, provides fundamental data structures for matrices and rows, along with algorithms for transforming matrices into Row Echelon Form (REF) and Reduced Row Echelon Form (RREF).

## Features

- **Matrix Representation**: A `Matrix` struct built upon `Row`s, supporting dynamic dimensions.
- **Row Operations**: `Row` struct with overloaded operators for scalar multiplication/division, and row addition/subtraction.
- **Gaussian Elimination**: Algorithms to convert matrices to:
  - **Row Echelon Form (REF)**: `row_echelon()`
  - **Reduced Row Echelon Form (RREF)**: `reduced_row_echelon()`
- **Matrix-Vector Multiplication**: Supports multiplication of a `Matrix` by a `Row` (interpreted as a column vector).
- **Error Handling**: Robust error handling for invalid matrix creations.
- **Floating Point Precision**: Uses a small `EPSILON` for stable floating-point comparisons.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
linrow = "0.2.2" # Or the latest version
```

## Usage

Here's a quick example of how to create a matrix and reduce it to its Reduced Row Echelon Form (RREF):

```rust
use linrow::matrix::Matrix;

fn main() {
    // Define a system of linear equations as a matrix
    // 1x + 2y + 3z = 9
    // 2x - 1y + 1z = 8
    // 3x + 0y - 1z = 3
    let mut matrix = Matrix::new(vec![
        vec![1.0, 2.0, 3.0, 9.0],
        vec![2.0, -1.0, 1.0, 8.0],
        vec![3.0, 0.0, -1.0, 3.0],
    ]).unwrap();

    println!("Original Matrix:\n{}", matrix);

    // Convert the matrix to Reduced Row Echelon Form (RREF)
    matrix.reduced_row_echelon();

    println!("Matrix in RREF:\n{}", matrix);

    // For this specific system, the RREF will directly give the solution:
    // x = 2, y = -1, z = 3
    // The matrix will look something like:
    // [[1, 0, 0, 2],
    //  [0, 1, 0, -1],
    //  [0, 0, 1, 3]]
}
```

For more detailed examples and API documentation, please refer to the [docs.rs page](https://docs.rs/linrow).

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT - see the [LICENSE.md](LICENSE.md) file for details.
