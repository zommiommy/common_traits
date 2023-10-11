use crate::{AtomicNumber, FiniteNumber};
use core::sync::atomic::*;

/// An atomic finite number type.
pub trait AtomicFiniteNumber: AtomicNumber
where
    Self::NonAtomicType: FiniteNumber,
{
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
