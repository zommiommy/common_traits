use core::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

use crate::{AtomicF32, AtomicF64, IntoAtomic};

/// Unsafe marker type for types whose atomic version has the same memory layout
/// and semantics.
///
/// This marker is used by the trait [`Atomic`](crate::Atomic) to guarantee that
/// non-atomic and associated atomic types have the same memory layout and
/// semantics.
///
/// It is implemented for all primitive types and for the types of the
/// [`half`](https://crates.io/crates/half) crate if the corresponding gate
/// feature is enabled.
///
/// # Safety
///
/// The implementor must ensure that `T` has the same memory layout and
/// semantics as the associated atomic type [`IntoAtomic::AtomicType`].
pub unsafe trait SameAs<T>: IntoAtomic<AtomicType = T> {}

unsafe impl SameAs<AtomicU8> for u8 {}
unsafe impl SameAs<AtomicU16> for u16 {}
unsafe impl SameAs<AtomicU32> for u32 {}
unsafe impl SameAs<AtomicU64> for u64 {}
unsafe impl SameAs<AtomicUsize> for usize {}

unsafe impl SameAs<AtomicI8> for i8 {}
unsafe impl SameAs<AtomicI16> for i16 {}
unsafe impl SameAs<AtomicI32> for i32 {}
unsafe impl SameAs<AtomicI64> for i64 {}
unsafe impl SameAs<AtomicIsize> for isize {}

unsafe impl SameAs<AtomicBool> for bool {}

unsafe impl SameAs<AtomicF32> for f32 {}
unsafe impl SameAs<AtomicF64> for f64 {}

#[cfg(feature = "half")]
mod half_same_as {
    use crate::{AtomicBF16, AtomicF16};

    use super::*;
    use half::{bf16, f16};

    unsafe impl SameAs<AtomicF16> for f16 {}
    unsafe impl SameAs<AtomicBF16> for bf16 {}
}
