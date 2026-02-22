use core::sync::atomic::Ordering;

use crate::{AtomicNumber, False, Integer, IsSigned, SignedInt, True, UnsignedInt};

/// An atomic integer type.
pub trait AtomicInteger: AtomicNumber
where
    Self::NonAtomicType: Integer,
{
    /// Bitwise “and” with the current value.
    ///
    /// Performs a bitwise “and” operation on the current value and the argument
    /// val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`AtomicInteger::fetch_and`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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

    /// Bitwise “nand” with the current value.
    ///
    /// Performs a bitwise “nand” operation on the current value and the
    /// argument val, and sets the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// [`AtomicInteger::fetch_nand`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    /// [`AtomicInteger::fetch_or`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
    /// [`AtomicInteger::fetch_xor`] takes an [`Ordering`](`core::sync::atomic::Ordering`) argument
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
}

/// An atomic signed integer type.
pub trait AtomicSignedInt: AtomicInteger + IsSigned<Signed = True>
where
    Self::NonAtomicType: SignedInt,
{
}

/// An atomic unsigned integer type.
pub trait AtomicUnsignedInt: AtomicInteger + IsSigned<Signed = False>
where
    Self::NonAtomicType: UnsignedInt,
{
}
