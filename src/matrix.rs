use crate::constants::EPSILON;
use crate::row::Row;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Mul};
use thiserror::Error;
#[derive(Debug)]
pub struct Matrix {
    rows: Vec<Row>,
}

impl Display for Matrix {
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
    #[error("All columns must be of the same size")]
    InvalidDimensionError,
    #[error("Wtf is this empty matrix?")]
    EmptyMatrixError,
}
enum PivotMoving {
    PivotMoved { row: usize, col: usize },
    REFAchieved,
}
impl Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
impl Matrix {
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
    pub fn get_dimensions(&self) -> (usize, usize) {
        if self.rows.is_empty() {
            return (0, 0);
        }
        (self.rows.len(), self.rows.first().unwrap().columns.len())
    }
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
