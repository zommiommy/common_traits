/**

Binary selection trait that makes it possible to implement traits differently on disjoint types.

The only two implementing types are [`True`] and [`False`].

This is used to
[circumvent a compiler limitation and implement traits differently
on disjoint types](https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983).

See [`IsAtomic`] for an example.

*/
pub trait BooleanSelector {}
/// [`BooleanSelector`] version of [`true`], this is an empty struct used only for
/// type system bounds
pub struct True {}
impl BooleanSelector for True {}
/// [`BooleanSelector`] version of [`false`], this is an empty struct used only for
/// type system bounds
pub struct False {}
impl BooleanSelector for False {}

/// A trait with an associated [`BooleanSelector`] type specifying whether the type is atomic.
/// It can be used to implement traits differently for atomic and non-atomic types.
/// See the `atomic_data` example.
pub trait IsAtomic {
    type Atomic: BooleanSelector;
}

/// A generic trait with an associated boolean, which can be used to do
/// specialization. See the example `atomic_data` for more information.
pub trait IsNonZero {
    type NonZero: BooleanSelector;
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

/// A trait with an associated [`BooleanSelector`] type specifying whether an integer type is signed.
/// It can be used to implement traits differently for signed and unsigned types.
/// See the `atomic_data` example.
pub trait IsSigned {
    type Signed: BooleanSelector;
}

/// A trait with an associated [`BooleanSelector`] type specifying whether a type is a float number.
/// It can be used to implement traits differently for float and non-float types.
/// See the `atomic_data` example.
pub trait IsFloat {
    type Float: BooleanSelector;
}

/// A trait with an associated [`BooleanSelector`] type specifying whether a type is an integer number.
/// It can be used to implement traits differently for integer and non-integer types.
/// See the `atomic_data` example.
pub trait IsInteger {
    type Integer: BooleanSelector;
}
