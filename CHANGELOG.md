# Change Log

## [unreleased]

### Changed

- 2024 edition, Rust 1.85.

### Fixed

- Bumped `impl-tools` to 0.12, which replaces the unmaintained
  `proc-macro-error2` (rejected by a future Rust version) with
  `proc-macro-error3` (#9).

- Used `as _` instead of `as usize` for `fN::BITS` cast to prepare for
  `float_bits_const` stabilization without triggering clippy (#7, #8).

- Removed stale `#![allow(unstable_name_collisions)]`.

- `UnsignedInt::div_ceil` was dividing by `self`.

- Constants for mantissa, exp, etc. were of type `usize`, whereas some are
  signed.

- The test for subnormality in `f16` was actually a test for normality.

- `SignedInt::NonZeroSignedInt` was named `SignedInt::NonZeroUnsignedInt`.

## [0.12.1] - 2026-03-28

### Fixed

- Cast `<f32>::BITS` / `<f64>::BITS` to `usize` in `impl_float!` atomic
  `AsBytes` impl, preventing a build failure when `float_bits_const` stabilizes.

## [0.11.4] - 2026-03-28

### Fixed

- Cast `<f32>::BITS` / `<f64>::BITS` to `usize` in `impl_float!` atomic
  `AsBytes` impl, preventing a build failure when `float_bits_const` stabilizes.

## [0.12.0] - 2025-07-01

### New

- New unsafe trait `SameAs` to guarantee that atomic types and associated
  non-atomic types have the same memory representation.

## [0.11.3] - 2025-04-30

### Fixed

- Removed all glob imports as they produced errors on Rust 1.88-nightly.

## [0.11.2] - 2025-01-21

### New

- Added `usize` and `isize` impls to `To`.

### Fixed

- `AtomicF64`, `AtomicF32`, `AtomicF16`, and `AtomicBF16` had wrong
  implementation of `fetch_add`, `fetch_sub`, `fetch_min`, `fetch_max`.

## [0.11.1] - 2024-06-19

### Fixed

- Now `invariant_eq` and `invariant_ne` use `invariant` with an absolute path so
  it's no longer needed to import `common_traits::invariant` in order to use
  `invariant_eq` and `invariant_ne`.

## [0.11.0] - 2024-06-14

### New

- `invariant!`, `invariant_eq!`, `invariant_ne!`, to use instead of `debug_assert!`
- auto-publish pipeline, just create a tagged release. The pipeline will test
  everything, put the changelog section in the release, check semantic versioning
  check that the tag is equal to the package version, and finally publish on
  crates.io

### Removed

- Removed feature atomic_from_mut as now we always provide an implementation

### Fixed

- Removed cases where `pointer_width` in [8, 128] as Rust doesn't actually support them
