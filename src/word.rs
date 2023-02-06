use crate::*;

#[cfg(feature = "simd")]
/// Internal trait to allow SIMD opertations on [`Word`] when the feature is enabled
pub trait WSimd: SimdUint {}
#[cfg(not(feature = "simd"))]
/// Internal trait to allow SIMD opertations on [`Word`] when the feature is enabled
pub trait WSimd {}

/// Unsigned word common operations
pub trait Word: Integer + Splat<u8> + WSimd {
    /// The signed variant of the word
    type SignedWord: SignedWord<UnsignedWord=Self>;
    /// The atomically modifiable variant of the word
    type AtomicWord: AtomicWord<NonAtomicWord=Self>;
    /// The non-zero variant of the word
    type NonZeroWord: NonZero<BaseType = Self>;
    
    /// Convert `self` into the signed variant of `Self`
    fn to_signed(self) -> Self::SignedWord;

    /// Convert `self` into the atomic variant of `Self`
    fn to_atomic(self) -> Self::AtomicWord;

    #[cfg(feature="atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self::Atomic]) -> &mut [Self];
    
    #[cfg(feature="atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::Atomic];

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, rhs: Self) -> Self;

    /// Logical shift left `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`Number::BITS`] will result in zero.
    fn overflow_shl(self, rhs: Self) -> Self;

    /// Logical shift right `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`Number::BITS`] will result in zero.
    fn overflow_shr(self, rhs: Self) -> Self;

    /// Arithmetic shift right `self` by `rhs`, returing the result.
    /// Overshifting by larger than [`Number::BITS`] will result in either
    /// `!0` or `0`, depending on the sign bit of `self`.
    fn overflow_sar(self, rhs: Self) -> Self;
    
    /// Interpret `self` as `rhs` bits and sign-extend it to [`Number::BITS`].
    fn sign_extend(self, rhs: u32) -> Self;
    
    /// Interpret `self` as `rhs` bits and zero-extend it to [`Number::BITS`].
    fn zero_extend(self, rhs: u32) -> Self;
    
    /// Checked addition with a signed integer. Computes self + rhs, returning 
    /// None if overflow occurred.
    fn checked_add_signed(self, rhs: Self::SignedWord) -> Option<Self>;
    /// Saturating integer addition. Computes self + rhs, saturating at the 
    /// numeric bounds instead of overflowing.
    fn saturating_add_signed(self, rhs: Self::SignedWord) -> Self;
    /// Wrapping (modular) addition with a signed integer. Computes self + rhs, 
    /// wrapping around at the boundary of the type.
    fn wrapping_add_signed(self, rhs: Self::SignedWord) -> Self;

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
}