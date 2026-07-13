#![cfg_attr(not(feature = "std"), no_std)]
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
pub use integer::Integer;

mod fastrange;
pub use fastrange::FastRange;

mod select_in_word;
pub use select_in_word::SelectInWord;

mod selectors;
pub use selectors::{
    BooleanSelector, False, IsAtomic, IsFloat, IsInteger, IsNonZero, IsSigned, NonZero, True,
};

mod signed_int;
pub use signed_int::SignedInt;

mod atomic;
pub use atomic::{Atomic, IntoAtomic};

mod float;
pub use float::Float;

mod number;
pub use number::FiniteRangeNumber;
pub use number::Number;

mod atomic_float;
#[cfg(feature = "half")]
pub use atomic_float::{AtomicBF16, AtomicF16};
pub use atomic_float::{AtomicF32, AtomicF64, AtomicFloat};

mod atomic_number;
pub use atomic_number::AtomicFiniteRangeNumber;
pub use atomic_number::AtomicNumber;

mod atomic_integer;
pub use atomic_integer::{AtomicInteger, AtomicSignedInt, AtomicUnsignedInt};

mod impls;

mod rnd;
pub use rnd::{Rng, RngNext};

mod same_as;
pub use same_as::SameAs;

mod to;
pub use to::To;

mod splat;
pub use splat::Splat;

mod sequence;
pub use sequence::{Sequence, SequenceGrowable, SequenceMut};

mod hash;
pub use hash::{Hash, Hasher, SeedableHasher};

mod upcastable;
pub use upcastable::{UpcastableFrom, UpcastableInto};

mod downcastable;
pub use downcastable::{DowncastableFrom, DowncastableInto};

mod castable;
pub use castable::{CastableFrom, CastableInto};

/// A trait for types that have a fixed-length representation as a sequence of bytes.
/// This includes all standard numerical scalar types.
///
/// It is required that implementations of `AsRef<[u8]>` and `AsMut<[u8]>`
/// return a slice of length [`AsBytes::BYTES`].
pub trait AsBytes: Sized + Send + Sync + Default {
    /// Length in bytes of the representation of the type.
    const BYTES: usize;
    /// Convenience constant equal to [`AsBytes::BYTES`] * 8.
    const BITS: usize;
    /// The byte array that can be used to build the value. It must always be
    /// `[u8; Self::BYTES]` (but with the present Rust syntax we cannot enforce it).
    type Bytes: AsRef<[u8]> + AsMut<[u8]> + Sized + Send + Sync + Copy + Default;
}

/// Trait for types that can be created safely from an array of bytes.
pub trait FromBytes: AsBytes {
    /// Creates a native endian value from its representation as a byte
    /// array in big endian.
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a native endian value from its representation as a byte
    /// array in little endian.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a native endian value from its memory representation as
    /// a byte array in native endianness.
    /// As the target platform’s native endianness is used, portable code likely
    /// wants to use [`FromBytes::from_be_bytes`] or [`FromBytes::from_le_bytes`],
    /// as appropriate, instead.
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;
}

/// Trait for types that can be cast to an array of bytes.
pub trait ToBytes: AsBytes {
    /// Returns the memory representation of this value as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of this value as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of this value as a byte array in
    /// native byte order.
    /// As the target platform’s native endianness is used, portable code should
    /// use [`ToBytes::to_be_bytes`] or [`ToBytes::to_le_bytes`], as appropriate,
    /// instead.
    fn to_ne_bytes(self) -> Self::Bytes;
}

/// An **`unsafe`** assert macro that checks an invariant in debug mode and
/// optimizes it away in release mode. It has the same syntax as [`assert!`].
/// - In debug mode (when `debug_assertions` are enabled) it checks the condition
///   with [`assert!`].
/// - In release mode it assumes the condition via
///   [`core::hint::unreachable_unchecked`] and generates no check.
///
/// The core difference from [`assert!`] is that the check is elided in release
/// mode, so the compiler assumes the invariant holds.
///
/// # Safety
/// This macro must be invoked inside an `unsafe` block: in release mode a false
/// condition is undefined behavior. The caller must guarantee the condition
/// always holds.
///
/// # Examples
/// You can double check on [compiler explorer](https://godbolt.org/z/G3K31a93o).
/// ```rust
/// use common_traits::invariant;
/// pub fn test1(x: usize) -> u32 {
///     x.ilog2()
/// }
///
/// pub fn test2(x: usize) -> u32 {
///     unsafe { invariant!(x > 0, "x must be positive") };
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
/// Because the macro is unsafe, invoking it outside an `unsafe` block does not
/// compile:
/// ```compile_fail
/// use common_traits::invariant;
/// pub fn bad(x: usize) -> u32 {
///     invariant!(x > 0, "x must be positive");
///     x.ilog2()
/// }
/// ```
#[macro_export]
macro_rules! invariant {
    ($cond:expr $(,)?) => {{
        let cond: bool = $cond;
        #[cfg(debug_assertions)]
        assert!(cond, "invariant failed: {}", stringify!($cond));
        // SAFETY: the caller (see the macro's `# Safety` section) guarantees the
        // condition holds; in debug builds the assertion above has verified it.
        if !cond {
            core::hint::unreachable_unchecked();
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        let cond: bool = $cond;
        #[cfg(debug_assertions)]
        assert!(cond, $($arg)+);
        // SAFETY: the caller (see the macro's `# Safety` section) guarantees the
        // condition holds; in debug builds the assertion above has verified it.
        if !cond {
            core::hint::unreachable_unchecked();
        }
    }};
}

/// An [`assert_eq!`] macro to check invariants in debug mode and to optimize them away in release mode.
/// This has the same syntax as the [`assert_eq!`] macro.
/// See [`invariant!`] for more details.
#[macro_export]
macro_rules! invariant_eq {
    ($left:expr, $right:expr $(,)?) => {
        common_traits::invariant!(($left == $right), )
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        common_traits::invariant!(($left == $right), $($arg)+)
    };
}

/// An [`assert_ne!`] macro to check invariants in debug mode and to optimize them away in release mode.
/// This has the same syntax as the [`assert_ne!`] macro.
/// See [`invariant!`] for more details.
#[macro_export]
macro_rules! invariant_ne {
    ($left:expr, $right:expr $(,)?) => {
        common_traits::invariant!(($left != $right), )
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        common_traits::invariant!(($left != $right), $($arg)+)
    };
}
