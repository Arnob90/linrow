use crate::{matrix::Matrix, traits::Scalar};
use num_traits::{One, Zero};

/// This trait is used to log row operations in form of elementary matrices
pub trait MatrixLogger<T = f64> {
    fn log(&mut self, given_elementary_matrix: Matrix<T>);
}

#[derive(Debug)]
pub struct InvertMatrixLogger<T = f64> {
    result_so_far: Matrix<T>,
}

impl<T> MatrixLogger<T> for InvertMatrixLogger<T>
where
    T: Scalar,
{
    fn log(&mut self, given_elementary_matrix: Matrix<T>) {
        self.result_so_far = &given_elementary_matrix * &self.result_so_far;
    }
}

impl<T: Zero + One + Clone> InvertMatrixLogger<T> {
    pub fn with_dimensions(dimension: usize) -> InvertMatrixLogger<T> {
        InvertMatrixLogger {
            result_so_far: Matrix::identity_matrix(dimension).unwrap(),
        }
    }
    pub fn inverse_matrix(self) -> Matrix<T> {
        self.result_so_far
    }
}

#[derive(Debug)]
pub struct NoopLogger {}

impl<T> MatrixLogger<T> for NoopLogger {
    #[inline]
    fn log(&mut self, _: Matrix<T>) {}
}
