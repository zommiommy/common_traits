use common_traits::IntoAtomic;
use core::sync::atomic::Ordering;

// These use `u64`, whose atomic (`AtomicU64`) is 8-aligned while `u64` is only
// 4-aligned on 32-bit x86. The conversions must round-trip on aligned data and
// must not choke on empty slices/arrays (the zero-length path). On 32-bit x86
// they now check alignment at run time instead of forming an under-aligned
// reference (UB); that path is exercised by the i686 build.

#[test]
fn from_mut_slice_roundtrips_and_handles_empty() {
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
fn from_mut_array_roundtrips_and_handles_empty() {
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
