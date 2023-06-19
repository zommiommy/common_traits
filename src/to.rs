/// Primitive cast between types using `as`
pub trait To<T> {
    fn to(self) -> T;
}

macro_rules! impl_to {
    ($ty1:ty, $($ty:ty,)*) => {
$(
    impl To<$ty> for $ty1 {
        #[inline(always)]
        fn to(self) -> $ty {
            self as $ty
        }
    }
    impl To<$ty1> for $ty {
        #[inline(always)]
        fn to(self) -> $ty1 {
            self as $ty1
        }
    }
)*

impl_to!($($ty,)*);

};
    () => {};
}

impl_to!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64,);

#[cfg(feature="half")]
impl To<half::f16> for f32 {
    #[inline(always)]
    fn to(self) -> half::f16 {
        half::f16::from_f32(self)
    }
}
#[cfg(feature="half")]
impl To<half::bf16> for f32 {
    #[inline(always)]
    fn to(self) -> half::bf16 {
        half::bf16::from_f32(self)
    }
}
#[cfg(feature="half")]
impl To<half::f16> for f64 {
    #[inline(always)]
    fn to(self) -> half::f16 {
        half::f16::from_f64(self)
    }
}
#[cfg(feature="half")]
impl To<half::bf16> for f64 {
    #[inline(always)]
    fn to(self) -> half::bf16 {
        half::bf16::from_f64(self)
    }
}
#[cfg(feature="half")]
impl To<f32> for half::f16 {
    #[inline(always)]
    fn to(self) -> f32 {
        self.to_f32()
    }
}
#[cfg(feature="half")]
impl To<f32> for half::bf16 {
    #[inline(always)]
    fn to(self) -> f32 {
        self.to_f32()
    }
}
#[cfg(feature="half")]
impl To<f64> for half::f16 {
    #[inline(always)]
    fn to(self) -> f64 {
        self.to_f64()
    }
}
#[cfg(feature="half")]
impl To<f64> for half::bf16 {
    #[inline(always)]
    fn to(self) -> f64 {
        self.to_f64()
    }
}