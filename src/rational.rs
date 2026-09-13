#![cfg(feature = "rational")]
use crate::traits::{HasConj, Metric};
use num_rational::{self};
macro_rules! impl_real_num_traits {
    ($($t:ty),*) => {
        $(
            impl Metric for $t {
                #[inline(always)]
                fn is_near(&self, other: &Self) -> bool {
                    self==other
                }
            }

            impl HasConj for $t {
                fn conj(&self) -> Self {
                    self.clone()
                }
            }
        )*
    };
}

// Usage:
impl_real_num_traits!(
    num_rational::Rational32,
    num_rational::Rational64,
    num_rational::BigRational
);
#[cfg(test)]
mod tests {
    use crate::def_matrix;
    use crate::matrix::Matrix;
    use crate::operation_logger::NoopLogger;
    use num_rational::Rational64;
    #[test]
    #[cfg(feature = "rational")]
    fn test_rref_rational_system() {
        let r = |n: i64, d: i64| Rational64::new(n, d);
        let i = |n: i64| r(n, 1);
        let mut m = def_matrix![[r(1, 2), r(1, 3), r(5, 6)], [r(2, 1), r(-1, 2), r(1, 4)]].unwrap();

        m.reduced_row_echelon(&mut NoopLogger {});
        let expected = def_matrix![[i(1), i(0), r(6, 11)], [i(0), i(1), r(37, 22)]].unwrap();

        assert_eq!(m, expected);
    }
}
