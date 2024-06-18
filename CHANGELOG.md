# Change Log

## [0.11.1] - 2024-06-19

### Fixed

* Now `invariant_eq` and `invariant_ne` use `invariant` with an absolute path so
    it's no longer needed to import `common_traits::invariant` in order to use
    `invariant_eq` and `invariant_ne`.


## [0.11.0] - 2024-06-14

### New

* `invariant`, `invariant_eq`, `invariant_ne`, to use instead of debug_asserts
* auto-publish pipeline, just create a tagged release. The pipeline will test 
  everything, put the changelog section in the release, check semantic versioning
  check that the tag is equal to the package version, and finally publish on 
  crates.io 

### Removed

* Removed feature atomic_from_mut as now we always provide an implementation

### Fixed

* Removed cases where `pointer_width` in [8, 128] as rust doesn't actually support them

