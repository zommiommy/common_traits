//! Regression tests for numeric trait method fixes.
use common_traits::{FiniteRangeNumber, Float, Integer, Number, SignedInt, UnsignedInt};

#[test]
fn ilog2_ceil_small_values() {
    // Regression: the `self <= 2` special case returned `self` verbatim, so
    // ilog2_ceil(1) == 1 (should be 0) and ilog2_ceil(2) == 2 (should be 1).
    assert_eq!(<u32 as UnsignedInt>::ilog2_ceil(1), 0);
    assert_eq!(<u32 as UnsignedInt>::ilog2_ceil(2), 1);
    assert_eq!(<u32 as UnsignedInt>::ilog2_ceil(3), 2);
    assert_eq!(<u32 as UnsignedInt>::ilog2_ceil(4), 2);
    assert_eq!(<u32 as UnsignedInt>::ilog2_ceil(5), 3);
    assert_eq!(<u64 as UnsignedInt>::ilog2_ceil(1), 0);
    assert_eq!(<u64 as UnsignedInt>::ilog2_ceil(2), 1);
}

#[test]
#[should_panic]
fn ilog2_ceil_zero_panics() {
    // Documented: panics for `self <= 0`.
    let _ = <u32 as UnsignedInt>::ilog2_ceil(0);
}

#[test]
fn div_ceil_no_overflow_near_max() {
    // Regression: `(self + rhs - 1)` overflowed for `self` near the maximum.
    assert_eq!(<u8 as UnsignedInt>::div_ceil(255, 2), 128);
    assert_eq!(<u8 as UnsignedInt>::div_ceil(255, 1), 255);
    assert_eq!(<u8 as UnsignedInt>::div_ceil(254, 2), 127);
    assert_eq!(<u8 as UnsignedInt>::div_ceil(0, 3), 0);
    assert_eq!(<u8 as UnsignedInt>::div_ceil(7, 3), 3);
    assert_eq!(
        <u64 as UnsignedInt>::div_ceil(u64::MAX, 2),
        (u64::MAX / 2) + 1
    );
}

#[test]
fn clamp_normal_range() {
    assert_eq!(<u8 as Number>::clamp(5, 0, 10), 5);
    assert_eq!(<u8 as Number>::clamp(15, 0, 10), 10);
    assert_eq!(<u8 as Number>::clamp(0, 3, 10), 3);
}

#[test]
#[should_panic]
fn clamp_panics_on_min_gt_max() {
    // Documented contract: panics if `min > max`.
    let _ = <u8 as Number>::clamp(5, 10, 0);
}

#[test]
fn extract_bitfield_keeps_sign_bit() {
    // Regression: mask via `Self::MAX` dropped the top bit for signed types.
    assert_eq!(<i8 as Integer>::extract_bitfield(&-1i8, 0, 8), -1);
    assert_eq!(<i8 as Integer>::extract_bitfield(&-1i8, 0, 7), 127);
    assert_eq!(<i16 as Integer>::extract_bitfield(&-1i16, 0, 16), -1);
    // unsigned behaviour is unchanged
    assert_eq!(<u8 as Integer>::extract_bitfield(&0xFFu8, 0, 8), 0xFF);
    assert_eq!(
        <u8 as Integer>::extract_bitfield(&0b1011_0100u8, 2, 6),
        0b1101
    );
}

#[test]
fn abs_diff_returns_unsigned_exact() {
    // Regression: signed `abs_diff` cast the unsigned result back to `Self`, so
    // `i8::MIN.abs_diff(i8::MAX)` returned -1; it now returns the unsigned type.
    assert_eq!(<i8 as Integer>::abs_diff(i8::MIN, i8::MAX), 255u8);
    assert_eq!(<i8 as Integer>::abs_diff(-1, 1), 2u8);
    assert_eq!(<u8 as Integer>::abs_diff(3, 10), 7u8);
    assert_eq!(<i32 as Integer>::abs_diff(i32::MIN, i32::MAX), u32::MAX);
}

#[test]
fn unsigned_sibling_types_are_coherent() {
    // These only type-check if the unsigned-sibling associated types agree, which
    // the `Integer<Unsigned = ...>` bounds on `SignedInt`/`UnsignedInt` enforce.
    fn signed_coherent<S: SignedInt>(s: <S as Integer>::Unsigned) -> <S as SignedInt>::UnsignedInt {
        s
    }
    fn unsigned_coherent<U: UnsignedInt>(u: <U as Integer>::Unsigned) -> U {
        u
    }
    assert_eq!(signed_coherent::<i32>(7u32), 7u32);
    assert_eq!(unsigned_coherent::<u32>(9u32), 9u32);
}

#[test]
fn pow_normal_exponents() {
    assert_eq!(<u64 as Number>::pow(2, 10), 1024);
    assert_eq!(<u32 as Number>::pow(3, 4), 81);
}

#[test]
#[should_panic]
fn pow_panics_on_exponent_over_u32() {
    // Regression: `exp as u32` silently truncated; out-of-range now panics.
    let _ = <u64 as Number>::pow(2, 1u64 << 40);
}

// `powi`'s exact float result is not reproducible under Miri's intrinsic
// emulation; the `i32` exponent API is verified by compilation regardless.
#[cfg(not(miri))]
#[test]
fn powi_takes_i32() {
    assert_eq!(<f64 as Float>::powi(2.0, 10), 1024.0);
    assert_eq!(<f64 as Float>::powi(2.0, -1), 0.5);
}

#[test]
fn saturating_pow_saturates() {
    assert_eq!(<u8 as FiniteRangeNumber>::saturating_pow(2, 3), 8);
    assert_eq!(<u8 as FiniteRangeNumber>::saturating_pow(10, 3), u8::MAX);
}

#[test]
#[should_panic]
fn saturating_pow_panics_on_exponent_over_u32() {
    // Regression: `rhs as u32` silently truncated the exponent.
    let _ = <u64 as FiniteRangeNumber>::saturating_pow(2, 1u64 << 40);
}

#[cfg(feature = "half")]
#[test]
fn half_mul_add_is_single_rounding() {
    use half::f16;
    // Regression: `f16`/`bf16` used an unfused `(self * a) + b` (two roundings).
    // Under `std` they must now match a single f32 fused multiply-add rounded
    // once to `f16`.
    let b = f16::from_bits(0x3C05);
    let c = f16::from_bits(0x1234);
    let mut differed_from_unfused = false;
    for a_bits in 0x3C00u16..0x3E00 {
        let a = f16::from_bits(a_bits);
        let got = <f16 as Number>::mul_add(a, b, c);
        let fused = f16::from_f32(a.to_f32().mul_add(b.to_f32(), c.to_f32()));
        assert_eq!(got.to_bits(), fused.to_bits(), "a={}", a.to_f32());
        if (a * b + c).to_bits() != fused.to_bits() {
            differed_from_unfused = true;
        }
    }
    assert!(
        differed_from_unfused,
        "test inputs did not distinguish fused from unfused"
    );
}
