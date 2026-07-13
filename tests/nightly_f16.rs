#![cfg_attr(feature = "nightly_f16", feature(f16))]
//! Regression tests for native `f16` (`nightly_f16`) trait implementations.

// Float math on `f16` is not reproducible under Miri's intrinsic emulation, so
// gate these value assertions out of Miri (matching the `half` tests).
#[cfg(all(feature = "nightly_f16", not(feature = "half"), not(miri)))]
mod tests {
    use common_traits::*;

    #[test]
    fn native_f16_number_and_float() {
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
    fn native_f16_casts() {
        assert_eq!(<f16 as UpcastableInto<f32>>::upcast(2.0), 2.0_f32);
        assert_eq!(<f16 as UpcastableInto<f64>>::upcast(2.0), 2.0_f64);
        assert_eq!(<f32 as DowncastableInto<f16>>::downcast(2.0_f32), 2.0);
        assert_eq!(<f16 as CastableInto<f32>>::cast(3.0), 3.0_f32);
        assert_eq!(<u8 as To<f16>>::to(5), 5.0);
    }
}
