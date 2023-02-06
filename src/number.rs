
use core::fmt::{Debug, Display}; 
use core::ops::*;

pub trait Number:
    Sized + Send + Sync + Display +
    Default + Debug + Clone + Copy +
    PartialOrd + PartialEq +
    Add<Output=Self> + AddAssign<Self> +
    Div<Output=Self> + DivAssign<Self> +
    Mul<Output=Self> + MulAssign<Self> + 
    Rem<Output=Self> + RemAssign<Self> +
    Sub<Output=Self> + SubAssign<Self> + 
{
    /// Number of bits in the word
    const BITS: usize;
    /// Number of bytes in the word
    const BYTES: usize;
    /// The byte array form of the value = `[u8; Self::BYTES]`
    type BytesForm;
    /// Zero represented by `Self`
    const ZERO: Self;
    /// One represented by `Self`
    const ONE: Self;
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Create a native endian integer value from its representation as a byte 
    /// array in big endian.
    fn from_be_bytes(bytes: Self::BytesForm) -> Self;

    /// Create a native endian integer value from its representation as a byte 
    /// array in little endian.
    fn from_le_bytes(bytes: Self::BytesForm) -> Self;

    /// Create a native endian integer value from its memory representation as 
    /// a byte array in native endianness.
    /// As the target platform’s native endianness is used, portable code likely 
    /// wants to use from_be_bytes or from_le_bytes, as appropriate instead.
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self;

    /// Return the memory representation of this integer as a byte array in 
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::BytesForm;

    /// Return the memory representation of this integer as a byte array in 
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::BytesForm;

    /// Return the memory representation of this integer as a byte array in 
    /// native byte order.
    /// As the target platform’s native endianness is used, portable code should
    /// use to_be_bytes or to_le_bytes, as appropriate, instead.
    fn to_ne_bytes(self) -> Self::BytesForm;
}