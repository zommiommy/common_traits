use crate::Boolean;
use crate::False;
use crate::Number;
use crate::Scalar;
use crate::True;
use core::sync::atomic::*;

/// A generic trait with an associated boolean, which can be used to do
/// specialization. See the example `atomic_data` for more information.
pub trait IsAtomic {
    type Atomic: Boolean;
}

/// A trait for numbers that can be atomically read and written
pub trait NonAtomic: IsAtomic<Atomic = False> + Sized + Send + Sync {
    /// The atomic variant of the number
    type AtomicType: Atomic<NonAtomicType = Self>;
    /// Convert `self` into the atomic variant of `Self`
    fn to_atomic(self) -> Self::AtomicType;

    /// Convert an array of non atomic values into an array of atomic values
    fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N];
    /// Convert an array of atomic values into an array of non atomic values
    fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N];

    /// Convert an slice of non atomic values into an slice of atomic values
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self];
    /// Convert an slice of atomic values into an slice of non atomic values
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType];

    /// Convert an array reference of non atomic values into an array reference of atomic values
    fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N];
    /// Convert an array reference of atomic values into an array reference of non atomic values
    fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N];
}

/// Values that can be atomically read and written
pub trait Atomic: IsAtomic<Atomic = True> + Sized + Send + Sync {
    /// The non atomic variant of this type
    type NonAtomicType: NonAtomic<AtomicType = Self>;

    /// Creates a new atomic integer.
    fn new(value: Self::NonAtomicType) -> Self;
    /// Loads a value from the atomic integer.
    ///
    /// load takes an [`Ordering`](`core::sync::atomic::Ordering`) argument which describes
    /// the memory ordering of this operation.
    /// Possible values are [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    ///[`Acquire`](`core::sync::atomic::Ordering::Acquire`) and [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// # Panics
    /// Panics if order is [`Release`](`core::sync::atomic::Ordering::Release`) or [`AcqRel`](`core::sync::atomic::Ordering::AcqRel`).
    fn load(&self, order: Ordering) -> Self::NonAtomicType;
    /// Stores a value into the atomic integer.
    /// load takes an [`Ordering`](`core::sync::atomic::Ordering`) argument which describes
    /// the memory ordering of this operation.
    /// Possible values are [`SeqCst`](`core::sync::atomic::Ordering::SeqCst`),
    /// [`Release`](`core::sync::atomic::Ordering::Release`) and [`Relaxed`](`core::sync::atomic::Ordering::Relaxed`).
    ///
    /// # Panics
    /// Panics if order is [`Acquire`](`core::sync::atomic::Ordering::Acquire`) or
    /// [`AcqRel`](`core::sync::atomic::Ordering::AcqRel`).
    fn store(&self, value: Self::NonAtomicType, order: Ordering);
    /// Returns a mutable reference to the underlying integer.
    ///
    /// This is safe because the mutable reference guarantees that no other
    /// threads are concurrently accessing the atomic data.
    fn get_mut(&mut self) -> &mut Self::NonAtomicType;
    /// Consumes the atomic and returns the contained value.
    ///
    /// This is safe because passing `self` by value guarantees that no other
    /// threads are concurrently accessing the atomic data.
    fn into_inner(self) -> Self::NonAtomicType;

    fn into_non_atomic_array<const N: usize>(data: [Self; N]) -> [Self::NonAtomicType; N];
    fn from_non_atomic_array<const N: usize>(data: [Self::NonAtomicType; N]) -> [Self; N];

    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType];
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self];

    fn get_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::NonAtomicType; N];
    fn from_mut_array<const N: usize>(this: &mut [Self::NonAtomicType; N]) -> &mut [Self; N];

    /// Stores a value into the atomic integer if the current value is the same
    /// as the current value.
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

    /// Stores a value into the atomic integer if the current value is the same
    /// as the current value.
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

    /// Stores a value into the atomic integer, returning the previous value.
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

    /// Bitwise “and” with the current value.
    ///
    /// Performs a bitwise “and” operation on the current value and the argument
    /// val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_and`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Maximum with the current value.
    ///
    /// Finds the maximum of the current value and the argument val, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_max`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Minimum with the current value.
    ///
    /// Finds the minimum of the current value and the argument val, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_min`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Bitwise “nand” with the current value.
    ///
    /// Performs a bitwise “nand” operation on the current value and the
    /// argument val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_nand`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Bitwise “or” with the current value.
    ///
    /// Performs a bitwise “or” operation on the current value and the argument val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_or`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Bitwise “xor” with the current value.
    ///
    /// Performs a bitwise “xor” operation on the current value and the argument val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`Atomic::fetch_xor`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;

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
    ///  arguments to describe the memory ordering of this operation. The first
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

/// An atomic number type.
pub trait AtomicNumber: Atomic + Scalar
where
    Self::NonAtomicType: Number,
{
    /// Adds to the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///
    /// [`fetch_add`][`AtomicNumber::fetch_add`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    /// Subtracts from the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///
    /// Returns the previous value.
    ///
    /// [`fetch_sub`][`AtomicNumber::fetch_sub`] an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    fn fetch_sub(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType;
    #[inline(always)]
    /// Adds to the current value, returning the previous value.
    ///
    /// This operation staturates at the bounds and does not
    /// overflow. For floats it saturets at the biggset non infinity value and
    /// NAN are just forwarded.
    ///
    /// This is a convenience method for [`fetch_update`](`Atomic::fetch_update`).
    fn fetch_saturating_add(
        &self,
        value: Self::NonAtomicType,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> Self::NonAtomicType {
        let mut base = self.load(fetch_order);
        loop {
            let new = base.saturating_add(value);
            let res = self.compare_exchange_weak(base, new, set_order, fetch_order);
            match res {
                Ok(val) => return val,
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline(always)]
    /// Subtract from the current value, returning the previous value.
    ///
    /// This operation staturates at the bounds and does not
    /// overflow. For floats it saturets at the biggset non infinity value and
    /// NAN are just forwarded.
    ///
    /// This is a convenience method for [`fetch_update`](`Atomic::fetch_update`).
    fn fetch_saturating_sub(
        &self,
        value: Self::NonAtomicType,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> Self::NonAtomicType {
        let mut base = self.load(fetch_order);
        loop {
            let new = base.saturating_sub(value);
            let res = self.compare_exchange_weak(base, new, set_order, fetch_order);
            match res {
                Ok(val) => return val,
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline(always)]
    /// This is a convenience method for [`fetch_update`](`Atomic::fetch_update`).
    fn fetch_saturating_mul(
        &self,
        value: Self::NonAtomicType,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> Self::NonAtomicType {
        let mut base = self.load(fetch_order);
        loop {
            let new = base.saturating_mul(value);
            let res = self.compare_exchange_weak(base, new, set_order, fetch_order);
            match res {
                Ok(val) => return val,
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline(always)]
    /// This is a convenience method for [`fetch_update`](`Atomic::fetch_update`).
    fn fetch_saturating_div(
        &self,
        value: Self::NonAtomicType,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> Self::NonAtomicType {
        let mut base = self.load(fetch_order);
        loop {
            let new = base.saturating_div(value);
            let res = self.compare_exchange_weak(base, new, set_order, fetch_order);
            match res {
                Ok(val) => return val,
                Err(val) => {
                    base = val;
                }
            }
        }
    }
    #[cfg(feature = "std")]
    #[inline(always)]
    /// This is a convenience method for [`fetch_update`](`Atomic::fetch_update`).
    fn fetch_saturating_pow(
        &self,
        value: Self::NonAtomicType,
        set_order: Ordering,
        fetch_order: Ordering,
    ) -> Self::NonAtomicType {
        let mut base = self.load(fetch_order);
        loop {
            let new = base.saturating_pow(value);
            let res = self.compare_exchange_weak(base, new, set_order, fetch_order);
            match res {
                Ok(val) => return val,
                Err(val) => {
                    base = val;
                }
            }
        }
    }
}
