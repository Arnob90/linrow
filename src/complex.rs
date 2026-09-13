#![cfg(feature = "complex")]
use crate::traits::{HasConj, Metric};
use num_complex::ComplexFloat;
// Optional macro for num-complex when the feature flag is enabled
#[cfg(feature = "complex")]
macro_rules! impl_complex_scalar {
    ($($t:ty),*) => {
        $(
            impl Metric for num_complex::Complex<$t> {
                #[inline(always)]
                fn is_near(&self, other: &Self) -> bool {
                    ComplexFloat::abs(*self - *other)
                        <= crate::utils::get_generic_eps()
                }
            }

            impl HasConj for num_complex::Complex<$t> {
                #[inline(always)]
                fn conj(&self) -> Self {
                    ComplexFloat::conj(*self)
                }
            }
        )*
    };
}
impl_complex_scalar!(f32, f64);
#[cfg(test)]
mod tests {
    use crate::def_matrix;
    use crate::matrix::Matrix;
    use crate::operation_logger::NoopLogger; // Import your Matrix, Row, and EPSILON

    // Helper macro to compare two f64 values within EPSILON
    #[test]
    #[cfg(feature = "complex")]
    fn test_rref_complex_system() {
        use num_complex::Complex;

        // Helper closure to cleanly construct double-precision complex numbers
        let c = |re: f64, im: f64| Complex::new(re, im);

        // Augmented matrix [A | b] representing the complex linear system
        let mut m = def_matrix![
            [c(1.0, 1.0), c(2.0, -1.0), c(5.0, 1.0)],
            [c(0.0, 2.0), c(1.0, 3.0), c(-1.0, 5.0)]
        ]
        .unwrap();

        // Perform in-place RREF reduction without progress logging
        m.reduced_row_echelon(&mut NoopLogger {});

        // Expected identity-augmented form [I | x] corresponding to:
        // x = 0.25 - 1.25i
        // y = 1.00 + 1.50i
        let expected = def_matrix![
            [c(1.0, 0.0), c(0.0, 0.0), c(0.25, -1.25)],
            [c(0.0, 0.0), c(1.0, 0.0), c(1.0, 1.5)]
        ]
        .unwrap();

        // Assert exact equality between the reduced matrix and expected RREF solution
        assert_eq!(m, expected);
    }
}
