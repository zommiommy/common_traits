use common_traits::{Atomic, AtomicF64, AtomicNumber};
use core::sync::atomic::Ordering;

/// Regression: float atomic RMW methods passed the caller's `order` as the
/// *load* ordering of their `compare_exchange` loop, so `Release`/`AcqRel`
/// panicked ("there is no such thing as a release load") and the successful
/// store was always `Relaxed`.
#[test]
fn float_rmw_accepts_every_ordering() {
    for order in [
        Ordering::Relaxed,
        Ordering::Acquire,
        Ordering::Release,
        Ordering::AcqRel,
        Ordering::SeqCst,
    ] {
        let a = <AtomicF64 as Atomic>::new(1.0);
        assert_eq!(a.fetch_add(2.0, order), 1.0);
        assert_eq!(a.load(Ordering::Relaxed), 3.0);

        let b = <AtomicF64 as Atomic>::new(10.0);
        assert_eq!(b.fetch_min(4.0, order), 10.0);
        assert_eq!(b.load(Ordering::Relaxed), 4.0);
    }
}
