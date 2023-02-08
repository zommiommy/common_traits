#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(feature="atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature="simd", feature(portable_simd))]
#![cfg_attr(feature="simd", feature(generic_const_exprs))]
#![deny(unconditional_recursion)]
//! This is a collection of traits and dependancies I often use to write generic
//! code for both data-structures and scientific computing. 
//! 
//! Everything is experimental and I'll change them to my needs :) 
//! 
//! The point of making this crate public is to be able to discuss this 
//! as it covers many core missings from Rust.
//! 
//! The numerical traits dependancy chains is this:
//! 
//! ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/deps.svg)
//! 
//! The crate also contains a couple of extra traits:
//! - [`To`], to cast primitve values using `as`.
//! - [`Rng`] for a generic random number generator.
//! - [`Splat`] to broadcast a smaller type on a larger type, mainly used for [SWAR](https://en.wikipedia.org/wiki/SWAR).
//! 
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
//! #[derive(Debug)]
//! /// a 1D vector
//! pub struct Vector<T: Number>(Vec<T>);
//! 
//! impl<T: Number> Vector<T> {
//!    #[inline]
//!    pub fn dot_product<T2: Number, RT: Number>(&self, other: &Vector<T2>) -> RT
//!    where
//!        T: To<RT>,
//!        T2: To<T>,
//!    {
//!         // Check compatability of the vectors
//!         assert_eq!(self.0.len(), other.0.len());
//! 
//!         // Compute the dot product
//!         let mut accum = T::ZERO;
//!         for (a, b) in self.0.iter().zip(other.0.iter()) {
//!             accum = (*a).mul_add(b.to(), accum);
//!         }
//! 
//!         accum.to()
//!    }
//! }
//! 
//! fn main() {
//!     let x: Vector<f32> = Vector(vec![1.0, 2.0, 3.0]);
//!     let w: Vector<u8> = Vector(vec![3, 2, 1]);
//! 
//!     let res: u16 = x.dot_product(&w);
//! 
//!     println!("{:?}", res);
//! }
//! ```
//! 
mod word;
pub use word::Word;

mod integer;
pub use integer::Integer;

mod signed_word;
pub use signed_word::SignedWord;

mod atomic_word;
pub use atomic_word::AtomicWord;

mod float;
pub use float::Float;

mod non_zero;
pub use non_zero::NonZero;

mod number;
pub use number::Number;

mod impls;

mod splat;
pub use splat::Splat;
 
mod to;
pub use to::To;

mod rnd;
pub use rnd::*;