use crate::{False, IsFloat, IsInteger, IsNonZero, Number, True};
use core::fmt::{Binary, LowerHex};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// A trait for operations that are shared by signed and unsigned integers.
pub trait Integer:
    Number
    + IsInteger<Integer = True>
    + IsFloat<Float = False>
    + IsNonZero<NonZero = False>
    + LowerHex
    + Ord
    + Eq
    + Binary
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<Output = Self>
    + ShlAssign
    + Shr<Output = Self>
    + ShrAssign
    + Shl<u8, Output = Self>
    + ShlAssign<u8>
    + Shr<u8, Output = Self>
    + ShrAssign<u8>
    + Shl<u16, Output = Self>
    + ShlAssign<u16>
    + Shr<u16, Output = Self>
    + ShrAssign<u16>
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + Shl<u64, Output = Self>
    + ShlAssign<u64>
    + Shr<u64, Output = Self>
    + ShrAssign<u64>
    + Shl<u128, Output = Self>
    + ShlAssign<u128>
    + Shr<u128, Output = Self>
    + ShrAssign<u128>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Shl<i8, Output = Self>
    + ShlAssign<i8>
    + Shr<i8, Output = Self>
    + ShrAssign<i8>
    + Shl<i16, Output = Self>
    + ShlAssign<i16>
    + Shr<i16, Output = Self>
    + ShrAssign<i16>
    + Shl<i32, Output = Self>
    + ShlAssign<i32>
    + Shr<i32, Output = Self>
    + ShrAssign<i32>
    + Shl<i64, Output = Self>
    + ShlAssign<i64>
    + Shr<i64, Output = Self>
    + ShrAssign<i64>
    + Shl<i128, Output = Self>
    + ShlAssign<i128>
    + Shr<i128, Output = Self>
    + ShrAssign<i128>
    + Shl<isize, Output = Self>
    + ShlAssign<isize>
    + Shr<isize, Output = Self>
    + ShrAssign<isize>
{
    /// Get the i-th bit in the UnsignedInt. Valid values: [0, Self::BITS)
    fn extract_bit(&self, bit: usize) -> bool;

    /// Get the bits in range [START; END_BIT) in the UnsignedInt.
    /// START valid values: [0, Self::BITS)
    /// END valid values: [1, Self::BITS]
    /// START < END!!!
    fn extract_bitfield(&self, start_bit: usize, end_bit: usize) -> Self;

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, rhs: Self) -> Self;

    /// Performs Euclidean division.
    /// Since, for the positive integers, all common definitions of division are
    /// equal, this is exactly equal to self / rhs.
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least remainder of self (mod rhs).
    /// Since, for the positive integers, all common definitions of division are
    /// equal, this is exactly equal to self % rhs.
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Converts an integer from big endian to the target’s endianness.
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(rhs: Self) -> Self;

    /// Converts an integer from little endian to the target’s endianness.
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(rhs: Self) -> Self;

    /// Converts self to big endian from the target’s endianness.
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;

    /// Converts self to little endian from the target’s endianness.
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;

    /// Reverse the byte order of the integer
    fn swap_bytes(self) -> Self;

    /// Checked integer addition. Computes self + rhs, returning None if
    /// overflow occurred.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Checked integer division. Computes self / rhs, returning None
    /// if rhs == 0.
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean division. Computes self.div_euclid(rhs), returning
    /// None if rhs == 0.
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked integer multiplication. Computes self * rhs, returning None if
    /// overflow occurred.
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Checked negation. Computes -self, returning None unless self == 0.
    /// Note that negating any positive integer will overflow.
    fn checked_neg(self) -> Option<Self>;

    /// Checked exponentiation. Computes self.pow(exp), returning None if
    /// overflow occurred.
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// Checked integer remainder. Computes self % rhs, returning None
    /// if rhs == 0.
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean modulo. Computes self.rem_euclid(rhs), returning None
    /// if rhs == 0.
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked shift left. Computes self << rhs, returning None if rhs is
    /// larger than or equal to the Integer of bits in self.
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    /// Checked shift right. Computes self >> rhs, returning None if rhs is
    /// larger than or equal to the Integer of bits in self.
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    /// Checked integer subtraction. Computes self - rhs, returning None if
    /// overflow occurred.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Returns the Integer of ones in the binary representation of self.
    fn count_ones(self) -> u32;

    /// Returns the Integer of zeros in the binary representation of self.
    fn count_zeros(self) -> u32;

    /// Returns the Integer of leading ones in the binary representation of self.
    fn leading_ones(self) -> u32;
    /// Returns the Integer of trailing zeros in the binary representation of self.
    fn leading_zeros(self) -> u32;

    /// Reverses the order of bits in the integer. The least significant bit
    /// becomes the most significant bit, second least-significant bit becomes
    /// second most-significant bit, etc.
    fn reverse_bits(self) -> Self;

    /// Shifts the bits to the left by a specified amount, n, wrapping the t
    /// runcated bits to the end of the resulting integer.
    /// Please note this isn’t the same operation as the << shifting operator!
    fn rotate_left(self, exp: u32) -> Self;

    /// Shifts the bits to the right by a specified amount, n, wrapping the
    /// truncated bits to the beginning of the resulting integer.
    /// Please note this isn’t the same operation as the >> shifting operator!
    fn rotate_right(self, exp: u32) -> Self;

    /// Returns the Integer of trailing ones in the binary representation of self.
    fn trailing_ones(self) -> u32;

    /// Returns the Integer of trailing zeros in the binary representation of self.
    fn trailing_zeros(self) -> u32;

    /// Logical shift left `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`AsBytes::BITS`](`crate::AsBytes::BITS`) will result in zero.
    fn overflow_shl(self, rhs: Self) -> Self;

    /// Logical shift right `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`AsBytes::BITS`](`crate::AsBytes::BITS`) will result in zero.
    fn overflow_shr(self, rhs: Self) -> Self;

    /// Add `self` and `rhs`, returning the result using wrapping arithmetic
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on
    /// unsigned types is just normal division. There’s no way wrapping could
    /// ever happen. This function exists, so that all operations are accounted
    /// for in the wrapping operations.
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Wrapping Euclidean division. Computes self.div_euclid(rhs). Wrapped
    /// division on unsigned types is just normal division. There’s no way
    /// wrapping could ever happen. This function exists, so that all operations
    /// are accounted for in the wrapping operations. Since, for the positive
    /// integers, all common definitions of division are equal, this is exactly
    /// equal to self.wrapping_div(rhs).
    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around
    /// at the boundary of the type.
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Wrapping (modular) negation. Computes -self, wrapping around at the
    /// boundary of the type.
    /// Since unsigned types do not have negative equivalents all applications
    /// of this function will wrap (except for -0). For values smaller than the
    /// corresponding signed type’s maximum the result is the same as casting
    /// the corresponding signed value. Any larger values are equivalent to
    /// MAX + 1 - (val - MAX - 1) where MAX is the corresponding signed type’s
    /// maximum.
    fn wrapping_neg(self) -> Self;

    /// Wrapping (modular) exponentiation. Computes self.pow(exp), wrapping
    /// around at the boundary of the type.
    fn wrapping_pow(self, exp: u32) -> Self;

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so
    /// that all operations are accounted for in the wrapping operations.
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Wrapping Euclidean modulo. Computes self.rem_euclid(rhs). Wrapped modulo
    /// calculation on unsigned types is just the regular remainder calculation.
    /// There’s no way wrapping could ever happen. This function exists, so that
    /// all operations are accounted for in the wrapping operations. Since, for
    /// the positive integers, all common definitions of division are equal,
    /// this is exactly equal to self.wrapping_rem(rhs).
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask
    /// removes any high-order bits of rhs that would cause the shift to exceed
    /// the bitwidth of the type.
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping
    /// shift-left is restricted to the range of the type, rather than the bits
    /// shifted out of the LHS being returned to the other end. The primitive
    /// integer types all implement a rotate_left function, which may be what
    /// you want instead.
    fn wrapping_shl(self, rhs: u32) -> Self;

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask
    /// removes any high-order bits of rhs that would cause the shift to exceed
    /// the bitwidth of the type.
    /// Note that this is not the same as a rotate-right; the RHS of a wrapping
    /// shift-right is restricted to the range of the type, rather than the bits
    /// shifted out of the LHS being returned to the other end. The primitive
    /// integer types all implement a rotate_right function, which may be what
    /// you want instead.
    fn wrapping_shr(self, rhs: u32) -> Self;

    /// Subtract `self` and `rhs`, returning the result using wrapping
    /// arithmetic
    fn wrapping_sub(self, rhs: Self) -> Self;
}
