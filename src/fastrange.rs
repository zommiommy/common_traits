/// Operation that maps a number between 0 and `n`.
///
/// # Reference
/// <https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/>
pub trait FastRange {
    fn fast_range(&self, n: Self) -> Self;
}

impl FastRange for u8 {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        ((*self as u16 * n as u16) >> 8) as u8
    }
}

impl FastRange for u16 {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        ((*self as u32 * n as u32) >> 16) as u16
    }
}

impl FastRange for u32 {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        ((*self as u64 * n as u64) >> 32) as u32
    }
}

impl FastRange for u64 {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        ((*self as u128 * n as u128) >> 64) as u64
    }
}

impl FastRange for u128 {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        // TODO!: is there a faster way to do this?
        self % n
    }
}

#[cfg(target_pointer_width = "8")]
impl FastRange for usize {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        (*self as u8).fast_range(n as u8) as usize
    }
}

#[cfg(target_pointer_width = "16")]
impl FastRange for usize {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        (*self as u16).fast_range(n as u16) as usize
    }
}

#[cfg(target_pointer_width = "32")]
impl FastRange for usize {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        (*self as u32).fast_range(n as u32) as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl FastRange for usize {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        (*self as u64).fast_range(n as u64) as usize
    }
}

#[cfg(target_pointer_width = "128")]
impl FastRange for usize {
    #[inline(always)]
    fn fast_range(&self, n: Self) -> Self {
        (*self as u128).fast_range(n as u128) as usize
    }
}
