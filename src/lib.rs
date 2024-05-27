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
pub use number::FiniteRangeNumber;
pub use number::Number;

mod atomic_float;
pub use atomic_float::*;

mod atomic_number;
pub use atomic_number::AtomicFiniteRangeNumber;
pub use atomic_number::*;

mod atomic_integer;
pub use atomic_integer::*;

mod impls;

mod non_min_max;
pub use non_min_max::*;

mod rnd;
pub use rnd::*;

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

/// A trait for types that have a fixed-length representation as a sequence of bytes.
/// This includes all standard numerical scalar types.
///
/// It is required that implementations of `AsRef<[u8]>` and `AsMut<[u8]>`
/// return a slice of length [`AsBytes::BYTES`].
pub trait AsBytes: Sized + Send + Sync + Default {
    /// Length in bytes of the representation of the type.
    const BYTES: usize;
    /// Convenience costant field equal to [`AsBytes::BYTES`] * 8.
    const BITS: usize;
    /// The byte array that can be use to build the value. It must always be
    /// `[u8; Self::BYTES]` (but with the present Rust syntax we cannot enforce it).
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Sized + Send + Sync + Copy + Default;
}

/// Traits for types that can be created safely from an array of bytes.
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

/// Traits for types that can be cast to an array of bytes.
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

/// An assert macro to check invariants in debug mode and to optimize them away in release mode.
/// This has the same syntax as the [`std::assert`] macro.
/// - On debug mode, i.e. when debug_assertions are enabled, it will call [`std::assert`].
/// - On release mode it will call [`core::hint::unreachable_unchecked`].
///
/// The core difference with [`std::assert`] is that this macro will not have
/// the check in release mode, because the compiler will assume the invariant
/// holds.
///
/// # Example:
/// You can double check on [compiler explorer](https://godbolt.org/z/G3K31a93o).
/// ```rust
/// use common_traits::invariant;
/// pub fn test1(x: usize) -> u32 {
///     x.ilog2()
/// }
///
/// pub fn test2(x: usize) -> u32 {
///     invariant!(x > 0, "x must be positive");
///     x.ilog2()
/// }
/// ```
/// will generate respectively:
/// ```x86asm
/// test    rdi, rdi
/// je      .LBB0_2
/// bsr     rax, rdi
/// ret
/// .LBB0_2:
/// push    rax
/// lea     rdi, [rip + .L__unnamed_1]
/// call    qword ptr [rip + core::num::int_log10::panic_for_nonpositive_argument::h3a8d3f879c6e5198@GOTPCREL]
/// ```
/// and
/// ```x86asm
/// bsr     rax, rdi
/// ret
/// ```
#[macro_export]
macro_rules! invariant {
    ($cond:expr $(,)?) => {
        {
            #[cfg(debug_assertions)]
            {
                assert!($cond);
            }
            #[cfg(not(debug_assertions))]
            {
                if !($cond) {
                    unsafe{
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        {
            #[cfg(debug_assertions)]
            {
                assert!($cond, $($arg)+);
            }
            #[cfg(not(debug_assertions))]
            {
                if !($cond) {
                    unsafe{
                        core::hint::unreachable_unchecked();
                    }
                }
            }
        }
    };
}

/// An assert_eq macro to check invariants in debug mode and to optimize them away in release mode.
/// This has the same syntax as the [`std::assert_eq`] macro.
/// Look at [`invariant!`] for more details.
#[macro_export]
macro_rules! invariant_eq {
    ($left:expr, $right:expr $(,)?) => {
        invariant!(($left == $right), )
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        invariant!(($left == $right), $($arg)+)
    };
}

/// An assert_ne macro to check invariants in debug mode and to optimize them away in release mode.
/// This has the same syntax as the [`std::assert_ne`] macro.
/// Look at [`invariant!`] for more details.
#[macro_export]
macro_rules! invariant_ne {
    ($left:expr, $right:expr $(,)?) => {
        invariant!(($left != $right), )
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        invariant!(($left != $right), $($arg)+)
    };
}
