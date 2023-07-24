/// `DowncastableInto : DowncastableFrom = Into : From`, It's easier to use to
/// specify bounds on generic variables
pub trait DowncastableInto<W>: Sized {
    /// Call `W::downcast_from(self)`
    fn downcast(self) -> W;
}

/// Trait for primitive integers, the expected behaviour is to **truncate**
/// the bits in the word to the possibly smaller word size.
pub trait DowncastableFrom<W>: Sized {
    /// Truncate the current word to a possibly smaller size
    fn downcast_from(value: W) -> Self;
}

/// DowncastableFrom implies DowncastableInto
impl<T, U> DowncastableInto<U> for T
where
    U: DowncastableFrom<T>,
{
    #[inline(always)]
    fn downcast(self) -> U {
        U::downcast_from(self)
    }
}

/// Riflexivity
impl<T> DowncastableFrom<T> for T {
    #[inline(always)]
    fn downcast_from(value: T) -> Self {
        value
    }
}

macro_rules! impl_downcasts {
    ($base_type:ty, $($ty:ty,)*) => {$(
impl DowncastableFrom<$base_type> for $ty {
    #[inline(always)]
    fn downcast_from(value: $base_type) -> Self {
        value as $ty
    }
}
    )*
    impl_downcasts!($($ty,)*);
};
    () => {};
}

impl_downcasts!(u128, u64, u32, u16, u8,);

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl DowncastableFrom<usize> for u8 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u8
    }
}

#[cfg(target_pointer_width = "8")]
impl DowncastableFrom<u8> for usize {
    #[inline(always)]
    fn downcast_from(value: u8) -> Self {
        value as usize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl DowncastableFrom<usize> for u16 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u16
    }
}
#[cfg(any(target_pointer_width = "8", target_pointer_width = "16",))]
impl DowncastableFrom<u16> for usize {
    #[inline(always)]
    fn downcast_from(value: u16) -> Self {
        value as usize
    }
}

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl DowncastableFrom<usize> for u32 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u32
    }
}

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
))]
impl DowncastableFrom<u32> for usize {
    #[inline(always)]
    fn downcast_from(value: u32) -> Self {
        value as usize
    }
}

#[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
impl DowncastableFrom<usize> for u64 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u64
    }
}

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<u64> for usize {
    #[inline(always)]
    fn downcast_from(value: u64) -> Self {
        value as usize
    }
}

#[cfg(target_pointer_width = "128")]
impl DowncastableFrom<usize> for u128 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u128
    }
}

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl DowncastableFrom<u128> for usize {
    #[inline(always)]
    fn downcast_from(value: u128) -> Self {
        value as usize
    }
}

impl DowncastableFrom<f64> for f32 {
    #[inline(always)]
    fn downcast_from(value: f64) -> Self {
        value as f32
    }
}

#[cfg(feature = "half")]
mod half_impl {
    use super::*;
    impl DowncastableFrom<f32> for half::f16 {
        #[inline(always)]
        fn downcast_from(value: f32) -> Self {
            half::f16::from_f32(value)
        }
    }
    impl DowncastableFrom<f32> for half::bf16 {
        #[inline(always)]
        fn downcast_from(value: f32) -> Self {
            half::bf16::from_f32(value)
        }
    }
    impl DowncastableFrom<f64> for half::f16 {
        #[inline(always)]
        fn downcast_from(value: f64) -> Self {
            half::f16::from_f64(value)
        }
    }
    impl DowncastableFrom<f64> for half::bf16 {
        #[inline(always)]
        fn downcast_from(value: f64) -> Self {
            half::bf16::from_f64(value)
        }
    }
}
