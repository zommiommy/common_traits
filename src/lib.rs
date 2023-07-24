#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![allow(incomplete_features)]
#![cfg_attr(feature = "simd", feature(generic_const_exprs))]
#![deny(unconditional_recursion)]
//! This is a collection of traits and dependancies I often use to write generic
//! code for both data-structures and scientific computing.
//!
//! Everything is experimental and I'll change them to my needs :)
//!
//! The point of making this crate public is to be able to discuss this
//! as it covers many core missings from Rust.
//!
//! The crate contains the following traits:
//! - [`Number`] to abstract over all the numerical traits.
//! - [`Float`] for floating point numbers.
//! - [`Word`] for unsigned integers.
//! - [`SignedWord`] for signed integers.
//! - [`Atomic`] for Atomic values.
//! - [`AtomicNumber`] for Atomic numbers.
//! - [`NonZero`] for the non zero variants of numbers.
//!
//!  These are similar to the ones from [`num-traits`](https://docs.rs/num-traits/latest/num_traits/)
//! but they are more connected which allows to write generic codes with less bounds.
//!
//! The numerical traits dependancy chains is this:
//!
//! ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/deps.svg)
//!
//! This crate adds emulated atomic floats through [`fetch_update`](`core::sync::atomic::AtomicU32::fetch_update`)
//! for the following types:
//! - [`f64`] as [`AtomicF64`]
//! - [`f32`] as [`AtomicF32`]
//! - [`half::f16`] as [`AtomicF16`]
//! - [`half::bf16`] as [`AtomicBF16`]
//!
//! The crate also contains a couple of extra traits:
//! - [`Rng`] for a generic random number generator.
//! - [`Splat`] to broadcast a smaller type on a larger type, mainly used for [SWAR](https://en.wikipedia.org/wiki/SWAR).
//! - [`Sequence`], [`SequenceMut`], and [`SequenceGrowable`] to abstract over slices and other sequence like types.
//!
//! Traits for conversion between types are also provided:
//! - [`UpcastableInto`] and [`UpcastableFrom`] to cast primitive values which are known to don't lose precision.
//!
//! ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/upcast.svg)
//!
//! - [`DowncastableInto`] and [`DowncastableFrom`] to cast primitive values which are known to lose precision.
//!
//! ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/downcast.svg)
//!
//! - [`CastableInto`] and [`CastableFrom`] to cast primitive values which may or may not lose precision.
//!     This is the union of [`DowncastableInto`] and [`UpcastableInto`].
//! - [`To`], to cast primitve values using `as`.
//!
//! The difference between `Castable` and [`To`] is that `Castable` does not
//! allow casting from `f32` to `u32` for example,
//! because `Castable` is implemented only between integers and between floats,
//! while [`To`] is implemented for all primitive types.
//!
//! ### Features
//! This crate has the following features:
//! - `simd`: To enable `portable_simd` and be able to do generic simd code
//! - `atomic_from_mut`: to add the `get_mut_slice` and `from_mut_slice` methods
//! - `std`: to disable for `no_std`
//! - `half`: to enable support for `half::f16` (WORK IN PROGRESS)
//!
//! # Example
//! Mixed precision generic dot products!
//! ```rust
//! use common_traits::*;
//!
//! #[inline]
//! pub fn dot_product<MT: Number, RT: Number, A, B>(a: A, b: B) -> RT
//! where
//!     A: Sequence,
//!     B: Sequence,
//!     A::Item: To<MT>,
//!     B::Item: To<MT>,
//!     MT: To<RT>,
//!     RT: To<MT>,
//! {
//!     // Check compatability of the vectors
//!     assert_eq!(a.len(), b.len());
//!
//!     // Compute the dot product
//!     let mut accum = RT::ZERO;
//!     for (a, b) in a.iter().zip(b.iter()) {
//!         accum = (a.to()).mul_add(b.to(), accum.to()).to();
//!     }
//!
//!     accum
//! }
//!
//! let x: Vec<f32> = vec![1.0, 2.0, 3.0];
//! let w: Vec<u8> = vec![3, 2, 1];
//! // compute the dot product between f32 and u8, casting to f64 and
//! // accumulating as u16
//! let res: u16 = dot_product::<f64, _, _, _>(&x, &w);
//! println!("{:?}", res);
//! ```
//!

#[cfg(feature = "alloc")]
extern crate alloc;

mod word;
pub use word::Word;

mod integer;
pub use integer::Integer;

mod fastrange;
pub use fastrange::FastRange;

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
