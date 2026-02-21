use common_traits::*;
use core::fmt::Debug;
use core::sync::atomic::{
    AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32,
    AtomicU64, AtomicUsize, Ordering,
};

fn test_atomic<F: Atomic>()
where
    usize: To<F::NonAtomicType>,
    F::NonAtomicType: Debug + Number,
{
    let x = F::new(1_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 1_usize.to());

    x.store(2_usize.to(), Ordering::Relaxed);
    assert_eq!(x.load(Ordering::Relaxed), 2_usize.to());

    assert_eq!(x.swap(3_usize.to(), Ordering::Relaxed), 2_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 3_usize.to());

    assert_eq!(
        x.compare_exchange(
            3_usize.to(),
            4_usize.to(),
            Ordering::Relaxed,
            Ordering::Relaxed
        ),
        Ok(3_usize.to())
    );
    assert_eq!(x.load(Ordering::Relaxed), 4_usize.to());

    assert_eq!(
        x.compare_exchange(
            3_usize.to(),
            5_usize.to(),
            Ordering::Relaxed,
            Ordering::Relaxed
        ),
        Err(4_usize.to())
    );
    assert_eq!(x.load(Ordering::Relaxed), 4_usize.to());
}

fn test_atomic_number<F: AtomicNumber>()
where
    usize: To<F::NonAtomicType>,
    F::NonAtomicType: Debug + Number,
{
    let x = F::new(1.to());
    assert_eq!(x.fetch_add(3_usize.to(), Ordering::Relaxed), 1_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 4_usize.to());

    assert_eq!(x.fetch_sub(4_usize.to(), Ordering::Relaxed), 4_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 0_usize.to());

    assert_eq!(x.fetch_max(63_usize.to(), Ordering::Relaxed), 0_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 63_usize.to());

    assert_eq!(x.fetch_min(30_usize.to(), Ordering::Relaxed), 63_usize.to());
    assert_eq!(x.load(Ordering::Relaxed), 30_usize.to());
}

#[cfg(feature = "half")]
#[test]
fn test_atomic_float_bf16() {
    test_atomic::<AtomicBF16>();
    test_atomic_number::<AtomicBF16>();
}

#[cfg(feature = "half")]
#[test]
fn test_atomic_float_f16() {
    test_atomic::<AtomicF16>();
    test_atomic_number::<AtomicF16>();
}

#[test]
fn test_atomic_float_f32() {
    test_atomic::<AtomicF32>();
    test_atomic_number::<AtomicF32>();
}

#[test]
fn test_atomic_float_f64() {
    test_atomic::<AtomicF64>();
    test_atomic_number::<AtomicF64>();
}

#[test]
fn test_atomic_int_i8() {
    test_atomic::<AtomicI8>();
    test_atomic_number::<AtomicI8>();
}

#[test]
fn test_atomic_int_i16() {
    test_atomic::<AtomicI16>();
    test_atomic_number::<AtomicI16>();
}

#[test]
fn test_atomic_int_i32() {
    test_atomic::<AtomicI32>();
    test_atomic_number::<AtomicI32>();
}

#[test]
fn test_atomic_int_i64() {
    test_atomic::<AtomicI64>();
    test_atomic_number::<AtomicI64>();
}

#[test]
fn test_atomic_int_isize() {
    test_atomic::<AtomicIsize>();
    test_atomic_number::<AtomicIsize>();
}

#[test]
fn test_atomic_uint_u8() {
    test_atomic::<AtomicU8>();
    test_atomic_number::<AtomicU8>();
}

#[test]
fn test_atomic_uint_u16() {
    test_atomic::<AtomicU16>();
    test_atomic_number::<AtomicU16>();
}

#[test]
fn test_atomic_uint_u32() {
    test_atomic::<AtomicU32>();
    test_atomic_number::<AtomicU32>();
}

#[test]
fn test_atomic_uint_u64() {
    test_atomic::<AtomicU64>();
    test_atomic_number::<AtomicU64>();
}

#[test]
fn test_atomic_uint_usize() {
    test_atomic::<AtomicUsize>();
    test_atomic_number::<AtomicUsize>();
}
