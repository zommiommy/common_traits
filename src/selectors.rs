/**

Binary selection trait  that make it possible to implement traits differently on disjoint types.

The only two implementing types are [`True`] and [`False`].

This is used to
[circumvent a compiler limitation and implement traits differently
on disjoint types](https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983).

See [`IsAtomic`] for an example.

*/
pub trait BooleanSelector {}
pub struct True {}
impl BooleanSelector for True {}
pub struct False {}
impl BooleanSelector for False {}

/// A trait with an associated [`Boolean`] type specifying whether the type is atomic.
/// It can be used to implement traits differently for atomic and non-atomic types.
/// See the `atomic_data` example.
pub trait IsAtomic {
    type Atomic: BooleanSelector;
}

/// A trait for types that have a fixed-length representation as a sequence of bytes.
/// This includes all standard numerical scalar types.
///
/// It is required that implementations of `AsRef<[u8]>` and `AsMut<[u8]>`
/// return a slice of length [`AsBytes::BYTES`].

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

/// A trait with an associated [`Boolean`] type specifying whether an integer type is signed.
/// It can be used to implement traits differently for signed and unsigned types.
/// See the `atomic_data` example.
pub trait IsSigned {
    type Signed: BooleanSelector;
}
