use crate::{False, True};
use crate::{IsAtomic, SameAs};
use core::sync::atomic::Ordering;

/// A trait for types that have an equivalent atomic type.
pub trait IntoAtomic: IsAtomic<Atomic = False> + Sized + Send + Sync {
    /// The atomic variant of the type.
    type AtomicType: Atomic<NonAtomicType = Self>;
    /// Converts `self` into the atomic variant of `Self`.
    fn to_atomic(self) -> Self::AtomicType;

    /// Converts an array of non-atomic values into an array of atomic values.
    fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N];
    /// Converts an array of atomic values into an array of non-atomic values.
    fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N];

    /// Converts a mutable slice of atomic values into a mutable slice of non-atomic values.
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self];
    /// Converts a mutable slice of non-atomic values into a mutable slice of atomic values.
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType];

    /// Converts a mutable reference to an array of atomic values into one of non-atomic values.
    fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N];
    /// Converts a mutable reference to an array of non-atomic values into one of atomic values.
    fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N];
}

/// Values that can be atomically read and written.
pub trait Atomic: IsAtomic<Atomic = True> + Sized + Send + Sync {
    /// The non-atomic variant of this type.
    type NonAtomicType: IntoAtomic<AtomicType = Self> + SameAs<Self>;

    /// Creates a new atomic value.
    fn new(value: Self::NonAtomicType) -> Self;
    /// Loads a value from the atomic.
    ///
    /// `load` takes an [`Ordering`](`core::sync::atomic::Ordering`) argument which describes
    /// the memory ordering of this operation.
    /// Possible values are [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Acquire`](`core::sync::atomic::Ordering::Acquire`) and [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// # Panics
    /// Panics if order is [`Release`](`core::sync::atomic::Ordering::Release`) or [`AcqRel`](`core::sync::atomic::Ordering::AcqRel`).
    fn load(&self, order: Ordering) -> Self::NonAtomicType;
    /// Stores a value into the atomic.
    ///
    /// `store` takes an [`Ordering`](`core::sync::atomic::Ordering`) argument which describes
    /// the memory ordering of this operation.
    /// Possible values are [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Release`](`core::sync::atomic::Ordering::Release`) and [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// # Panics
    /// Panics if order is [`Acquire`](`core::sync::atomic::Ordering::Acquire`) or
    /// [`AcqRel`](`core::sync::atomic::Ordering::AcqRel`).
    fn store(&self, value: Self::NonAtomicType, order: Ordering);
    /// Returns a mutable reference to the underlying value.
    ///
    /// This is safe because the mutable reference guarantees that no other
    /// threads are concurrently accessing the atomic data.
    fn get_mut(&mut self) -> &mut Self::NonAtomicType;
    /// Consumes the atomic and returns the contained value.
    ///
    /// This is safe because passing `self` by value guarantees that no other
    /// threads are concurrently accessing the atomic data.
    fn into_inner(self) -> Self::NonAtomicType;

    /// Converts an array of atomic values into an array of non-atomic values.
    fn into_non_atomic_array<const N: usize>(data: [Self; N]) -> [Self::NonAtomicType; N];
    /// Converts an array of non-atomic values into an array of atomic values.
    fn from_non_atomic_array<const N: usize>(data: [Self::NonAtomicType; N]) -> [Self; N];

    /// Returns a mutable slice of non-atomic values from a mutable slice of atomic values.
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType];
    /// Returns a mutable slice of atomic values from a mutable slice of non-atomic values.
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self];

    /// Returns a mutable array of non-atomic values from a mutable array of atomic values.
    fn get_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::NonAtomicType; N];
    /// Returns a mutable array of atomic values from a mutable array of non-atomic values.
    fn from_mut_array<const N: usize>(this: &mut [Self::NonAtomicType; N]) -> &mut [Self; N];

    /// Stores a value into the atomic if the current value is the same
    /// as the expected value.
    ///
    /// The return value is a result indicating whether the new value was
    /// written and containing the previous value. On success this value is
    /// guaranteed to be equal to current.
    ///
    /// [`compare_exchange`](`Atomic::compare_exchange`) takes two
    /// [`Ordering`](`core::sync::atomic::Ordering`)
    /// arguments to describe the memory ordering of this operation. success
    /// describes the required ordering for the read-modify-write operation that
    /// takes place if the comparison with current succeeds. failure describes
    /// the required ordering for the load operation that takes place when the
    /// comparison fails. Using [`Acquire`](`core::sync::atomic::Ordering::Acquire`)
    /// as success ordering makes the store part of this operation
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`), and
    /// using [`Release`](`core::sync::atomic::Ordering::Release`) makes the
    /// successful load [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    /// The failure ordering can only be [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Acquire`](`core::sync::atomic::Ordering::Acquire`) or
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// Note: This method is only available on platforms that support atomic
    /// operations on the given type.
    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>;

    /// Stores a value into the atomic if the current value is the same
    /// as the expected value.
    ///
    /// Unlike [`Atomic::compare_exchange`], this function is allowed to
    /// spuriously fail even when the comparison succeeds, which can result in
    /// more efficient code on some platforms. The return value is a result
    /// indicating whether the new value was written and containing the previous
    /// value.
    ///
    /// [`Atomic::compare_exchange_weak`] takes two
    /// [`Ordering`](`core::sync::atomic::Ordering`) arguments to describe the
    /// memory ordering of this operation. success describes the required
    /// ordering for the read-modify-write operation that takes place if the
    /// comparison with current succeeds. failure describes the required
    /// ordering for the load operation that takes place when the comparison
    /// fails. Using [`Acquire`](`core::sync::atomic::Ordering::Acquire`) as
    /// success ordering makes the store part of this operation
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`), and using
    /// [`Release`](`core::sync::atomic::Ordering::Release`) makes the
    /// successful load [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    /// The failure ordering can only be [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Acquire`](`core::sync::atomic::Ordering::Acquire`) or
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// Note: This method is only available on platforms that support atomic
    /// operations on the given type.
    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>;

    /// Stores a value into the atomic, returning the previous value.
    ///
    /// [`Atomic::swap`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
    /// which describes the memory ordering of this operation. All ordering
    /// modes are possible.
    /// Note that using [`Acquire`](`core::sync::atomic::Ordering::Acquire`)
    /// makes the store part of this operation
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`), and using
    /// [`Release`](`core::sync::atomic::Ordering::Release`) makes the load part
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// Note: This method is only available on platforms that support atomic
    /// operations on the given type.
    fn swap(&self, new: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a [`Result`](`core::result::Result`) of
    /// `Ok(previous_value)` if the function returned `Some(_)`, else
    /// `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been
    /// changed from other threads in the meantime, as long as the function
    /// returns `Some(_)`, but the function will have been applied only once to
    /// the stored value.
    ///
    /// [`Atomic::fetch_update`] takes two [`Ordering`](`core::sync::atomic::Ordering`)
    /// arguments to describe the memory ordering of this operation. The first
    /// describes the required ordering for when the operation finally succeeds
    /// while the second describes the required ordering for loads. These
    /// correspond to the success and failure orderings of
    /// [`Atomic::compare_exchange`] respectively.
    ///
    /// Using [`Acquire`](`core::sync::atomic::Ordering::Acquire`) as success
    /// ordering makes the store part of this operation
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`), and using
    /// [`Release`](`core::sync::atomic::Ordering::Release`) makes the final
    /// successful load
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    /// The failure ordering can only be
    /// [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Acquire`](`core::sync::atomic::Ordering::Acquire`) or
    /// [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// Note: This method is only available on platforms that support atomic
    /// operations on usize.
    ///
    /// # Considerations
    /// This method is not magic; it is not provided by the hardware. It is
    /// implemented in terms of [`Atomic::compare_exchange_weak`], and suffers
    /// from the same drawbacks. In particular, this method will not circumvent
    /// the ABA Problem.
    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>;
}
