use crate::constants::EPSILON;
use crate::row::Row;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Mul};
use thiserror::Error;
/// Represents a mathematical matrix.
///
/// A `Matrix` is composed of a vector of `Row`s, where each `Row` contains
/// a vector of `f64` values representing the elements of the matrix.
///
/// # Examples
///
/// ```
/// use matrix_solver_lib::matrix::Matrix;
///
/// let matrix_data = vec![
///     vec![1.0, 2.0, 3.0],
///     vec![4.0, 5.0, 6.0],
/// ];
/// let matrix = Matrix::new(matrix_data).unwrap();
/// ```
#[derive(Debug)]
pub struct Matrix {
    rows: Vec<Row>,
}

impl Display for Matrix {
    /// Formats the matrix for display.
    ///
    /// Each row is displayed on a single line, and rows are separated by commas.
    /// The entire matrix is enclosed in square brackets.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    ///
    /// let matrix_data = vec![
    ///     vec![1.0, 2.0],
    ///     vec![3.0, 4.0],
    /// ];
    /// let matrix = Matrix::new(matrix_data).unwrap();
    /// assert_eq!(format!("{}", matrix), "[[1, 2],[3, 4]]");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, row) in self.rows.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", row)?; // This calls Row's Display directly
        }
        write!(f, "]")
    }
}

#[derive(Debug, Error)]
pub enum MatrixCreationError {
    /// Returned when the input `Vec<Vec<f64>>` has rows of inconsistent lengths.
    #[error("All columns must be of the same size")]
    InvalidDimensionError,
    /// Returned when the input `Vec<Vec<f64>>` is empty, but its first row is also empty.
    #[error("Wtf is this empty matrix?")]
    EmptyMatrixError,
}

/// Internal enum used to track the result of moving a pivot during row operations.
enum PivotMoving {
    /// Indicates that a pivot was successfully moved to the specified `row` and `col`.
    PivotMoved { row: usize, col: usize },
    /// Indicates that the Row Echelon Form (REF) has been achieved and no more pivots can be found.
    REFAchieved,
}
impl Index<usize> for Matrix {
    type Output = Row;
    /// Allows immutable access to a specific row of the matrix using array-like indexing.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the row to access.
    ///
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
impl IndexMut<usize> for Matrix {
    /// Allows mutable access to a specific row of the matrix using array-like indexing.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the row to access mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    /// use matrix_solver_lib::row::Row;
    ///
    /// let matrix_data = vec![
    ///     vec![1.0, 2.0],
    ///     vec![3.0, 4.0],
    /// ];
    /// let mut matrix = Matrix::new(matrix_data).unwrap();
    /// matrix[0] = Row::new(vec![5.0, 6.0]);
    /// // assert_eq!(matrix[0], Row::new(vec![5.0, 6.0]));
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
impl Matrix {
    /// Creates a new `Matrix` from a `Vec<Vec<f64>>`.
    ///
    /// The input `given_raw_matrix` is a vector of vectors, where each inner vector
    /// represents a row of the matrix. All inner vectors must have the same length
    /// to form a valid matrix.
    ///
    /// # Arguments
    ///
    /// * `given_raw_matrix` - A `Vec<Vec<f64>>` representing the initial state of the matrix.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(Matrix)` if the matrix is successfully created.
    /// - `Err(MatrixCreationError::InvalidDimensionError)` if rows have inconsistent lengths.
    /// - `Err(MatrixCreationError::EmptyMatrixError)` if the matrix is empty (no rows or no columns).
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::{Matrix, MatrixCreationError};
    ///
    /// // Valid matrix
    /// let m1 = Matrix::new(vec![
    ///     vec![1.0, 2.0],
    ///     vec![3.0, 4.0],
    /// ]).unwrap();
    ///
    /// // Invalid: inconsistent row lengths
    /// let m2 = Matrix::new(vec![
    ///     vec![1.0, 2.0],
    ///     vec![3.0],
    /// ]);
    /// assert!(matches!(m2, Err(MatrixCreationError::InvalidDimensionError)));
    ///
    /// // Invalid: empty matrix (no columns)
    /// let m3 = Matrix::new(vec![
    ///     vec![],
    ///     vec![],
    /// ]);
    /// assert!(matches!(m3, Err(MatrixCreationError::EmptyMatrixError)));
    ///
    /// // Valid: empty matrix (no rows)
    /// let m4 = Matrix::new(vec![]).unwrap();
    /// assert_eq!(m4.get_dimensions(), (0, 0));
    /// ```
    pub fn new(given_raw_matrix: Vec<Vec<f64>>) -> Result<Self, MatrixCreationError> {
        if given_raw_matrix.is_empty() {
            return Ok(Matrix { rows: vec![] });
        }
        let first_row_len = given_raw_matrix.first().unwrap().len();
        for row in &given_raw_matrix {
            if row.len() != first_row_len {
                return Err(MatrixCreationError::InvalidDimensionError);
            }
        }
        if first_row_len == 0 {
            return Err(MatrixCreationError::EmptyMatrixError);
        }
        Ok(Matrix {
            rows: given_raw_matrix
                .into_iter()
                .map(|row| Row { columns: row })
                .collect(),
        })
    }
    pub fn swap_rows(&mut self, i: usize, j: usize) {
        self.rows.swap(i, j);
    }
    /// Returns the dimensions of the matrix as a tuple `(rows, columns)`.
    ///
    /// If the matrix is empty (contains no rows), it returns `(0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    ///
    /// let m1 = Matrix::new(vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![4.0, 5.0, 6.0],
    /// ]).unwrap();
    /// assert_eq!(m1.get_dimensions(), (2, 3));
    ///
    /// let m2 = Matrix::new(vec![]).unwrap();
    /// assert_eq!(m2.get_dimensions(), (0, 0));
    /// ```
    pub fn get_dimensions(&self) -> (usize, usize) {
        if self.rows.is_empty() {
            return (0, 0);
        }
        (self.rows.len(), self.rows.first().unwrap().columns.len())
    }
    /// Swaps two rows in the matrix.
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the first row.
    /// * `j` - The index of the second row.
    ///
    /// # Panics
    ///
    /// Panics if `i` or `j` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    /// use matrix_solver_lib::row::Row;
    ///
    /// let mut m = Matrix::new(vec![
    ///     vec![1.0, 2.0],
    ///     vec![3.0, 4.0],
    /// ]).unwrap();
    /// m.swap_rows(0, 1);
    /// // assert_eq!(m[0], Row::new(vec![3.0, 4.0]));
    /// // assert_eq!(m[1], Row::new(vec![1.0, 2.0]));
    /// ```
    /// Finds the first non-zero element (pivot) in or below `row_to_start`
    /// and moves its row to `row_to_start` by swapping.
    ///
    /// This is a crucial step in Gaussian elimination to find a pivot for a column.
    ///
    /// # Arguments
    ///
    /// * `row_to_start` - The row index from which to start searching for a pivot.
    ///
    /// # Returns
    ///
    /// A `PivotMoving` enum indicating whether a pivot was found and moved,
    /// or if the matrix is already in Row Echelon Form (REF) from this point onwards.
    fn move_first_pivot(&mut self, row_to_start: usize) -> PivotMoving {
        let (rows_len, columns_len) = self.get_dimensions();
        for col_idx in 0..columns_len {
            for row_idx in row_to_start..rows_len {
                if self[row_idx][col_idx].abs() > EPSILON {
                    self.swap_rows(row_idx, row_to_start);
                    return PivotMoving::PivotMoved {
                        row: row_to_start,
                        col: col_idx,
                    };
                }
            }
        }
        PivotMoving::REFAchieved
    }

    /// Reduces the rows below the given pivot row by performing row operations
    /// to make the elements below the pivot zero.
    ///
    /// This function assumes that the pivot element at `(pivot_row_idx, pivot_col_idx)`
    /// is non-zero. It first normalizes the pivot row (makes the pivot element 1)
    /// and then uses it to eliminate elements in the same column in subsequent rows.
    ///
    /// # Arguments
    ///
    /// * `(pivot_row_idx, pivot_col_idx)` - A tuple indicating the row and column
    ///   of the current pivot element.
    fn reduce_bottom_rows(&mut self, (pivot_row_idx, pivot_col_idx): (usize, usize)) {
        let (row_len, _) = self.get_dimensions();
        let to_divide = self[pivot_row_idx][pivot_col_idx];
        self[pivot_row_idx] /= to_divide;
        for row_idx in pivot_row_idx + 1..row_len {
            let scale = self[row_idx][pivot_col_idx];
            let scaled_pivot_row = self[pivot_row_idx].clone() * scale;
            self[row_idx] -= scaled_pivot_row;
        }
    }
    /// Converts the matrix into Row Echelon Form (REF) using Gaussian elimination.
    ///
    /// This method modifies the matrix in-place.
    ///
    /// The algorithm proceeds as follows:
    /// 1. Iterates through rows, finding the first non-zero element (pivot) in the current column.
    /// 2. Swaps the row containing the pivot with the current row if necessary.
    /// 3. Divides the pivot row by the pivot element to make the pivot 1.
    /// 4. Eliminates all elements below the pivot in the current column by subtracting
    ///    multiples of the pivot row from the rows below it.
    /// 5. Moves to the next row and column, repeating the process.
    ///
    /// # Returns
    ///
    /// A `Vec` of `(usize, usize)` tuples, where each tuple represents the `(row, column)`
    /// index of a pivot element in the resulting REF matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    ///
    /// let mut m = Matrix::new(vec![
    ///     vec![1.0, 2.0, -1.0, -4.0],
    ///     vec![2.0, 3.0, -1.0, -11.0],
    ///     vec![-2.0, 0.0, -3.0, 22.0],
    /// ]).unwrap();
    ///
    /// m.row_echelon();
    ///
    /// // The exact values might vary slightly due to floating point arithmetic,
    /// // but the form should be echelon.
    /// // Example expected form (not exact values):
    /// // [[1, 2, -1, -4],
    /// //  [0, 1, -1, -3],
    /// //  [0, 0, 1, -4]]
    /// ```
    pub fn row_echelon(&mut self) -> Vec<(usize, usize)> {
        let mut current_pivot_row: usize = 0;
        let mut pivot_locations: Vec<(usize, usize)> = vec![];
        loop {
            let move_first_pivot = self.move_first_pivot(current_pivot_row);
            match move_first_pivot {
                PivotMoving::PivotMoved { row, col } => {
                    self.reduce_bottom_rows((row, col));
                    current_pivot_row += 1;
                    pivot_locations.push((row, col));
                }
                PivotMoving::REFAchieved => {
                    return pivot_locations;
                }
            }
        }
    }

    /// Converts the matrix into Reduced Row Echelon Form (RREF) using Gauss-Jordan elimination.
    ///
    /// This method modifies the matrix in-place. It first calls `row_echelon` to get
    /// the matrix into Row Echelon Form, and then proceeds to eliminate elements
    /// above the pivots.
    ///
    /// The algorithm extends `row_echelon` by:
    /// 1. Identifying all pivot positions from the REF matrix.
    /// 2. For each pivot, starting from the last pivot and moving upwards,
    ///    it eliminates all elements above the pivot in its column by subtracting
    ///    multiples of the pivot row from the rows above it.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    ///
    /// let mut m = Matrix::new(vec![
    ///     vec![1.0, 2.0, 3.0, 9.0],
    ///     vec![2.0, -1.0, 1.0, 8.0],
    ///     vec![3.0, 0.0, -1.0, 3.0],
    /// ]).unwrap();
    ///
    /// m.reduced_row_echelon();
    ///
    /// // Expected RREF for a unique solution:
    /// // [[1, 0, 0, 2],
    /// //  [0, 1, 0, -1],
    /// //  [0, 0, 1, 3]]
    /// ```
    pub fn reduced_row_echelon(&mut self) {
        let pivots = self.row_echelon();
        for (pivot_row_idx, pivot_col_idx) in pivots.into_iter().rev() {
            for to_check_row in (0..pivot_row_idx).rev() {
                let item_in_col = self[to_check_row][pivot_col_idx];
                if item_in_col.abs() > EPSILON {
                    let scale = self[pivot_row_idx].clone() * item_in_col;
                    self[to_check_row] -= scale;
                }
            }
        }
    }
}

impl Mul<Row> for Matrix {
    type Output = Row;
    /// Performs matrix-vector multiplication.
    ///
    /// Multiplies this `Matrix` by a `Row` (interpreted as a column vector).
    /// The number of columns in the matrix must equal the number of elements in the row.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `Row` to multiply the matrix by.
    ///
    /// # Returns
    ///
    /// A new `Row` representing the result of the multiplication.
    ///
    /// # Panics
    ///
    /// Panics if the number of columns in the matrix does not match the number of
    /// elements in the `rhs` row.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix_solver_lib::matrix::Matrix;
    /// use matrix_solver_lib::row::Row;
    ///
    /// let m = Matrix::new(vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![4.0, 5.0, 6.0],
    /// ]).unwrap();
    ///
    /// let v = Row::new(vec![7.0, 8.0, 9.0]);
    /// let result = m * v;
    ///
    /// assert_eq!(result, Row::new(vec![50.0, 122.0]));
    /// ```
    fn mul(self, rhs: Row) -> Self::Output {
        let (rows_len, columns_len) = self.get_dimensions();
        assert_eq!(
            columns_len,
            rhs.columns.len(),
            "Cannot multiply: Matrix has {} columns but Row has {} columns",
            columns_len,
            rhs.columns.len()
        );
        let mut result_columns = vec![0.0; rows_len];
        for row_idx in 0..rows_len {
            let mut sum = 0.0;
            for col_idx in 0..columns_len {
                sum += self[row_idx][col_idx] * rhs[col_idx];
            }
            result_columns[row_idx] = sum;
        }
        Row {
            columns: result_columns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import your Matrix, Row, and EPSILON

    // Helper macro to compare two f64 values within EPSILON
    macro_rules! assert_f64_eq {
        ($a:expr, $b:expr) => {
            assert!(($a - $b).abs() < EPSILON, "Expected {}, got {}", $b, $a);
        };
    }

    // Helper function to compare two matrices
    fn assert_matrix_eq(actual: &Matrix, expected: Vec<Vec<f64>>) {
        let (rows, cols) = actual.get_dimensions();
        assert_eq!(rows, expected.len(), "Row count mismatch");
        if rows > 0 {
            assert_eq!(cols, expected[0].len(), "Column count mismatch");
        }

        for r in 0..rows {
            for c in 0..cols {
                assert_f64_eq!(actual.rows[r].columns[c], expected[r][c]);
            }
        }
    }

    #[test]
    fn test_rref_unique_solution() {
        // System:
        // 1x + 2y + 3z = 9
        // 2x - 1y + 1z = 8
        // 3x + 0y - 1z = 3
        // Solution: x=2, y=-1, z=3
        let mut m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 9.0],
            vec![2.0, -1.0, 1.0, 8.0],
            vec![3.0, 0.0, -1.0, 3.0],
        ])
        .unwrap();

        m.reduced_row_echelon();

        let expected = vec![
            vec![1.0, 0.0, 0.0, 2.0],
            vec![0.0, 1.0, 0.0, -1.0],
            vec![0.0, 0.0, 1.0, 3.0],
        ];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_rref_sympy_example() {
        let mut m = Matrix::new(vec![
            vec![1.0, 2.0, 0.0, 5.0, 0.0, -3.0],
            vec![-1.0, -2.0, 1.0, -6.0, 1.0, 2.0],
            vec![-2.0, -4.0, 0.0, -10.0, 1.0, 8.0],
        ])
        .unwrap();

        m.reduced_row_echelon();

        let expected = vec![
            vec![1.0, 2.0, 0.0, 5.0, 0.0, -3.0],
            vec![0.0, 0.0, 1.0, -1.0, 0.0, -3.0],
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0],
        ];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_rref_dependent_rows() {
        // Row 2 is exactly 2x Row 1. Row 3 is 3x Row 1.
        let mut m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 4.0, 6.0],
            vec![3.0, 6.0, 9.0],
        ])
        .unwrap();

        m.reduced_row_echelon();

        let expected = vec![
            vec![1.0, 2.0, 3.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_rref_inconsistent_system() {
        let mut m = Matrix::new(vec![vec![1.0, 1.0, 5.0], vec![1.0, 1.0, 10.0]]).unwrap();

        m.reduced_row_echelon();

        // The bottom row will evaluate to 0 = 1
        let expected = vec![
            vec![1.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0], // 0x + 0y = 1
        ];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_rref_requires_row_swap() {
        let mut m = Matrix::new(vec![
            vec![0.0, 2.0, 4.0], // Cannot use 0.0 as pivot!
            vec![1.0, 1.0, 3.0],
        ])
        .unwrap();

        m.reduced_row_echelon();

        let expected = vec![vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 2.0]];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_rref_identity_matrix() {
        let mut m = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();

        m.reduced_row_echelon();

        let expected = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        assert_matrix_eq(&m, expected);
    }
    #[test]
    fn test_matrix_vector_multiplication() {
        let m = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();

        let v = vec![7.0, 8.0, 9.0];
        let row = Row { columns: v };
        let result = m * row;

        let expected = vec![
            1.0 * 7.0 + 2.0 * 8.0 + 3.0 * 9.0, // 50.0
            4.0 * 7.0 + 5.0 * 8.0 + 6.0 * 9.0, // 122.0
        ];
        let expected_row = Row { columns: expected };

        assert_eq!(result, expected_row);
    }
}
