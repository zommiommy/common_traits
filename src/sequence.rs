#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use anyhow::{Result, bail};

/// A trait for types that can be viewed as a sequence of copiable elements,
/// such as `&[T]`.
///
/// The difference between this and `AsRef<[T]>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &T, &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait Sequence {
    /// The type of the elements stored in the Sequence.
    type Item: Copy;
    /// The type of the iterator returned by `iter`.
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
        Self: 'a;

    /// Return the length of the Sequence.
    fn len(&self) -> usize;

    /// Return the element of the Sequence at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the Sequence bounds.
    unsafe fn get_unchecked(&self, index: usize) -> Self::Item;

    /// Return the element of the Sequence at the given position, or `None` if the
    /// position is out of bounds.
    fn get(&self, index: usize) -> Result<Self::Item> {
        if index >= self.len() {
            bail!(
                "The index {} is out of bounds for the Sequence of length {}",
                index,
                self.len(),
            );
        }
        Ok(unsafe { self.get_unchecked(index) })
    }

    /// Return if the Sequence has length zero
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return an iterator over the elements of the Sequence.
    fn iter(&self) -> Self::Iter<'_>;
}

/// A trait for types that can be viewed as a mutable sequence of copiable elements,
/// such as `&mut [T]`.
///
/// The difference between this and `AsMut<[T]>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait SequenceMut: Sequence {
    /// Set the element of the Sequence at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the Sequence bounds.
    unsafe fn set_unchecked(&mut self, index: usize, value: Self::Item);

    /// Set the element of the Sequence at the given position
    fn set(&mut self, index: usize, value: Self::Item) -> Result<()> {
        if index >= self.len() {
            bail!(
                "The index {} is out of bounds for the Sequence of length {}",
                index,
                self.len(),
            );
        }
        unsafe { self.set_unchecked(index, value) };
        Ok(())
    }
}

/// A trait for types that can be viewed as a growable sequence of copiable elements,
/// such as `Vec<T>`.
///
/// The difference between this and `Vec<T>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait SequenceGrowable: SequenceMut {
    /// Resize the Sequence to the given length, filling with the given value.
    fn resize(&mut self, new_len: usize, value: Self::Item);
    /// Push an element to the end of the Sequence
    fn push(&mut self, value: Self::Item);
    /// Remove the last element from the Sequence and return it, or `None` if it is empty.
    fn pop(&mut self) -> Option<Self::Item>;
    /// Set len to 0
    fn clear(&mut self);
    /// Extend from another Sequence
    fn extend_from<S: Sequence<Item = Self::Item>>(&mut self, other: &S);
}

impl<T: Copy, const N: usize> Sequence for [T; N] {
    type Item = T;
    type Iter<'a>
        = core::iter::Copied<core::slice::Iter<'a, Self::Item>>
    where
        Self::Item: 'a,
        Self: 'a;
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            <[T; N]>::get_unchecked(self, index)
        }
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_ref().iter().copied()
    }
}

impl<T: Copy> Sequence for [T] {
    type Item = T;
    type Iter<'b>
        = core::iter::Copied<core::slice::Iter<'b, Self::Item>>
    where
        Self::Item: 'b,
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            *<[T]>::get_unchecked(self, index)
        }
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
}

impl<T: Copy, const N: usize> SequenceMut for [T; N] {
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            *self.get_unchecked_mut(index) = value;
        }
    }
}

impl<T: Copy> SequenceMut for [T] {
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            *<[T]>::get_unchecked_mut(self, index) = value;
        }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: Copy> Sequence for Vec<T> {
    type Item = T;
    type Iter<'a>
        = core::iter::Copied<core::slice::Iter<'a, Self::Item>>
    where
        Self::Item: 'a,
        Self: 'a;

    #[inline(always)]
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            *<[T]>::get_unchecked(self, index)
        }
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: Copy> SequenceMut for Vec<T> {
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        unsafe {
            debug_assert!(index < self.len(), "{} {}", index, self.len());
            *<[T]>::get_unchecked_mut(self, index) = value;
        }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: Copy> SequenceGrowable for Vec<T> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: Self::Item) {
        <Vec<T>>::resize(self, new_len, value);
    }
    #[inline(always)]
    fn push(&mut self, value: Self::Item) {
        <Vec<T>>::push(self, value);
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<Self::Item> {
        <Vec<T>>::pop(self)
    }
    #[inline(always)]
    fn clear(&mut self) {
        <Vec<T>>::clear(self);
    }
    #[inline(always)]
    fn extend_from<S: Sequence<Item = Self::Item>>(&mut self, other: &S) {
        for i in 0..other.len() {
            self.push(other.get(i).unwrap());
        }
    }
}
