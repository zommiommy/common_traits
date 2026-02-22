use crate::Atomic;
use crate::FiniteRangeNumber;
use crate::Number;
use core::sync::atomic::Ordering;

/// An atomic number type.
pub trait AtomicNumber: Atomic
where
    Self::NonAtomicType: Number,
{
    /// Adds to the current value, returning the previous value.
    ///
    /// This operation wraps around on overflow.
    ///
    /// [`fetch_add`][`AtomicNumber::fetch_add`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    /// [`fetch_sub`][`AtomicNumber::fetch_sub`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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

    /// Maximum with the current value.
    ///
    /// Finds the maximum of the current value and the argument val, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`AtomicNumber::fetch_max`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    /// [`AtomicNumber::fetch_min`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
}

/// An atomic finite number type.
pub trait AtomicFiniteRangeNumber: AtomicNumber
where
    Self::NonAtomicType: FiniteRangeNumber,
{
    #[inline(always)]
    /// Adds to the current value, returning the previous value.
    ///
    /// This operation saturates at the bounds and does not overflow. For floats
    /// it saturates at the biggest non-infinity value and NAN are just
    /// forwarded.
    ///
    /// This is a convenience method for
    /// [`fetch_update`](`Atomic::fetch_update`).
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
    /// This operation saturates at the bounds and does not
    /// overflow. For floats it saturates at the biggest non infinity value and
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
