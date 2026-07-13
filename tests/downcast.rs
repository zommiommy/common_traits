use common_traits::DowncastableFrom;

/// Regression: the unsigned pointer-width downcast ladder implemented
/// `DowncastableFrom<isize> for u8` (a copy-paste typo) and had no
/// `usize -> u8`, so `usize` could not be downcast to `u8`.
#[test]
fn usize_downcasts_to_u8() {
    assert_eq!(
        <u8 as DowncastableFrom<usize>>::downcast_from(0x1234usize),
        0x34
    );
    assert_eq!(
        <u8 as DowncastableFrom<usize>>::downcast_from(255usize),
        255
    );
}
