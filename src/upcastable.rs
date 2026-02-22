/// [`UpcastableInto`] : [`UpcastableFrom`] = [`Into`] : [`From`]. It's easier
/// to use to specify bounds on generic variables.
pub trait UpcastableInto<W>: Sized {
    /// Calls `W::upcast_from(self)`.
    fn upcast(self) -> W;
}

/// Trait for primitive integers; the expected behavior for unsigned integers
/// is to zero-extend the value, while for signed integers it will sign-extend
/// it to the possibly bigger size.
pub trait UpcastableFrom<W>: Sized {
    /// Extends the current value to a possibly bigger size.
    fn upcast_from(value: W) -> Self;
}

/// UpcastableFrom implies UpcastableInto
impl<T, U> UpcastableInto<U> for T
where
    U: UpcastableFrom<T>,
{
    #[inline(always)]
    fn upcast(self) -> U {
        U::upcast_from(self)
    }
}

/// Reflexivity
impl<T> UpcastableFrom<T> for T {
    #[inline(always)]
    fn upcast_from(value: T) -> Self {
        value
    }
}

macro_rules! impl_upcasts {
    ($base_type:ty, $($ty:ty,)*) => {$(
impl UpcastableFrom<$base_type> for $ty {
    #[inline(always)]
    fn upcast_from(value: $base_type) -> Self {
        value as $ty
    }
}
    )*
    impl_upcasts!($($ty,)*);
};
    () => {};
}

impl_upcasts!(u8, u16, u32, u64, u128,);
impl_upcasts!(i8, i16, i32, i64, i128,);

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<i8> for isize {
    #[inline(always)]
    fn upcast_from(value: i8) -> Self {
        value as isize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<i16> for isize {
    #[inline(always)]
    fn upcast_from(value: i16) -> Self {
        value as isize
    }
}

#[cfg(target_pointer_width = "16")]
impl UpcastableFrom<isize> for i16 {
    #[inline(always)]
    fn upcast_from(value: isize) -> Self {
        value as i16
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64",))]
impl UpcastableFrom<i32> for isize {
    #[inline(always)]
    fn upcast_from(value: i32) -> Self {
        value as isize
    }
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32",))]
impl UpcastableFrom<isize> for i32 {
    #[inline(always)]
    fn upcast_from(value: isize) -> Self {
        value as i32
    }
}

#[cfg(target_pointer_width = "64")]
impl UpcastableFrom<i64> for isize {
    #[inline(always)]
    fn upcast_from(value: i64) -> Self {
        value as isize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<isize> for i64 {
    #[inline(always)]
    fn upcast_from(value: isize) -> Self {
        value as i64
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<isize> for i128 {
    #[inline(always)]
    fn upcast_from(value: isize) -> Self {
        value as i128
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<u8> for usize {
    #[inline(always)]
    fn upcast_from(value: u8) -> Self {
        value as usize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<u16> for usize {
    #[inline(always)]
    fn upcast_from(value: u16) -> Self {
        value as usize
    }
}

#[cfg(target_pointer_width = "16")]
impl UpcastableFrom<usize> for u16 {
    #[inline(always)]
    fn upcast_from(value: usize) -> Self {
        value as u16
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64",))]
impl UpcastableFrom<u32> for usize {
    #[inline(always)]
    fn upcast_from(value: u32) -> Self {
        value as usize
    }
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32",))]
impl UpcastableFrom<usize> for u32 {
    #[inline(always)]
    fn upcast_from(value: usize) -> Self {
        value as u32
    }
}

#[cfg(target_pointer_width = "64")]
impl UpcastableFrom<u64> for usize {
    #[inline(always)]
    fn upcast_from(value: u64) -> Self {
        value as usize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<usize> for u64 {
    #[inline(always)]
    fn upcast_from(value: usize) -> Self {
        value as u64
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
))]
impl UpcastableFrom<usize> for u128 {
    #[inline(always)]
    fn upcast_from(value: usize) -> Self {
        value as u128
    }
}

impl UpcastableFrom<f32> for f64 {
    #[inline(always)]
    fn upcast_from(value: f32) -> Self {
        value as f64
    }
}

#[cfg(feature = "half")]
mod half_impl {
    use super::*;
    impl UpcastableFrom<half::f16> for f32 {
        #[inline(always)]
        fn upcast_from(value: half::f16) -> Self {
            value.to_f32()
        }
    }
    impl UpcastableFrom<half::bf16> for f32 {
        #[inline(always)]
        fn upcast_from(value: half::bf16) -> Self {
            value.to_f32()
        }
    }
    impl UpcastableFrom<half::f16> for f64 {
        #[inline(always)]
        fn upcast_from(value: half::f16) -> Self {
            value.to_f64()
        }
    }
    impl UpcastableFrom<half::bf16> for f64 {
        #[inline(always)]
        fn upcast_from(value: half::bf16) -> Self {
            value.to_f64()
        }
    }
}
