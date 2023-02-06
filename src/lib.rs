#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(feature="atomic_from_mut", feature(atomic_from_mut))]
#![cfg_attr(feature="simd", feature(portable_simd))]
#![deny(unconditional_recursion)]

use core::ops::*;
use core::num::*;

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

mod impls;

mod splat;
pub use splat::Splat;

mod max_lanes;
pub use max_lanes::MaxLanes;
 
mod number;
pub use number::Number;

mod to;
pub use to::To;

/// Collection of sub-traits needed for conditional compilation of simd traits 
/// for generical SIMD operations
pub mod simd {
    pub use crate::word::WSimd;
    pub use crate::integer::ISimd;
    pub use crate::signed_word::SSimd;
    pub use crate::float::FSimd;
}
