#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use anyhow::{Result, bail};

/// A trait for types that can be viewed as a sequence of copyable elements,
/// such as `&[T]`.
///
/// The difference between this and `AsRef<[T]>` is that the [`get`](`Sequence::get`) method doesn't
/// return a reference, but a copy of the element. This makes it possible to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &T, &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait Sequence {
    /// The type of the elements stored in the sequence.
    type Item: Copy;
    /// The type of the iterator returned by [`iter`](`Sequence::iter`).
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
        Self: 'a;

    /// Returns the length of the sequence.
    fn len(&self) -> usize;

    /// Returns the element of the sequence at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the sequence bounds.
    unsafe fn get_unchecked(&self, index: usize) -> Self::Item;

    /// Returns the element of the sequence at the given position, or an error if the
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

    /// Returns whether the sequence has length zero.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the elements of the sequence.
    fn iter(&self) -> Self::Iter<'_>;
}

/// A trait for types that can be viewed as a mutable sequence of copyable elements,
/// such as `&mut [T]`.
///
/// The difference between this and `AsMut<[T]>` is that the [`get`](`Sequence::get`) method doesn't
/// return a reference, but a copy of the element. This makes it possible to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait SequenceMut: Sequence {
    /// Sets the element of the sequence at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the sequence bounds.
    unsafe fn set_unchecked(&mut self, index: usize, value: Self::Item);

    /// Sets the element of the sequence at the given position.
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

/// A trait for types that can be viewed as a growable sequence of copyable elements,
/// such as `Vec<T>`.
///
/// The difference between this and `Vec<T>` is that the [`get`](`Sequence::get`) method doesn't
/// return a reference, but a copy of the element. This makes it possible to use
/// transparently compressed or succinct data structures as if they were slices.
#[impl_tools::autoimpl(for<T: trait + ?Sized> &mut T)]
#[cfg_attr(feature = "alloc", impl_tools::autoimpl(for<T: trait + ?Sized> Box<T>))]
pub trait SequenceGrowable: SequenceMut {
    /// Resizes the sequence to the given length, filling with the given value.
    fn resize(&mut self, new_len: usize, value: Self::Item);
    /// Pushes an element to the end of the sequence.
    fn push(&mut self, value: Self::Item);
    /// Removes the last element from the sequence and returns it, or `None` if it is empty.
    fn pop(&mut self) -> Option<Self::Item>;
    /// Sets the length to 0.
    fn clear(&mut self);
    /// Extends from another sequence.
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
