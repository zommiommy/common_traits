use crate::{False, IsNonZero, IsSigned, True};
use crate::{Integer, NonZero, UnsignedInt};
use core::ops::*;

/// Signed UnsignedInt common operations
pub trait SignedInt:
    IsSigned<Signed = True> + IsNonZero<NonZero = False> + Neg<Output = Self> + Integer
{
    type UnsignedInt: UnsignedInt<SignedInt = Self>;
    /// The non-zero variant of the UnsignedInt
    type NonZeroUnsignedInt: NonZero<BaseType = Self>;

    /// Convert `self` into the unsigned variant of `Self`
    fn to_unsigned(self) -> Self::UnsignedInt;

    /// Computes the absolute value of self.
    /// # Overflow behavior
    /// The absolute value of Self::MIN cannot be represented as an Self, and a
    /// ttempting to calculate it will cause an overflow. This means that code
    /// in debug mode will trigger a panic on this case and optimized code will
    /// return Self::MIN without a panic.
    fn abs(self) -> Self;

    /// Checked absolute value. Computes self.abs(), returning None if
    /// self == MIN.
    fn checked_abs(self) -> Option<Self>;

    /// Checked negation. Computes -self, returning None if self == MIN.
    fn checked_neg(self) -> Option<Self>;

    /// Return a number representing the sign of `self`, i.e.
    /// * `0` if the number is zero
    /// * `1` if the number is positive
    /// * `-1` if the number is negative
    fn signum(self) -> Self;

    /// Checked subtraction with an unsigned integer. Computes self - rhs,
    /// returning None if overflow occurred.
    fn checked_sub_unsigned(self, rhs: Self::UnsignedInt) -> Option<Self>;

    /// Saturating addition with an unsigned integer. Computes self + rhs,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_add_unsigned(self, rhs: Self::UnsignedInt) -> Self;

    /// Saturating subtraction with an unsigned integer. Computes self - rhs,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedInt) -> Self;

    /// Wrapping (modular) addition with an unsigned integer. Computes
    /// self + rhs, wrapping around at the boundary of the type.
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedInt) -> Self;

    /// Wrapping (modular) subtraction with an unsigned integer. Computes
    /// self - rhs, wrapping around at the boundary of the type.
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedInt) -> Self;
}
