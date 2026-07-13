use common_traits::Sequence;

/// Regression: the `[T; N]` impl of `Sequence::get_unchecked` used to call
/// `<[T; N]>::get_unchecked`, which resolves to the trait method itself and
/// recurses until the stack overflows. Any `Sequence::get`/`get_unchecked` over
/// an array must simply return the element.
#[test]
fn test_array_get_does_not_recurse() {
    let a = [10u8, 20, 30, 40];
    assert_eq!(Sequence::get(&a, 0).unwrap(), 10);
    assert_eq!(Sequence::get(&a, 3).unwrap(), 40);
    assert!(Sequence::get(&a, 4).is_err());
    // SAFETY: index 2 is within the length-4 array.
    assert_eq!(unsafe { Sequence::get_unchecked(&a, 2) }, 30);
}
