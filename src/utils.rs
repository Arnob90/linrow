use crate::constants::EPSILON;
use crate::{
    matrix::{Matrix, MatrixCreationError, transpose},
    row::{DotProductError, Row},
};
pub fn is_almost_eq(val1: f64, val2: f64) -> bool {
    (val1 - val2).abs() < EPSILON
}

pub fn is_orthonormal(
    given_vecs: Vec<Row>,
    inner_product: impl Fn(&Row, &Row) -> Result<f64, DotProductError> + Send + Sync,
) -> Result<bool, MatrixCreationError> {
    let req_matrix = Matrix::from_rows(given_vecs)?;
    let transposed = transpose(&req_matrix);
    let res = Matrix::multiply(&transposed, &req_matrix, inner_product);
    let (row_num, _) = res.get_dimensions();
    for i in 0..row_num {
        for j in 0..row_num {
            if i == j {
                if !is_almost_eq(res[i][j], 1.0) {
                    return Ok(false);
                }
                continue;
            }
            if !is_almost_eq(res[i][j], 0.0) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
