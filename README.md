# common_traits

[![downloads](https://img.shields.io/crates/d/common_traits)](https://crates.io/crates/common_traits)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/common_traits)](https://crates.io/crates/common_traits/reverse_dependencies)
![GitHub CI](https://github.com/zommiommy/common_traits/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/crates/l/common_traits)
[![](https://tokei.rs/b1/github/zommiommy/common_traits?type=Rust,Python)](https://github.com/zommiommy/common_traits)
[![Latest version](https://img.shields.io/crates/v/common_traits.svg)](https://crates.io/crates/common_traits)
[![Documentation](https://docs.rs/common_traits/badge.svg)](https://docs.rs/common_traits)

A collection of traits and dependencies that can be used to write code
that is generic over numerical types. It provides also atomic floats
implemented using the integer atomic byte with the same number of bits,
and support for half precision floats via the crate [`half`](https://crates.io/crates/half).

Additionally, there are a few traits
missing from the standard library, such as [`Sequence`], variants
of existing library traits such as [`Rng`] and [`Hash`], and macros like
[`invariant`].

Finally, we provide traits for casting between types, such as [`UpcastableInto`],
and fast implementation of a few primitives such [`FastRange`] and [`SelectInWord`].

Everything is experimental and I'll change them to my needs, respecting
semantic versioning. :)

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
    // Ensure compatability of the vectors
    invariant_eq!(a.len(), b.len());

    // Compute the dot product
    let mut accum = RT::ZERO;
    for (a, b) in a.iter().zip(b.iter()) {
        accum = (a.to()).mul_add(b.to(), accum.to()).to();
    }

    accum
}

// Using arrays instead of Vec to avoid alloc dependency
let x: [f32; 3] = [1.0, 2.0, 3.0];
let w: [u8; 3] = [3, 2, 1];
// compute the dot product between f32 and u8, casting to f64 and
// accumulating as u16
let res: u16 = dot_product::<f64, _, _, _>(&x, &w);
println!("{:?}", res);
```

# Numerical traits at a glance

The numerical traits dependancy chains is the following.
Black arcs represent the traits dependancies, the blu arcs represent the
possibility to access an associated type implementing that trait.

![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/deps.svg)

# Why?

The point of making this crate public is to be able to discuss this
as it covers many core missings from Rust.

The traits in this crate are similar to the ones from
[`num-traits`](https://docs.rs/num-traits/latest/num_traits/)
but they are more interconnected (the blue arcs in the previous graph), which allows to write generic code
(e.g., code mixing a type and its associated atomic type) more easily
and with less trait bounds.

## Summary

An highlight of common_traits most noteworthy features.

#### Macros

This crate adds the following macros, [`invariant`], [`invariant_eq`], [`invariant_ne`]
which are similar to the std debug_assert macros, which get checked during debug
runs and get replaced with an `unreachable_unchecked` on release builds.

#### Structs

This crate adds emulated atomic floats through [`fetch_update`](`core::sync::atomic::AtomicU32::fetch_update`)
for the following types:

- [`f64`] as [`AtomicF64`]
- [`f32`] as [`AtomicF32`]
- [`f16`] or [`half::f16`] as [`AtomicF16`]
- [`half::bf16`] as [`AtomicBF16`]

#### Numerical Traits

This crate provides the following traits for numerical types:

- [`Number`] Something that can be added, subtracted, multiplied, divided and
  has a Zero and a One.
- [`FiniteRangeNumber`] a [`Number`] which has a Minimum and a Maximum.
- [`Float`] float numbers.
- [`Integer`] an integer number represented as a sequence of bits.
- [`SignedInt`] a signed integer represented in 2-complement.
- [`UnsignedInt`] an unsigned integer.

#### Atomic Numerical Traits

There are two main traits for working with atomic values:

- [`Atomic`] for values that can be read and written atomically.
- [`IntoAtomic`] for values that can be converted into atomic types.

Each numerical trait has an atomic equivalent:

- [`AtomicNumber`]
- [`AtomicFiniteRangeNumber`]
- [`AtomicFloat`]
- [`AtomicInteger`]
- [`AtomicSignedInt`]
- [`AtomicUnsignedInt`]

#### Miscellaneous Traits

The crate also contains a couple of extra traits:

- [`Sequence`], [`SequenceMut`], and [`SequenceGrowable`] to abstract over
  slices and other sequence like types.
- [`AsBytes`], [`ToBytes`] and [`FromBytes`] are traits used to convert forward
  and back types to bytes.
- [`NonZero`] a version of `Self` which cannot be zero, [`UnsignedInt`] and
  [`SignedInt`] have an associated type implementing this.
- [`FastRange`] for faster div, mod, and range operations.
- [`SelectInWord`] to find the position of the i-th 1 or 0 in words of memory.
- [`Splat`] to broadcast a smaller type on a larger type, mainly used for
  [SWAR](https://en.wikipedia.org/wiki/SWAR).
- [`Rng`] for a generic random number generator.
- [`Hasher`] which is like [`std::hash::Hasher`] but allow returing a generic
  type instead of an `u64`.
- [`SeedableHasher`] which is a standard way to initialize hashers

#### Conversion traits

Traits for conversion between types are also provided:

- [`To`], to cast primitve values using `as`.
- [`DoubleType`] and [`HalfType`] can be used to access bigger or smaller
  types in a generic way.
- [`UpcastableInto`] and [`UpcastableFrom`] to cast primitive values which
  can not lose precision.

![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/upcast.svg)

- [`DowncastableInto`] and [`DowncastableFrom`] to cast primitive values which
  can lose precision.

![](https://raw.githubusercontent.com/zommiommy/common_traits/main/img/downcast.svg)

- [`CastableInto`] and [`CastableFrom`] to cast primitive values which may or may not lose precision.
  This is the union of [`DowncastableInto`] and [`UpcastableInto`].

The difference between `Castable` and [`To`] is that `Castable` does not
allow casting from `f32` to `u32` for example,
because `Castable` is implemented only between integers and between floats,
while [`To`] is implemented for all primitive types.

# Features

This crate has the following features:

- `simd`: To enable `portable_simd` and be able to do generic simd code
- `atomic_from_mut`: to add the `get_mut_slice` and `from_mut_slice` methods
- `std`: to disable for `no_std`
- `half`: to enable support for `half::f16` (Experimental)
