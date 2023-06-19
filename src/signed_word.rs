use crate::{Integer, NonZero, Word};
use core::ops::*;

/// Signed word common operations
pub trait SignedWord: Neg<Output = Self> + Integer {
    type UnsignedWord: Word<SignedWord = Self>;
    /// The non-zero variant of the word
    type NonZeroWord: NonZero<BaseType = Self>;

    /// Convert `self` into the unsigned variant of `Self`
    fn to_unsigned(self) -> Self::UnsignedWord;

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

    /// Checked subtraction with an unsigned integer. Computes self - rhs,
    /// returning None if overflow occurred.
    fn checked_sub_unsigned(self, rhs: Self::UnsignedWord) -> Option<Self>;

    /// Saturating addition with an unsigned integer. Computes self + rhs,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_add_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Saturating subtraction with an unsigned integer. Computes self - rhs,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Wrapping (modular) addition with an unsigned integer. Computes
    /// self + rhs, wrapping around at the boundary of the type.
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Wrapping (modular) subtraction with an unsigned integer. Computes
    /// self - rhs, wrapping around at the boundary of the type.
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, rhs: Self) -> Self::UnsignedWord;
}
