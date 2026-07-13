//! Regression tests for numeric trait method fixes.
use common_traits::{Integer, Number, UnsignedInt};

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
