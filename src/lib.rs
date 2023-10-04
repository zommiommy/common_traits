#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![allow(incomplete_features)]
#![cfg_attr(feature = "simd", feature(generic_const_exprs))]
#![deny(unconditional_recursion)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

mod bits;
pub use bits::Bits;

mod word;
pub use word::Word;

mod integer;
pub use integer::Integer;

mod fastrange;
pub use fastrange::FastRange;

mod select_in_word;
pub use select_in_word::SelectInWord;

mod signed_word;
pub use signed_word::SignedWord;

mod atomic;
pub use atomic::*;

mod float;
pub use float::Float;

mod non_zero;
pub use non_zero::NonZero;

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

mod memory_footprint;
pub use memory_footprint::*;

mod sequence;
pub use sequence::*;

mod upcastable;
pub use upcastable::*;

mod downcastable;
pub use downcastable::*;

mod castable;
pub use castable::*;
