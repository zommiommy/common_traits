use crate::{False, True};
use crate::{IsAtomic, SameAs};
use core::sync::atomic::Ordering;

/// Maps the ordering requested for a read-modify-write operation to a valid
/// *load* ordering for the observe/failure path of a `compare_exchange` loop.
/// `Release` and `AcqRel` are not valid load orderings (using them would panic),
/// so they are weakened to `Relaxed` and `Acquire`; the success store still uses
/// the original `order`.
#[inline(always)]
pub(crate) fn load_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Release => Ordering::Relaxed,
        Ordering::AcqRel => Ordering::Acquire,
        other => other,
    }
}

/// Emulates the standard-library `fetch_update` with a `load` +
/// `compare_exchange_weak` loop built on this crate's [`Atomic`] trait.
///
/// The inherent `fetch_update` on the standard atomics is deprecated on recent
/// toolchains (renamed to `try_update`), which does not exist on our MSRV, so
/// the loop is built from the always-available `load`/`compare_exchange_weak`
/// primitives. Semantics match the standard `fetch_update`: `f` is retried on
/// contention until it returns `None` (yielding `Err(prev)`) or the store
/// succeeds (yielding `Ok(prev)`).
#[inline]
pub(crate) fn fetch_update_loop<A: Atomic>(
    this: &A,
    set_order: Ordering,
    fetch_order: Ordering,
    mut f: impl FnMut(A::NonAtomicType) -> Option<A::NonAtomicType>,
) -> Result<A::NonAtomicType, A::NonAtomicType>
where
    A::NonAtomicType: Copy,
{
    let mut prev = this.load(fetch_order);
    while let Some(next) = f(prev) {
        match this.compare_exchange_weak(prev, next, set_order, fetch_order) {
            Ok(x) => return Ok(x),
            Err(x) => prev = x,
        }
    }
    Err(prev)
}

/// Reinterprets `&mut [P]` as `&mut [A]`, where `A` has the same size as `P` but
/// possibly stricter alignment (as for a primitive and its atomic counterpart).
///
/// # Panics
/// Panics if the slice's data is not aligned for `A`. This can only happen when
/// `align_of::<A>() > align_of::<P>()`, i.e. for 64-bit types on 32-bit x86,
/// where `align_of::<u64>() == 4` but `align_of::<AtomicU64>() == 8`.
#[inline(always)]
pub(crate) fn reinterpret_mut_slice<P, A>(this: &mut [P]) -> &mut [A] {
    debug_assert_eq!(core::mem::size_of::<P>(), core::mem::size_of::<A>());
    let len = this.len();
    let ptr = this.as_mut_ptr().cast::<A>();
    if len == 0 {
        // SAFETY: a zero-length slice only needs a non-null, well-aligned
        // pointer, which `NonNull::dangling` provides for `A`.
        return unsafe {
            core::slice::from_raw_parts_mut(core::ptr::NonNull::<A>::dangling().as_ptr(), 0)
        };
    }
    assert!(
        ptr.is_aligned(),
        "cannot reinterpret slice: data is not aligned to the atomic type (this happens for 64-bit types on 32-bit x86)"
    );
    // SAFETY: `A` has the same size as `P` (checked in debug), the data pointer
    // is aligned for `A` (asserted above), the length in elements is unchanged,
    // and the exclusive `&mut` borrow is preserved, so no aliasing is created.
    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}

/// Like [`reinterpret_mut_slice`] but for fixed-size arrays.
///
/// # Panics
/// See [`reinterpret_mut_slice`].
#[inline(always)]
pub(crate) fn reinterpret_mut_array<P, A, const N: usize>(this: &mut [P; N]) -> &mut [A; N] {
    debug_assert_eq!(core::mem::size_of::<P>(), core::mem::size_of::<A>());
    let ptr = this.as_mut_ptr().cast::<A>();
    if N == 0 {
        // SAFETY: a zero-length array is a ZST; `NonNull::dangling` yields a
        // non-null, well-aligned pointer for `[A; N]`.
        return unsafe { &mut *core::ptr::NonNull::<[A; N]>::dangling().as_ptr() };
    }
    assert!(
        ptr.is_aligned(),
        "cannot reinterpret array: data is not aligned to the atomic type (this happens for 64-bit types on 32-bit x86)"
    );
    // SAFETY: `A` has the same size as `P` (checked in debug), the pointer is
    // aligned for `A`/`[A; N]` (asserted above), and the exclusive `&mut` borrow
    // is preserved.
    unsafe { &mut *ptr.cast::<[A; N]>() }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // On a 64-bit host no real primitive/atomic pair has a stricter-aligned
    // atomic, so the alignment guard is driven directly with `P = [u8; 8]`
    // (alignment 1) reinterpreted as `u64` (alignment >= 2 on every target) from
    // a deliberately odd, hence misaligned, start address.

    #[test]
    #[should_panic(expected = "not aligned")]
    fn test_reinterpret_mut_slice_rejects_misaligned() {
        let mut buf = [0u8; 24];
        let base = buf.as_mut_ptr();
        // Pick an odd (hence u64-misaligned) start offset from the pointer's
        // address; `addr()` keeps strict provenance and involves no cast.
        let off = (base.addr() & 1) ^ 1;
        // SAFETY: `[u8; 8]` has alignment 1, so `base.add(off)` (off is 0 or 1)
        // is valid and aligned for `[u8; 8]`, and `off + 2 * 8 <= 24` keeps both
        // elements inside `buf`; the exclusive `buf` borrow backs the slice.
        let slice: &mut [[u8; 8]] =
            unsafe { core::slice::from_raw_parts_mut(base.add(off).cast(), 2) };
        // The data pointer is odd, so it is not aligned for `u64`: must panic.
        let _ = reinterpret_mut_slice::<[u8; 8], u64>(slice);
    }

    #[test]
    #[should_panic(expected = "not aligned")]
    fn test_reinterpret_mut_array_rejects_misaligned() {
        let mut buf = [0u8; 16];
        let base = buf.as_mut_ptr();
        // Force an odd (hence u64-misaligned) start offset (see above).
        let off = (base.addr() & 1) ^ 1;
        // SAFETY: `[u8; 8]` has alignment 1, so `base.add(off)` is valid and
        // aligned for `[[u8; 8]; 1]`, and `off + 8 <= 16` stays inside `buf`.
        let array: &mut [[u8; 8]; 1] = unsafe { &mut *base.add(off).cast::<[[u8; 8]; 1]>() };
        let _ = reinterpret_mut_array::<[u8; 8], u64, 1>(array);
    }

    #[test]
    fn test_fetch_update_loop_semantics() {
        use core::sync::atomic::{AtomicU32, Ordering};
        let a = AtomicU32::new(5);
        // `Some(_)` stores the new value and returns the previous one.
        let updated = fetch_update_loop(&a, Ordering::Relaxed, Ordering::Relaxed, |x| Some(x + 1));
        assert_eq!(updated, Ok(5));
        assert_eq!(a.load(Ordering::Relaxed), 6);
        // `None` leaves the value untouched and returns `Err(current)`.
        let unchanged = fetch_update_loop(&a, Ordering::Relaxed, Ordering::Relaxed, |_| None);
        assert_eq!(unchanged, Err(6));
        assert_eq!(a.load(Ordering::Relaxed), 6);
    }
}
