#![cfg(all(feature = "complex", feature = "rational"))]

use crate::traits::{HasConj, Metric};
use num_complex::Complex;
use num_rational::{BigRational, Rational32, Rational64};

macro_rules! impl_complex_rational_scalar {
    ($($t:ty),*) => {
        $(
            impl Metric for Complex<$t> {
                #[inline(always)]
                fn is_near(&self, other: &Self) -> bool {
                    self == other
                }
            }

            impl HasConj for Complex<$t> {
                #[inline(always)]
                fn conj(&self) -> Self {
                    Complex::new(self.re.clone(), -self.im.clone())
                }
            }
        )*
    };
}

impl_complex_rational_scalar!(Rational32, Rational64, BigRational);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::def_matrix;
    use crate::matrix::Matrix;
    use crate::operation_logger::NoopLogger;

    #[test]
    fn test_rref_complex_rational_system() {
        let r = |n: i64, d: i64| Rational64::new(n, d);
        let c = |re: Rational64, im: Rational64| Complex::new(re, im);

        // Same complex linear system as test_rref_complex_system, but with exact fractions:
        // [ (1 + i)x + (2 - i)y = 5 + i  ]
        // [    2i x  + (1 + 3i)y = -1 + 5i ]
        let mut m = def_matrix![
            [
                c(r(1, 1), r(1, 1)),
                c(r(2, 1), r(-1, 1)),
                c(r(5, 1), r(1, 1))
            ],
            [
                c(r(0, 1), r(2, 1)),
                c(r(1, 1), r(3, 1)),
                c(r(-1, 1), r(5, 1))
            ]
        ]
        .unwrap();

        // Perform in-place RREF
        m.reduced_row_echelon(&mut NoopLogger {});

        // Exact solution without any floating-point drift:
        // x = 1/4 - 5/4 i  (0.25 - 1.25i)
        // y = 1   + 3/2 i  (1.00 + 1.50i)
        let expected = def_matrix![
            [
                c(r(1, 1), r(0, 1)),
                c(r(0, 1), r(0, 1)),
                c(r(1, 4), r(-5, 4))
            ],
            [
                c(r(0, 1), r(0, 1)),
                c(r(1, 1), r(0, 1)),
                c(r(1, 1), r(3, 2))
            ]
        ]
        .unwrap();

        assert_eq!(m, expected);
    }
}
