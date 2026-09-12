use crate::impl_real_scalar;
use num_traits::{Float, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Defines a magnitude metric for elements of a scalar field.
///
/// This trait abstracts absolute value and norm operations, mapping
/// scalar entries to an underlying real floating-point type for distance
/// metrics, pivoting weight calculations, and noise evaluation.
pub trait HasNorm {
    /// The real floating-point type representing the magnitude/norm.
    type Real: Float;

    /// Computes the norm (or absolute value) of the scalar.
    fn norm(&self) -> Self::Real;
}

/// Defines complex conjugation for scalar field elements.
///
/// For real fields ($\mathbb{R}$), this operation is an identity mapping.
/// For complex fields ($\mathbb{C}$), it negates the imaginary component.
pub trait HasConj {
    /// Returns the complex conjugate of `self`.
    fn conj(&self) -> Self;
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
    + HasNorm
    + Neg<Output = Self>
    + Send
    + Sync
    + HasConj
{
    /// Computes the magnitude of the difference between two scalars $\|a - b\|$.
    ///
    /// Used primarily for numerical stability checks and residual evaluations.
    fn norm_diff(&self, other: &Self) -> Self::Real;
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
        + HasNorm
        + Neg<Output = T>
        + Send
        + Sync
        + HasConj,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    #[inline]
    fn norm_diff(&self, other: &Self) -> Self::Real {
        (self - other).norm()
    }
}

// Instantiate HasNorm and HasConj for standard primitive floating-point types (f64, f32)
impl_real_scalar!(f64, f32);
