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

/// Trait for types that can be used as a boolean.
///
/// This is used to
/// [implement traits for disjoint types](https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983).

pub trait Boolean {}
pub struct True {}
impl Boolean for True {}
pub struct False {}
impl Boolean for False {}
