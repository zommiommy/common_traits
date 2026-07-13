# Change Log

## [0.13.0] - 2026-07-11

### Fixed

- `UnsignedInt::div_ceil` was dividing by `self`.

- The `fN::BITS` cast now uses `as _` instead of `as usize`, preparing for the
  stabilization of `float_bits_const` without triggering a clippy lint (#7, #8).

- The test for subnormality in `f16` was actually a test for normality.

- `SelectInWord for u64` gated its `core::arch::x86_64::_pdep_u64` fast path only
  on `target_feature = "bmi2"`, so it failed to compile on non-`x86_64` targets
  (e.g. `i686`) with BMI2 enabled; the fast path is now additionally gated on
  `target_arch = "x86_64"`.

- `Sequence::get`/`get_unchecked` on a `[T; N]` array recursed infinitely (stack
  overflow) because the array impl called `<[T; N]>::get_unchecked` (the trait
  method itself); it now delegates to the slice method.

- `UnsignedInt::ilog2_ceil` returned `self` for `self <= 2`, so `ilog2_ceil(1)`
  was 1 (should be 0) and `ilog2_ceil(2)` was 2 (should be 1), and it failed to
  panic at 0; it now rounds the base-2 logarithm up correctly.

### Changed

- Switched to the 2024 edition, bumping the MSRV to Rust 1.85.

- Upgraded to `impl-tools` 0.12, which replaces the unmaintained
  `proc-macro-error2` (flagged as future-incompatible by `rustc`) with the
  maintained `proc-macro-error3` (#9).

- The float constants `MANTISSA_DIGITS`, `MAX_EXP`, `MIN_EXP`, `MAX_10_EXP`, and
  `MIN_10_EXP` were of type `usize`; they now use the standard-library types
  (`u32` for `MANTISSA_DIGITS`, `i32` for the exponents).

- `SignedInt::NonZeroUnsignedInt` has been renamed `SignedInt::NonZeroSignedInt`.

- Removed the stale `#![allow(unstable_name_collisions)]`.

## [0.12.1] - 2026-03-28

### Fixed

- Cast `<f32>::BITS` / `<f64>::BITS` to `usize` in the `impl_float!` atomic
  `AsBytes` impl, preventing a build failure when `float_bits_const` stabilizes.

## [0.11.4] - 2026-03-28

### Fixed

- Cast `<f32>::BITS` / `<f64>::BITS` to `usize` in the `impl_float!` atomic
  `AsBytes` impl, preventing a build failure when `float_bits_const` stabilizes.

## [0.12.0] - 2025-07-01

### New

- New unsafe trait `SameAs` guaranteeing that atomic types and their associated
  non-atomic types have the same memory representation.

## [0.11.3] - 2025-04-30

### Fixed

- Removed all glob imports, as they produced errors on Rust 1.88-nightly.

## [0.11.2] - 2025-01-21

### New

- `usize` and `isize` impls for `To`.

### Fixed

- `AtomicF64`, `AtomicF32`, `AtomicF16`, and `AtomicBF16` had a wrong
  implementation of `fetch_add`, `fetch_sub`, `fetch_min`, and `fetch_max`.

## [0.11.1] - 2024-06-19

### Fixed

- `invariant_eq` and `invariant_ne` now refer to `invariant` by an absolute
  path, so it is no longer necessary to import `common_traits::invariant` to use
  them.

## [0.11.0] - 2024-06-14

### New

- `invariant!`, `invariant_eq!`, and `invariant_ne!`, to be used instead of
  `debug_assert!`.

- Auto-publish pipeline: creating a tagged release makes the pipeline test
  everything, put the changelog section in the release, check that the tag
  equals the package version, and finally publish on crates.io.

### Changed

- Removed the `atomic_from_mut` feature, as an implementation is now always
  provided.

### Fixed

- Removed cases where `pointer_width` was in [8, 128], as Rust does not actually
  support them.
