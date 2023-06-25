#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use anyhow::{bail, Result};
use core::ops::Range;

/// A trait for types that can be viewed as a sequence of copiable elements,
/// such as `&[T]`.
///
/// The difference between this and `AsRef<[T]>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succint data structures as if they were slices.
pub trait Sequence {
    /// The type of the elements stored in the Sequence.
    type Item: Copy;
    /// The type of the iterator returned by `iter`.
    type Iter<'a>: Iterator<Item = Self::Item>
    where
        Self::Item: 'a,
        Self: 'a;
    /// The type that acts as a subset of the sequence
    type Range<'a>: Sequence<Range<'a> = Self::Range<'a>> + 'a
    where
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

    /// Return a subset of the sequence
    fn get_range(&self, range: Range<usize>) -> Result<Self::Range<'_>> {
        if range.end >= self.len() || range.start > range.end {
            bail!(
                "The range {}..{} is out of bounds for the Sequence of length {} or is inconsistent.",
                range.start,
                range.end,
                self.len(),
            );
        }
        Ok(unsafe { self.get_range_unchecked(range) })
    }

    /// Return a subset of the sequence
    ///
    /// # Safety
    /// The range must be a subset of `0..self.len()`
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_>;
}

/// A trait for types that can be viewed as a mutable sequence of copiable elements,
/// such as `&mut [T]`.
///
/// The difference between this and `AsMut<[T]>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succint data structures as if they were slices.
pub trait SequenceMut: Sequence {
    /// The type that acts as a subset of the mutable sequence
    type RangeMut<'a>: SequenceMut<RangeMut<'a> = Self::RangeMut<'a>> + 'a
    where
        Self: 'a;

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

    /// Return a subset of the mutable sequence
    fn get_range_mut(&mut self, range: Range<usize>) -> Result<Self::RangeMut<'_>> {
        if range.end >= self.len() || range.start > range.end {
            bail!(
                "The range {}..{} is out of bounds for the SequenceMut of length {} or is inconsistent.",
                range.start,
                range.end,
                self.len(),
            );
        }
        Ok(unsafe { self.get_range_mut_unchecked(range) })
    }

    /// Return a subset of the mutable sequence
    ///
    /// # Safety
    /// The range must be a subset of `0..self.len()`
    unsafe fn get_range_mut_unchecked(&mut self, range: Range<usize>) -> Self::RangeMut<'_>;
}

/// A trait for types that can be viewed as a growable sequence of copiable elements,
/// such as `Vec<T>`.
///
/// The difference between this and `Vec<T>` is that the get method doesn't
/// return a reference, but a copy of the element. This allows to use
/// transparently compressed or succint data structures as if they were slices.
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
    type Iter<'a> = core::iter::Copied<core::slice::Iter<'a, Self::Item>> where Self::Item: 'a, Self: 'a;
    type Range<'a> = &'a [T]
        where
            Self: 'a;
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        <[T; N]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_ref().iter().copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self.as_slice()[range]
    }
}

impl<'a, T: Copy> Sequence for &'a [T] {
    type Item = T;
    type Iter<'b> = core::iter::Copied<core::slice::Iter<'b, Self::Item>> where Self::Item: 'b, Self: 'b;
    type Range<'b> = &'b [T]
    where
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self[range]
    }
}

impl<'a, T: Copy> Sequence for &'a mut [T] {
    type Item = T;
    type Iter<'b> = core::iter::Copied<core::slice::Iter<'b, Self::Item>> where Self::Item: 'b, Self: 'b;
    type Range<'b> = &'b [T]
    where
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self[range]
    }
}

impl<T: Copy, const N: usize> SequenceMut for [T; N] {
    type RangeMut<'b> = &'b mut [T]
    where
        Self: 'b;
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *self.get_unchecked_mut(index) = value;
    }
    #[inline(always)]
    unsafe fn get_range_mut_unchecked(&mut self, range: Range<usize>) -> Self::RangeMut<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &mut self[range]
    }
}

impl<'a, T: Copy> SequenceMut for &'a mut [T] {
    type RangeMut<'b> = &'b mut [T]
    where
        Self: 'b;
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked_mut(self, index) = value;
    }
    #[inline(always)]
    unsafe fn get_range_mut_unchecked(&mut self, range: Range<usize>) -> Self::RangeMut<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &mut self[range]
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: Copy> Sequence for Vec<T> {
    type Item = T;
    type Iter<'a> = core::iter::Copied<core::slice::Iter<'a, Self::Item>> where Self::Item: 'a, Self: 'a;
    type Range<'b> = &'b [T]
    where
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self[range]
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a, T: Copy> Sequence for &'a Vec<T> {
    type Item = T;
    type Iter<'b> = core::iter::Copied<core::slice::Iter<'b, Self::Item>> where Self::Item: 'b, Self: 'b;
    type Range<'b> = &'b [T]
    where
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self[range]
    }
}
#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a, T: Copy> Sequence for &'a mut Vec<T> {
    type Item = T;
    type Iter<'b> = core::iter::Copied<core::slice::Iter<'b, Self::Item>> where Self::Item: 'b, Self: 'b;
    type Range<'b> = &'b [T]
    where
        Self: 'b;
    #[inline(always)]
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self).copied()
    }
    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: Range<usize>) -> Self::Range<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &self[range]
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: Copy> SequenceMut for Vec<T> {
    type RangeMut<'b> = &'b mut [T]
    where
        Self: 'b;
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked_mut(self, index) = value;
    }
    #[inline(always)]
    unsafe fn get_range_mut_unchecked(&mut self, range: Range<usize>) -> Self::RangeMut<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &mut self[range]
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a, T: Copy> SequenceMut for &'a mut Vec<T> {
    type RangeMut<'b> = &'b mut [T]
    where
        Self: 'b;
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked_mut(self, index) = value;
    }
    #[inline(always)]
    unsafe fn get_range_mut_unchecked(&mut self, range: Range<usize>) -> Self::RangeMut<'_> {
        debug_assert!(range.end <= self.len(), "{} {}", range.end, self.len());
        debug_assert!(range.start <= range.end, "{} {}", range.start, range.end);
        &mut self[range]
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

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a, T: Copy> SequenceGrowable for &'a mut Vec<T> {
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
