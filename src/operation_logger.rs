use crate::matrix::Matrix;

///This struct is used to log row operations in form of elementary matrices
pub trait MatrixLogger {
    fn log(&mut self, given_elementary_matrix: Matrix);
}
#[derive(Debug)]
pub struct InvertMatrixLogger {
    result_so_far: Matrix,
}
impl MatrixLogger for InvertMatrixLogger {
    fn log(&mut self, given_elementary_matrix: Matrix) {
        self.result_so_far = given_elementary_matrix * &self.result_so_far;
    }
}
impl InvertMatrixLogger {
    pub fn with_dimensions(dimension: usize) -> InvertMatrixLogger {
        InvertMatrixLogger {
            result_so_far: Matrix::identity_matrix(dimension).unwrap(),
        }
    }
    pub fn inverse_matrix(self) -> Matrix {
        self.result_so_far
    }
}
#[derive(Debug)]
pub struct NoopLogger {}
impl MatrixLogger for NoopLogger {
    #[inline]
    fn log(&mut self, _: Matrix) {}
}
