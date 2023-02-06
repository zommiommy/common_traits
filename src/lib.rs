#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(feature="atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature="simd", feature(portable_simd))]
#![deny(unconditional_recursion)]
//! This is a collection of traits and dependancies I often use to write generic
//! code for both data-structures and scientific computing. 
//! 
//! The numerical traits dependancy chains is this:
//! ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/deps.svg)
//! 
//! The crate also contains a couple of extra traits:
//! - [`To`], to cast primitve values using `as`.
//! - [`Rng`] for a generic random number generator.
//! - [`Splat`] to broadcast a smaller type on a larger type, mainly used for SWAR.
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