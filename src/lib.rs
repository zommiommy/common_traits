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

mod scalar;
pub use scalar::Scalar;

mod unsigned_int;
pub use unsigned_int::UnsignedInt;

mod integer;
pub use integer::*;

mod fastrange;
pub use fastrange::FastRange;

mod select_in_word;
pub use select_in_word::SelectInWord;

mod signed_int;
pub use signed_int::*;

mod atomic;
pub use atomic::*;

mod float;
pub use float::Float;

mod non_zero;
pub use non_zero::*;

mod number;
pub use number::Number;

mod atomic_float;
pub use atomic_float::*;

mod atomic_integer;
pub use atomic_integer::*;

mod impls;

mod to;
pub use to::*;

mod splat;
pub use splat::Splat;

mod rnd;
pub use rnd::*;

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

/**

Binary selection trait  that make it possible to implement traits differently on disjoint types.

The only two implementing types are [`True`] and [`False`].

This is used to
[circumvent a compiler limitation and implement traits differently
on disjoint types](https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983).

See [`IsAtomic`] for an example.

*/
pub trait BooleanSelector {}
pub struct True {}
impl BooleanSelector for True {}
pub struct False {}
impl BooleanSelector for False {}

/// A trait with an associated [`Boolean`] type specifying whether the type is atomic.
/// It can be used to implement traits differently for atomic and non-atomic types.
/// See the `atomic_data` example.
pub trait IsAtomic {
    type Atomic: BooleanSelector;
}
