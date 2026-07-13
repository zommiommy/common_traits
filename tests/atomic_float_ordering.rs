use common_traits::{Atomic, AtomicF64, AtomicNumber};
use core::sync::atomic::Ordering;

/// Regression: float atomic RMW methods passed the caller's `order` as the
/// *load* ordering of their `compare_exchange` loop, so `Release`/`AcqRel`
/// panicked ("there is no such thing as a release load") and the successful
/// store was always `Relaxed`.
///
/// Only the single-ordering methods (`fetch_add`/`fetch_sub`/`fetch_min`/
/// `fetch_max`) derive their load ordering via `load_ordering`, so every store
/// ordering must be accepted. The `fetch_saturating_*` methods take an explicit
/// fetch ordering and are intentionally not exercised with invalid load
/// orderings here.
#[test]
fn test_float_rmw_accepts_every_ordering() {
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
        assert_eq!(b.fetch_sub(4.0, order), 10.0);
        assert_eq!(b.load(Ordering::Relaxed), 6.0);

        let c = <AtomicF64 as Atomic>::new(10.0);
        assert_eq!(c.fetch_min(4.0, order), 10.0);
        assert_eq!(c.load(Ordering::Relaxed), 4.0);

        let d = <AtomicF64 as Atomic>::new(10.0);
        assert_eq!(d.fetch_max(40.0, order), 10.0);
        assert_eq!(d.load(Ordering::Relaxed), 40.0);
    }
}
