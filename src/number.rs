use core::fmt::{Debug, Display};
use core::ops::*;

/// Trait containing common properties of both integers and floats
pub trait Number:
    Sized
    + Send
    + Sync
    + Display
    + Default
    + Debug
    + Clone
    + Copy
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
    /// Number of bits in the word
    const BITS: usize;
    /// Number of bytes in the word
    const BYTES: usize;
    /// The byte array that can be use to build the value. It will always be
    ///  `[u8; Self::BYTES]`
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Copy + Default;
    /// Zero represented by `Self`
    const ZERO: Self;
    /// One represented by `Self`
    const ONE: Self;
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Fused multiply-add. Computes (self * a) + b with only one rounding error,
    /// yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using mul_add may be more performant than an unfused multiply-add if the
    /// target architecture has a dedicated fma CPU instruction. However, this
    /// is not always true, and will be heavily dependant on designing
    /// algorithms with specific target hardware in mind.
    fn mul_add(self, a: Self, b: Self) -> Self;

    #[cfg(feature = "std")]
    /// Raises self to the power of exp, using exponentiation by squaring.
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

    /// Create a native endian integer value from its representation as a byte
    /// array in big endian.
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Create a native endian integer value from its representation as a byte
    /// array in little endian.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;

    /// Create a native endian integer value from its memory representation as
    /// a byte array in native endianness.
    /// As the target platform’s native endianness is used, portable code likely
    /// wants to use from_be_bytes or from_le_bytes, as appropriate instead.
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;

    /// Return the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::Bytes;

    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::Bytes;

    /// Return the memory representation of this integer as a byte array in
    /// native byte order.
    /// As the target platform’s native endianness is used, portable code should
    /// use to_be_bytes or to_le_bytes, as appropriate, instead.
    fn to_ne_bytes(self) -> Self::Bytes;

    /// Saturating integer addition. Computes self + rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating integer division. Computes self / rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating integer multiplication. Computes self * rhs, saturating at
    /// the numeric bounds instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    #[cfg(feature = "std")]
    /// Saturating integer exponentiation. Computes self.pow(exp), saturating
    /// at the numeric bounds instead of overflowing.
    fn saturating_pow(self, rhs: Self) -> Self;

    /// Saturating integer subtraction. Computes self - rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;
}
