use common_traits::{Atomic, IntoAtomic};
use core::sync::atomic::{AtomicU64, Ordering};

// These use `u64`, whose atomic (`AtomicU64`) is 8-aligned while `u64` is only
// 4-aligned on 32-bit x86. The conversions must round-trip on aligned data and
// must not choke on empty slices/arrays (the zero-length path). The runtime
// alignment check that guards the 32-bit-x86 case (where these would otherwise
// form an under-aligned reference: UB) is exercised directly by the
// `reinterpret_mut_*` unit tests in `src/atomic.rs`.

#[test]
fn test_from_mut_slice_roundtrips_and_handles_empty() {
    let mut v = [1u64, 2, 3, 4];
    {
        let atomics = <u64 as IntoAtomic>::from_mut_slice(&mut v);
        atomics[0].store(99, Ordering::Relaxed);
    }
    assert_eq!(v[0], 99);

    let mut empty: [u64; 0] = [];
    let atomics = <u64 as IntoAtomic>::from_mut_slice(&mut empty);
    assert_eq!(atomics.len(), 0);
}

#[test]
fn test_from_mut_array_roundtrips_and_handles_empty() {
    let mut v = [1u64, 2];
    {
        let atomics = <u64 as IntoAtomic>::from_mut_array(&mut v);
        atomics[1].store(7, Ordering::Relaxed);
    }
    assert_eq!(v[1], 7);

    let mut empty: [u64; 0] = [];
    let atomics = <u64 as IntoAtomic>::from_mut_array(&mut empty);
    assert_eq!(atomics.len(), 0);
}

#[test]
fn test_atomic_from_mut_slice_roundtrips_and_handles_empty() {
    let mut v = [1u64, 2, 3, 4];
    {
        let atomics = <AtomicU64 as Atomic>::from_mut_slice(&mut v);
        atomics[0].store(99, Ordering::Relaxed);
    }
    assert_eq!(v[0], 99);

    let mut empty: [u64; 0] = [];
    let atomics = <AtomicU64 as Atomic>::from_mut_slice(&mut empty);
    assert_eq!(atomics.len(), 0);
}

#[test]
fn test_atomic_from_mut_array_roundtrips() {
    let mut v = [1u64, 2];
    {
        let atomics = <AtomicU64 as Atomic>::from_mut_array(&mut v);
        atomics[1].store(7, Ordering::Relaxed);
    }
    assert_eq!(v[1], 7);
}
