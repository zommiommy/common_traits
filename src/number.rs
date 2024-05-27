use crate::{False, IsAtomic};
use core::fmt::{Debug, Display};
use core::ops::*;

/// A trait for operations that are shared by integers and floats.
pub trait Number:
    IsAtomic<Atomic = False>
    + Copy
    + Clone
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
    /// Zero represented by `Self`
    const ZERO: Self;
    /// One represented by `Self`
    const ONE: Self;
    /// Fused multiply-add. Computes (self * a) + b with only one rounding error,
    /// yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using mul_add may be more performant than an unfused multiply-add if the
    /// target architecture has a dedicated fma CPU instruction. However, this
    /// is not always true, and will be heavily dependant on designing
    /// algorithms with specific target hardware in mind.
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Raises self to the power of exp, using exponentiation by squaring.
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

    /// Restrict a value to a certain interval unless it is NaN on floats.
    ///
    /// Returns max if self is greater than max, and min if self is less than min. Otherwise this returns self.
    ///
    /// Note that this function returns NaN if the initial value was NaN as well.
    ///
    /// # Panics
    /// Panics if min > max, min is NaN, or max is NaN.
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// A number that has a Max and a Min.
pub trait FiniteRangeNumber: Number {
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Saturating integer addition. Computes self + rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating integer division. Computes self / rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating integer multiplication. Computes self * rhs, saturating at
    /// the numeric bounds instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating integer exponentiation. Computes self.pow(exp), saturating
    /// at the numeric bounds instead of overflowing.
    #[cfg(feature = "std")]
    fn saturating_pow(self, rhs: Self) -> Self;

    /// Saturating integer subtraction. Computes self - rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;
}
