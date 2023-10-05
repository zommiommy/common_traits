use crate::{Boolean, False, True};

/// A generic trait with an associated boolean, which can be used to do
/// specialization. See the example `atomic_data` for more information.
pub trait IsNonZero {
    type NonZero: Boolean;
}

/// Non zero variants of primitives types for enum optimizations
pub trait NonZero: IsNonZero<NonZero = True> + Sized {
    type BaseType: IsNonZero<NonZero = False>;

    /// Creates a non-zero without checking whether the value is non-zero. This
    /// results in undefined behaviour if the value is zero.
    /// # Safety
    /// The value must not be zero.
    unsafe fn new_unchecked(n: Self::BaseType) -> Self;

    /// Creates a non-zero if the given value is not zero.
    fn new(n: Self::BaseType) -> Option<Self>;

    /// Returns the value as a primitive type.
    fn get(self) -> Self::BaseType;
}
