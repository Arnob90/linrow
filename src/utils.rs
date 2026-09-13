use num_traits::Float;

use crate::constants::EPSILON;
use crate::matrix::conjugate_matrix;
use crate::traits::{Metric, Scalar};
use crate::{
    matrix::{Matrix, MatrixCreationError, row_to_col_matrix},
    row::{DotProductError, Row},
};

pub fn get_generic_eps<T: Float>() -> T {
    num_traits::cast::<f64, T>(EPSILON).unwrap_or_else(T::zero)
}
pub fn dot_product<T: Scalar>(vec1: &Row<T>, vec2: &Row<T>) -> Result<T, DotProductError> {
    if vec1.row_elems.len() != vec2.row_elems.len() {
        return Err(DotProductError::DimensionMismatch);
    }
    Ok(vec1
        .row_elems
        .iter()
        .zip(&vec2.row_elems)
        .fold(T::zero(), |acc, (e1, e2)| acc + (e1.clone() * e2.conj())))
}
pub fn conjugate_transpose<T: Scalar>(given_matrix: &Matrix<T>) -> Matrix<T> {
    let mut transposed = row_to_col_matrix(given_matrix);
    conjugate_matrix(&mut transposed);
    transposed
}

pub fn is_orthonormal<T: Scalar + Metric>(
    given_vecs: Vec<Row<T>>,
    inner_product: impl Fn(&Row<T>, &Row<T>) -> Result<T, DotProductError> + Send + Sync,
) -> Result<bool, MatrixCreationError> {
    let req_matrix = Matrix::from_rows(given_vecs)?;
    let transposed = row_to_col_matrix(&req_matrix);
    let res = Matrix::multiply(&transposed, &req_matrix, inner_product);
    let (row_num, _) = res.get_dimensions();
    for i in 0..row_num {
        for j in 0..row_num {
            if i == j {
                if !res[i][j].is_near(&T::one()) {
                    return Ok(false);
                }
                continue;
            }
            if !res[i][j].is_near(&T::zero()) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
