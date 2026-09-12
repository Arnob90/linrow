#![cfg(feature = "complex")]
use crate::impl_complex_scalar;
use crate::traits::{HasConj, HasNorm};
use num_complex::ComplexFloat;
impl_complex_scalar!(f32, f64);
