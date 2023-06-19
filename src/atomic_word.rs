use crate::Word;
use core::sync::atomic::*;

/// Values that can be atomically read and written
pub trait AtomicWord: Sized + Send + Sync {
    /// The non atomic variant of this type
    type NonAtomicWord: Word<AtomicWord = Self>;

    fn new(value: Self::NonAtomicWord) -> Self;
    fn load(&self, order: Ordering) -> Self::NonAtomicWord;
    fn store(&self, value: Self::NonAtomicWord, order: Ordering);
    fn get_mut(&mut self) -> &mut Self::NonAtomicWord;
    fn into_inner(self) -> Self::NonAtomicWord;

    #[cfg(feature = "atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicWord];
    #[cfg(feature = "atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self::NonAtomicWord]) -> &mut [Self];

    fn compare_exchange(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>;

    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>;

    fn swap(&self, new: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;

    fn fetch_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_saturating_add(
        &self,
        value: Self::NonAtomicWord,
        order: Ordering,
    ) -> Self::NonAtomicWord;
    fn fetch_and(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_max(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_min(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_nand(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_or(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_sub(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_xor(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;

    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>
    where
        F: FnMut(Self::NonAtomicWord) -> Option<Self::NonAtomicWord>;
}
