use crate::{DowncastableFrom, FiniteRangeNumber, FromBytes, Integer, To, ToBytes, UpcastableFrom};

/// A trait to access a type with double the number of bits of Self.
pub trait DoubleType:
    Integer + FiniteRangeNumber + ToBytes + FromBytes + DowncastableFrom<Self::DoubleType>
{
    type DoubleType: HalfType<HalfType = Self>
        + UpcastableFrom<Self>
        + To<Self>
        + Integer
        + FiniteRangeNumber
        + ToBytes
        + FromBytes;
}

/// A trait to access a type with half the number of bits of Self.
pub trait HalfType:
    Integer + FiniteRangeNumber + ToBytes + FromBytes + UpcastableFrom<Self::HalfType>
{
    type HalfType: DoubleType<DoubleType = Self>
        + DowncastableFrom<Self>
        + To<Self>
        + Integer
        + FiniteRangeNumber
        + ToBytes
        + FromBytes;
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
