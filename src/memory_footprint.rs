#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Like `core::mem::size_of()` but also for complex objects
pub trait MemSize {
    /// Memory Owned, i.e. how much data is copied on Clone
    fn mem_size(&self) -> usize;
    /// Memory Owned + Borrowed, i.e. also slices sizes
    fn mem_used(&self) -> usize;
}

macro_rules! impl_memory_size {
    ($($ty:ty),*) => {$(
impl MemSize for $ty {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
    #[inline(always)]
    fn mem_used(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
    )*};
}

#[cfg(not(feature = "half"))]
impl_memory_size! {
    bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
}
#[cfg(feature = "half")]
impl_memory_size! {
    bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64, half::f16, half::bf16
}

impl<'a, T: MemSize> MemSize for &'a [T] {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
    #[inline(always)]
    fn mem_used(&self) -> usize {
        self.mem_size() + self.iter().map(|x| x.mem_used()).sum::<usize>()
    }
}

impl<'a, T: MemSize> MemSize for &'a mut [T] {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
    #[inline(always)]
    fn mem_used(&self) -> usize {
        self.mem_size() + self.iter().map(|x| x.mem_used()).sum::<usize>()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: MemSize> MemSize for Vec<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
    #[inline(always)]
    fn mem_used(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_used()).sum::<usize>()
    }
}
