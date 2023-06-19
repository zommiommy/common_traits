/// Primitive cast between types using `as`
pub trait To<T> {
    fn to(self) -> T;
}

macro_rules! impl_to_inner {
    ($ty1:ty, $($ty:ty,)*) => {$(

impl To<$ty> for $ty1 {
    #[inline(always)]
    fn to(self) -> $ty {
        self as $ty
    }
}

    )*};
}

macro_rules! impl_to {
    ($($ty:ty,)*) => {$(

impl_to_inner!($ty,
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    f32, f64,
);

    )*};
}

impl_to!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64,);
