use crate::{False, IsAtomic};
use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// A trait for operations that are shared by integers and floats.
pub trait Number:
    IsAtomic<Atomic = False>
    + Copy
    + Display
    + Default
    + Debug
    + PartialOrd
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
{
    /// Zero represented by `Self`.
    const ZERO: Self;
    /// One represented by `Self`.
    const ONE: Self;
    /// Computes `(self * a) + b`.
    ///
    /// With the `std` feature this is a *fused* multiply-add: the product
    /// `self * a` is computed at full precision and rounded only once (using the
    /// platform FMA instruction where available), which is more accurate than an
    /// unfused multiply-add. Without `std` there is no fused primitive, so this
    /// falls back to a separate multiply and add (two roundings).
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Panics
    /// Integer implementations panic if `exp` does not fit in a `u32` (in
    /// particular, for any negative exponent).
    #[cfg(feature = "std")]
    fn pow(self, exp: Self) -> Self;

    /// Returns the maximum of the two numbers, ignoring NaN on floats.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    /// This follows the IEEE 754-2008 semantics for maxNum, except for handling
    /// of signaling NaNs; this function handles all NaNs the same way and
    /// avoids maxNum’s problems with associativity. This also matches the
    /// behavior of libm’s fmax.
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers, ignoring NaN on floats.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    /// This follows the IEEE 754-2008 semantics for minNum, except for handling
    /// of signaling NaNs; this function handles all NaNs the same way and
    /// avoids minNum’s problems with associativity. This also matches the
    /// behavior of libm’s fmin.
    fn min(self, other: Self) -> Self;

    /// Restricts a value to a certain interval unless it is NaN on floats.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is less than `min`. Otherwise this returns `self`.
    ///
    /// Note that this function returns NaN if the initial value was NaN as well.
    ///
    /// # Panics
    /// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// A number that has a [`MIN`](`FiniteRangeNumber::MIN`) and a [`MAX`](`FiniteRangeNumber::MAX`).
pub trait FiniteRangeNumber: Number {
    /// Minimum value represented by `Self`.
    const MIN: Self;
    /// Maximum value represented by `Self`.
    const MAX: Self;

    /// Saturating addition. Computes `self + rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating division. Computes `self / rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating multiplication. Computes `self * rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating exponentiation. Computes `self.pow(rhs)`, saturating
    /// at the numeric bounds instead of overflowing.
    ///
    /// # Panics
    /// Integer implementations panic if `rhs` does not fit in a `u32` (in
    /// particular, for any negative exponent).
    #[cfg(feature = "std")]
    fn saturating_pow(self, rhs: Self) -> Self;

    /// Saturating subtraction. Computes `self - rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;
}
