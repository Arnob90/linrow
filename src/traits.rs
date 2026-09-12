use crate::impl_real_scalar;
use num_traits::{Float, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub trait HasNorm {
    type Real: Float;
    fn norm(&self) -> Self::Real;
}
pub trait HasConj {
    fn conj(&self) -> Self;
}

/// Trait alias for scalar types usable in `Row` and `Matrix`.
pub trait Scalar:
    Clone
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + HasNorm
    + Neg<Output = Self>
    + Send
    + Sync
    + HasConj
{
    fn norm_diff(&self, other: &Self) -> Self::Real;
}

// Blanket implementation for any type matching the bounds
impl<T> Scalar for T
where
    T: Clone
        + Zero
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + HasNorm
        + Neg<Output = T>
        + Send
        + Sync
        + HasConj,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    fn norm_diff(&self, other: &Self) -> Self::Real {
        (self - other).norm()
    }
}

impl_real_scalar!(f64, f32);
