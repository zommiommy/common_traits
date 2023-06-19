#[cfg(feature="alloc")]
use alloc::vec::Vec;

pub trait Slice {
    type Item: Copy;

    /// Return the length of the slice.
    fn len(&self) -> usize;

    /// Return the element of the slice at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the slice bounds.
    unsafe fn get_unchecked(&self, index: usize) -> Self::Item;

    /// Return the element of the slice at the given position, or `None` if the
    /// position is out of bounds.
    fn get(&self, index: usize) -> Option<Self::Item> {
        if index >= self.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }
    /// Return if the slice has length zero
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SliceMut: Slice {
    /// Set the element of the slice at the given position, without
    /// doing any bounds checking.
    ///
    /// # Safety
    ///
    /// Must not be called with `index` out of the slice bounds.
    unsafe fn set_unchecked(&mut self, index: usize, value: Self::Item);

    fn set(&mut self, index: usize, value: Self::Item) -> Option<()> {
        if index >= self.len() {
            None
        } else {
            unsafe { self.set_unchecked(index, value) };
            Some(())
        }
    }
}

pub trait SliceGrowable: SliceMut {
    fn resize(&mut self, new_len: usize, value: Self::Item);
    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn clear(&mut self);
    fn extend_from<S: Slice<Item=Self::Item>>(&mut self, other: &S);
}

impl<'a, T: Copy> Slice for &'a [T] {
    type Item = T;
    #[inline(always)]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
}

impl<'a, T: Copy> Slice for &'a mut [T] {
    type Item = T;
    #[inline(always)]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
}

impl<'a, T: Copy> SliceMut for &'a mut [T] {
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked_mut(self, index) = value;
    }
}

#[cfg(any(feature="alloc", feature="std"))]
impl<T: Copy> Slice for Vec<T> {
    type Item = T;
    #[inline(always)]
    fn len(&self) -> usize {
        <Vec<T>>::len(self)
    }
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked(self, index)
    }
}

#[cfg(any(feature="alloc", feature="std"))]
impl<T: Copy> SliceMut for Vec<T> {
    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len(), "{} {}", index, self.len());
        *<[T]>::get_unchecked_mut(self, index) = value;
    }
}

#[cfg(any(feature="alloc", feature="std"))]
impl<T: Copy> SliceGrowable for Vec<T> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: Self::Item){
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
    fn extend_from<S: Slice<Item=Self::Item>>(&mut self, other: &S)
    {
        for i in 0..other.len() {
            self.push(other.get(i).unwrap());
        }
    }
}