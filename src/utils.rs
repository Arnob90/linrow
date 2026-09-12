use num_traits::Float;

use crate::constants::EPSILON;
use crate::matrix::conjugate_matrix;
use crate::traits::Scalar;
use crate::{
    matrix::{Matrix, MatrixCreationError, row_to_col_matrix},
    row::{DotProductError, Row},
};

pub fn get_generic_eps<T: Float>() -> T {
    num_traits::cast::<f64, T>(EPSILON).unwrap_or_else(T::zero)
}
pub fn is_almost_eq<T: Scalar>(val1: &T, val2: &T) -> bool {
    val1.norm_diff(val2) < get_generic_eps()
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

pub fn is_orthonormal<T: Scalar>(
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
                if !is_almost_eq(&res[i][j], &T::one()) {
                    return Ok(false);
                }
                continue;
            }
            if !is_almost_eq(&res[i][j], &T::zero()) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
