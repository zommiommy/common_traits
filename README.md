# common_traits

[![downloads](https://img.shields.io/crates/d/common_traits)](https://crates.io/crates/common_traits)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/common_traits)](https://crates.io/crates/common_traits/reverse_dependencies)
![GitHub CI](https://github.com/zommiommy/common_traits-rs/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/crates/l/common_traits)
[![](https://tokei.rs/b1/github/zommiommy/common_traits-rs?type=Rust,Python)](https://github.com/vigna/common_traits-rs)
[![Latest version](https://img.shields.io/crates/v/common_traits.svg)](https://crates.io/crates/common_traits)
[![Documentation](https://docs.rs/common_traits/badge.svg)](https://docs.rs/common_traits)
 
A collection of traits and dependencies that can be used to write code
that is generic over numerical types. It provides also atomic floats
implemented using the integer atomic byte with the same number of bits,
and support for half precision floats via the crate [`half`](https://crates.io/crates/half).

Additionally, there are a few traits
missing from the standard library, such as [`Sequence`], and variants
of existing library traits such as [`Rng`] and [`Hash`].

Finally, we provide traits for casting between types, such as [`UpcastableInto`],
and fast implementation of a few primitives such [`FastRange`] and [`SelectInWord`].

Everything is experimental and I'll change them to my needs, respecting
semantic versioning. :)

The point of making this crate public is to be able to discuss this
as it covers many core missings from Rust.

The traits in this crate are similar to the ones from 
[`num-traits`](https://docs.rs/num-traits/latest/num_traits/)
but they are more interconnected, which allows to write generic code
(e.g., code mixing a type and its associated atomic type) more easily
and with less trait bounds.

 The numerical traits dependancy chains is this:

 ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/deps.svg)

 This crate adds emulated atomic floats through [`fetch_update`](`core::sync::atomic::AtomicU32::fetch_update`)
 for the following types:
 - [`f64`] as [`AtomicF64`]
 - [`f32`] as [`AtomicF32`]
 - [`half::f16`] as [`AtomicF16`]
 - [`half::bf16`] as [`AtomicBF16`]

 The crate also contains a couple of extra traits:
 - [`Rng`] for a generic random number generator.
 - [`Splat`] to broadcast a smaller type on a larger type, mainly used for [SWAR](https://en.wikipedia.org/wiki/SWAR).
 - [`SelectInWord`] to find the position of the i-th 1 or 0 in words of memory.
 - [`FastRange`] for faster div, mod, and range operations.
 - [`Sequence`], [`SequenceMut`], and [`SequenceGrowable`] to abstract over slices and other sequence like types.

 Traits for conversion between types are also provided:
 - [`UpcastableInto`] and [`UpcastableFrom`] to cast primitive values which are known to don't lose precision.

 ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/upcast.svg)

 - [`DowncastableInto`] and [`DowncastableFrom`] to cast primitive values which are known to lose precision.

 ![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/downcast.svg)

 - [`CastableInto`] and [`CastableFrom`] to cast primitive values which may or may not lose precision.
     This is the union of [`DowncastableInto`] and [`UpcastableInto`].
 - [`To`], to cast primitve values using `as`.

 The difference between `Castable` and [`To`] is that `Castable` does not
 allow casting from `f32` to `u32` for example,
 because `Castable` is implemented only between integers and between floats,
 while [`To`] is implemented for all primitive types.

 ### Features
 This crate has the following features:
 - `simd`: To enable `portable_simd` and be able to do generic simd code
 - `atomic_from_mut`: to add the `get_mut_slice` and `from_mut_slice` methods
 - `std`: to disable for `no_std`
 - `half`: to enable support for `half::f16` (WORK IN PROGRESS)

 # Example
 Mixed precision generic dot products!
 ```rust
 use common_traits::*;

 #[inline]
 pub fn dot_product<MT: Number, RT: Number, A, B>(a: A, b: B) -> RT
 where
     A: Sequence,
     B: Sequence,
     A::Item: To<MT>,
     B::Item: To<MT>,
     MT: To<RT>,
     RT: To<MT>,
 {
     // Check compatability of the vectors
     assert_eq!(a.len(), b.len());

     // Compute the dot product
     let mut accum = RT::ZERO;
     for (a, b) in a.iter().zip(b.iter()) {
         accum = (a.to()).mul_add(b.to(), accum.to()).to();
     }

     accum
 }

 let x: Vec<f32> = vec![1.0, 2.0, 3.0];
 let w: Vec<u8> = vec![3, 2, 1];
 // compute the dot product between f32 and u8, casting to f64 and
 // accumulating as u16
 let res: u16 = dot_product::<f64, _, _, _>(&x, &w);
 println!("{:?}", res);
 ```
