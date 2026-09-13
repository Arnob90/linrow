use crate::impl_real_scalar;
use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Trait representing an element that can be checked for pivot eligibility in Gaussian elimination.
///
/// Types with approximate floating point behavior (like `f64`, `f32`) should check against an epsilon,
/// while exact types (like integers or rational numbers) can check for exact non-zero equality.
pub trait PivotElement {
    fn is_pivot(&self) -> bool;
}

/// Defines complex conjugation for scalar field elements.
///
/// For real fields ($\mathbb{R}$), this operation is an identity mapping.
/// For complex fields ($\mathbb{C}$), it negates the imaginary component.
pub trait HasConj {
    /// Returns the complex conjugate of `self`.
    fn conj(&self) -> Self;
}

pub trait Metric {
    fn is_near(&self, other: &Self) -> bool;
}

// Default 1: Anything with a norm/magnitude (f32, f64, Complex)
impl<T: Metric + Zero> PivotElement for T {
    #[inline]
    fn is_pivot(&self) -> bool {
        !self.is_near(&T::zero())
    }
}

/// Trait alias for scalar types usable in `Row` and `Matrix`.
///
/// Combines basic algebraic field operations (ring arithmetic, division, negation),
/// identity elements (`Zero`, `One`), thread-safety guarantees (`Send`, `Sync`),
/// and metric abstractions (`HasNorm`, `HasConj`).
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
    + Neg<Output = Self>
    + Send
    + Sync
    + PivotElement
    + HasConj
{
}

// Blanket implementation for any type matching the full suite of scalar bounds.
// This ensures any custom type implementing the base arithmetic traits automatically
// satisfies `Scalar` without requiring manual boilerplate.
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
        + Neg<Output = T>
        + Send
        + Metric
        + Sync
        + HasConj,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
}

// Instantiate HasNorm and HasConj for standard primitive floating-point types (f64, f32)
impl_real_scalar!(f64, f32);
