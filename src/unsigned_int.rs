use crate::*;
use crate::{IsSigned, False};


/// Unsigned UnsignedInt common operations
pub trait UnsignedInt: IsSigned<Signed=False> + IsNonZero<NonZero=False> + Integer + Splat<u8> {
    /// The signed variant of the UnsignedInt
    type SignedInt: SignedInt<UnsignedInt = Self>;
    /// The non-zero variant of the UnsignedInt
    type NonZeroUnsignedInt: NonZero<BaseType = Self>;

    /// Convert `self` into the signed variant of `Self`
    fn to_signed(self) -> Self::SignedInt;

    /// Interpret `self` as `rhs` bits and sign-extend it to [`Bits::BITS`].
    fn sign_extend(self, rhs: u32) -> Self;

    /// Interpret `self` as `rhs` bits and zero-extend it to [`Bits::BITS`].
    fn zero_extend(self, rhs: u32) -> Self;

    /// Return the base 2 logarithm of the number, rounded down.
    /// This function panic if `self` is less than or equal to zero.
    fn ilog2(self) -> Self;

    /// Return the base 2 logarithm of the number, rounded up.
    /// This function panic if `self` is less than or equal to zero.
    #[inline(always)]
    fn ilog2_ceil(self) -> Self {
        let two = Self::ONE + Self::ONE;
        if self <= two {
            self
        } else {
            (self - Self::ONE).ilog2() + Self::ONE
        }
    }

    /// Checked addition with a signed integer. Computes self + rhs, returning
    /// None if overflow occurred.
    fn checked_add_signed(self, rhs: Self::SignedInt) -> Option<Self>;
    /// Saturating integer addition. Computes self + rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add_signed(self, rhs: Self::SignedInt) -> Self;
    /// Wrapping (modular) addition with a signed integer. Computes self + rhs,
    /// wrapping around at the boundary of the type.
    fn wrapping_add_signed(self, rhs: Self::SignedInt) -> Self;

    /// Returns the smallest power of two greater than or equal to n.
    /// If the next power of two is greater than the type’s maximum value, None
    /// is returned, otherwise the power of two is wrapped in Some.
    fn checked_next_power_of_two(self) -> Option<Self>;
    /// Returns true if and only if self == 2^k for some k.
    fn is_power_of_two(self) -> bool;
    /// Returns the smallest power of two greater than or equal to self.
    /// When return value overflows (i.e., self > (1 << (N-1)) for type uN), it
    /// panics in debug mode and the return value is wrapped to 0 in release mode
    /// (the only situation in which method can return 0).
    fn next_power_of_two(self) -> Self;
    
    /// Arithmetic shift right `self` by `rhs`, returing the result.
    /// Overshifting by larger than [`Bits::BITS`] will result in either
    /// `!0` or `0`, depending on the sign bit of `self`.
    fn overflow_sar(self, rhs: Self) -> Self;
}
