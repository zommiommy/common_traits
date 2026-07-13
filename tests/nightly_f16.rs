#![cfg_attr(feature = "nightly_f16", feature(f16))]
//! Regression tests for native `f16` (`nightly_f16`) trait implementations.

// Float math on `f16` is not reproducible under Miri's intrinsic emulation, so
// gate these value assertions out of Miri (matching the `half` tests).
#[cfg(all(feature = "nightly_f16", not(feature = "half"), not(miri)))]
mod tests {
    use common_traits::*;

    #[test]
    fn test_native_f16_number_and_float() {
        assert_eq!(<f16 as Number>::mul_add(2.0, 3.0, 1.0), 7.0);
        assert_eq!(<f16 as Number>::max(1.0, 2.0), 2.0);
        assert_eq!(<f16 as Float>::sqrt(4.0), 2.0);
        assert_eq!(<f16 as Float>::powi(2.0, 3), 8.0);
        assert_eq!(<f16 as Float>::powf(2.0, 3.0), 8.0);
        assert_eq!(<f16 as Float>::floor(2.5), 2.0);
        assert_eq!(<f16 as Float>::abs(-1.5), 1.5);
        assert!(<f16 as Float>::is_nan(<f16 as Float>::NAN));
        assert_eq!(<f16 as Float>::recip(2.0), 0.5);
    }

    #[test]
    fn test_native_f16_casts() {
        assert_eq!(<f16 as UpcastableInto<f32>>::upcast(2.0), 2.0_f32);
        assert_eq!(<f16 as UpcastableInto<f64>>::upcast(2.0), 2.0_f64);
        assert_eq!(<f32 as DowncastableInto<f16>>::downcast(2.0_f32), 2.0);
        assert_eq!(<f16 as CastableInto<f32>>::cast(3.0), 3.0_f32);
        assert_eq!(<u8 as To<f16>>::to(5), 5.0);
    }

    #[test]
    fn test_native_f16_to_degrees_radians() {
        // Regression: native `f16` `to_degrees`/`to_radians` computed in the
        // `f16` domain (multiplying by an `f16`-rounded constant), so
        // `PI.to_degrees()` returned 179.9 instead of 180. They now widen
        // through `f32`, matching the accurate value and the `half` backend.
        let pi = core::f16::consts::PI;
        assert_eq!(<f16 as Float>::to_degrees(pi), 180.0);
        assert_eq!(
            <f16 as Float>::to_degrees(pi),
            (pi as f32).to_degrees() as f16
        );
        let deg: f16 = 180.0;
        assert_eq!(
            <f16 as Float>::to_radians(deg),
            (deg as f32).to_radians() as f16
        );

        // The atomic RMW mirror must agree with the value API (it previously
        // dispatched through the inaccurate inherent `f16` method).
        let a = <AtomicF16 as Atomic>::new(pi);
        <AtomicF16 as AtomicFloat>::fetch_to_degrees(&a, core::sync::atomic::Ordering::Relaxed);
        assert_eq!(
            <AtomicF16 as Atomic>::load(&a, core::sync::atomic::Ordering::Relaxed),
            180.0
        );
        let b = <AtomicF16 as Atomic>::new(deg);
        <AtomicF16 as AtomicFloat>::fetch_to_radians(&b, core::sync::atomic::Ordering::Relaxed);
        assert_eq!(
            <AtomicF16 as Atomic>::load(&b, core::sync::atomic::Ordering::Relaxed),
            (deg as f32).to_radians() as f16
        );
    }
}
