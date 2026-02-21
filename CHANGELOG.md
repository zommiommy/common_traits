# Change Log

## [unreleased]

### Fixed

- `UnsignedInt::div_ceil` was dividing by `self`.

- Constants for mantissa, exp, etc. were of type `usize`, whereas some are
  signed.

- The test for subnormality in `f16` was actually a test for normality.

- `SignedInt::NonZeroSignedInt` was named `SignedInt::NonZeroUnsignedInt`.

## [0.12.0] - 2025-07-01

### New

- New unsafe trait `SameAs` to guarantee that atomic types and associated
  non-atomic types have the same memory representation.

## [0.11.3] - 2025-04-30

### Fixed

- Removed all glob imports as produced errors on rust 1.88-nightly.

## [0.11.2] - 2025-01-21

### Fixed

- `AtomicF64`, `AtomicF32`, `AtomicF16`, and `AtomicBF16` had wrong
  implementation of `fetch_add`, `fetch_sub`, `fetch_min`,`fetch_max`.
- added `usize` and `isize` impls to `To`.

## [0.11.1] - 2024-06-19

### Fixed

- Now `invariant_eq` and `invariant_ne` use `invariant` with an absolute path so
  it's no longer needed to import `common_traits::invariant` in order to use
  `invariant_eq` and `invariant_ne`.

## [0.11.0] - 2024-06-14

### New

- `invariant`, `invariant_eq`, `invariant_ne`, to use instead of debug_asserts
- auto-publish pipeline, just create a tagged release. The pipeline will test
  everything, put the changelog section in the release, check semantic versioning
  check that the tag is equal to the package version, and finally publish on
  crates.io

### Removed

- Removed feature atomic_from_mut as now we always provide an implementation

### Fixed

- Removed cases where `pointer_width` in [8, 128] as rust doesn't actually support them
