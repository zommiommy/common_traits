use crate::{False, Integer, IsNonZero, IsSigned, NonZero, SignedInt, Splat};

/// Unsigned integer common operations.
#[allow(clippy::len_without_is_empty)]
pub trait UnsignedInt:
    IsSigned<Signed = False> + IsNonZero<NonZero = False> + Integer + Splat<u8>
{
    /// The signed variant of [`UnsignedInt`].
    type SignedInt: SignedInt<UnsignedInt = Self>;
    /// The non-zero variant of [`UnsignedInt`].
    type NonZeroUnsignedInt: NonZero<BaseType = Self>;

    /// Converts `self` into the signed variant of `Self`.
    fn to_signed(self) -> Self::SignedInt;

    /// Interprets `self` as `rhs` bits and sign-extends it to
    /// [`AsBytes::BITS`](crate::AsBytes::BITS).
    fn sign_extend(self, rhs: u32) -> Self;

    /// Interprets `self` as `rhs` bits and zero-extends it to
    /// [`AsBytes::BITS`](crate::AsBytes::BITS).
    fn zero_extend(self, rhs: u32) -> Self;

    /// Returns the base 2 logarithm of the number, rounded down.
    /// This function panics if `self` is less than or equal to zero.
    fn ilog2(self) -> u32;

    /// Returns the base 2 logarithm of the number, rounded up.
    /// This function panics if `self` is less than or equal to zero.
    fn ilog2_ceil(self) -> u32;

    /// Returns the number of bits that are necessary to represent `self`.
    /// This is one for zero; otherwise, it is equal to
    /// [`ilog2`](`UnsignedInt::ilog2`)`(self) + 1`.
    fn len(self) -> u32;

    /// Computes `(self + rhs - 1)` / rhs, which is equivalent to computing
    /// `((self as f64) / (rhs as f64)).ceil() as Self` but faster and without
    /// loss of precision.
    #[inline(always)]
    fn div_ceil(self, rhs: Self) -> Self {
        (self + rhs - Self::ONE) / rhs
    }

    /// Rounds up `self` so that `self.align_to(rhs) % rhs == 0`.
    /// `rhs` has to be a power of two, otherwise the result is undefined.
    #[inline(always)]
    fn align_to(self, rhs: Self) -> Self {
        self + self.pad_align_to(rhs)
    }

    /// Computes the padding needed for alignment, that is, the smallest
    /// number such that `(value + pad_align_to(value, align_to)) & (align_to - 1) == 0`.
    /// `rhs` has to be a power of two, otherwise the result is undefined.
    #[inline(always)]
    fn pad_align_to(self, rhs: Self) -> Self {
        debug_assert!(rhs.is_power_of_two());
        self.wrapping_neg() & (rhs - Self::ONE)
    }

    /// Checked addition with a signed integer. Computes `self + rhs`, returning
    /// `None` if overflow occurred.
    fn checked_add_signed(self, rhs: Self::SignedInt) -> Option<Self>;
    /// Saturating addition. Computes `self + rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add_signed(self, rhs: Self::SignedInt) -> Self;
    /// Wrapping (modular) addition with a signed integer. Computes `self + rhs`,
    /// wrapping around at the boundary of the type.
    fn wrapping_add_signed(self, rhs: Self::SignedInt) -> Self;

    /// Returns the smallest power of two greater than or equal to `self`.
    /// If the next power of two is greater than the type’s maximum value, `None`
    /// is returned, otherwise the power of two is wrapped in `Some`.
    fn checked_next_power_of_two(self) -> Option<Self>;
    /// Returns `true` if and only if `self == 2^k` for some `k`.
    fn is_power_of_two(self) -> bool;
    /// Returns the smallest power of two greater than or equal to `self`.
    /// When return value overflows (i.e., `self > (1 << (N-1))` for type `uN`), it
    /// panics in debug mode and the return value is wrapped to 0 in release mode
    /// (the only situation in which this method can return 0).
    fn next_power_of_two(self) -> Self;

    /// Shifts `self` right arithmetically by `rhs`, returning the result.
    /// Overshifting by larger than [`AsBytes::BITS`](crate::AsBytes::BITS) will
    /// result in either `!0` or `0`, depending on the sign bit of `self`.
    fn overflow_sar(self, rhs: Self) -> Self;
}
