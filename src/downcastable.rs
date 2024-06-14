/// `DowncastableInto : DowncastableFrom = Into : From`, It's easier to use to
/// specify bounds on generic variables
pub trait DowncastableInto<W>: Sized {
    /// Call `W::downcast_from(self)`
    fn downcast(self) -> W;
}

/// Trait for primitive integers, the expected behaviour is to **truncate**
/// the bits in the UnsignedInt to the possibly smaller UnsignedInt size.
pub trait DowncastableFrom<W>: Sized {
    /// Truncate the current UnsignedInt to a possibly smaller size
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
impl_downcasts!(i128, i64, i32, i16, i8,);

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<isize> for i8 {
    #[inline(always)]
    fn downcast_from(value: isize) -> Self {
        value as i8
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<isize> for i16 {
    #[inline(always)]
    fn downcast_from(value: isize) -> Self {
        value as i16
    }
}
#[cfg(target_pointer_width = "16")]
impl DowncastableFrom<i16> for isize {
    #[inline(always)]
    fn downcast_from(value: i16) -> Self {
        value as isize
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64",))]
impl DowncastableFrom<isize> for i32 {
    #[inline(always)]
    fn downcast_from(value: isize) -> Self {
        value as i32
    }
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32",))]
impl DowncastableFrom<i32> for isize {
    #[inline(always)]
    fn downcast_from(value: i32) -> Self {
        value as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl DowncastableFrom<isize> for i64 {
    #[inline(always)]
    fn downcast_from(value: isize) -> Self {
        value as i64
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<i64> for isize {
    #[inline(always)]
    fn downcast_from(value: i64) -> Self {
        value as isize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<i128> for isize {
    #[inline(always)]
    fn downcast_from(value: i128) -> Self {
        value as isize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<isize> for u8 {
    #[inline(always)]
    fn downcast_from(value: isize) -> Self {
        value as u8
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl DowncastableFrom<usize> for u16 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u16
    }
}
#[cfg(target_pointer_width = "16")]
impl DowncastableFrom<u16> for usize {
    #[inline(always)]
    fn downcast_from(value: u16) -> Self {
        value as usize
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64",))]
impl DowncastableFrom<usize> for u32 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u32
    }
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32",))]
impl DowncastableFrom<u32> for usize {
    #[inline(always)]
    fn downcast_from(value: u32) -> Self {
        value as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl DowncastableFrom<usize> for u64 {
    #[inline(always)]
    fn downcast_from(value: usize) -> Self {
        value as u64
    }
}

#[cfg(any(
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

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
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
