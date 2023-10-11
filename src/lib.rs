#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![allow(incomplete_features)]
#![cfg_attr(feature = "simd", feature(generic_const_exprs))]
#![deny(unconditional_recursion)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

mod double_half;
pub use double_half::{DoubleType, HalfType};

mod unsigned_int;
pub use unsigned_int::UnsignedInt;

mod integer;
pub use integer::*;

mod fastrange;
pub use fastrange::FastRange;

mod select_in_word;
pub use select_in_word::SelectInWord;

mod selectors;
pub use selectors::*;

mod signed_int;
pub use signed_int::*;

mod atomic;
pub use atomic::*;

mod float;
pub use float::Float;

mod number;
pub use number::Number;

mod atomic_float;
pub use atomic_float::*;

mod atomic_number;
pub use atomic_number::*;

mod atomic_integer;
pub use atomic_integer::*;

mod impls;

mod to;
pub use to::*;

mod splat;
pub use splat::Splat;

mod sequence;
pub use sequence::*;

mod hash;
pub use hash::*;

mod upcastable;
pub use upcastable::*;

mod downcastable;
pub use downcastable::*;

mod castable;
pub use castable::*;

pub trait AsBytes: Sized + Send + Sync + Default {
    /// Length in bytes of the representation of the type.
    const BYTES: usize;
    /// Convenience costant field equal to [`AsBytes::BYTES`] * 8.
    const BITS: usize;
    /// The byte array that can be use to build the value. It must always be
    /// `[u8; Self::BYTES]` (but with the present Rust syntax we cannot enforce it).
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Sized + Send + Sync + Copy + Default;
}

pub trait FromBytes: AsBytes {
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
}

pub trait ToBytes: AsBytes {
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
}
