use crate::{Bits, UpcastableFrom, DowncastableFrom};

/// A trait to access a type with double the number of bits.
pub trait DoubleType: Bits {
    type DoubleType: HalfType<HalfType=Self> + UpcastableFrom<Self> + Bits;
}

/// A trait to access a type with half the number of bits.
pub trait HalfType: Bits {
    type HalfType: DoubleType<DoubleType=Self> + DowncastableFrom<Self> + Bits;
}

macro_rules! impl_double_half {
    ($small:ty, $big:ty) => {
impl DoubleType for $small {
    type DoubleType = $big;
}
impl HalfType for $big {
    type HalfType = $small;
}
    };
    ($_:ty) => {};
    ($small:ty, $big:ty, $($tail:ty),*) => {
        impl_double_half!($small, $big);
        impl_double_half!($big, $($tail),*);
    };
}

impl_double_half!(u8, u16, u32, u64, u128);
impl_double_half!(i8, i16, i32, i64, i128);
impl_double_half!(f32, f64);