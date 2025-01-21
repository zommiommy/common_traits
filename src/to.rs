/// Primitive cast between types using `as`
pub trait To<T> {
    fn to(self) -> T;
}

/// blanket implementation to ensure reflexive `To` is the identity function
impl<T> To<T> for T {
    #[inline(always)]
    fn to(self) -> Self {
        self
    }
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

impl_to!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize,);

#[cfg(feature = "half")]
mod half_impl {
    use super::*;

    macro_rules! impl_to_half {
        ($ty1:ty, $($ty:ty,)*) => {

    impl To<half::f16> for $ty1 {
        #[inline(always)]
        fn to(self) -> half::f16 {
            (self as f32).to()
        }
    }
    impl To<half::bf16> for $ty1 {
        #[inline(always)]
        fn to(self) -> half::bf16 {
            (self as f32).to()
        }
    }
    impl To<$ty1> for half::f16 {
        #[inline(always)]
        fn to(self) -> $ty1 {
            self.to_f32().to()
        }
    }
    impl To<$ty1> for half::bf16 {
        #[inline(always)]
        fn to(self) -> $ty1 {
            self.to_f32().to()
        }
    }

    impl_to_half!($($ty,)*);
        };
        () => {};
    }

    impl_to_half!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize,);

    impl To<half::f16> for f32 {
        #[inline(always)]
        fn to(self) -> half::f16 {
            half::f16::from_f32(self)
        }
    }
    impl To<half::bf16> for f32 {
        #[inline(always)]
        fn to(self) -> half::bf16 {
            half::bf16::from_f32(self)
        }
    }
    impl To<half::f16> for f64 {
        #[inline(always)]
        fn to(self) -> half::f16 {
            half::f16::from_f64(self)
        }
    }
    impl To<half::bf16> for f64 {
        #[inline(always)]
        fn to(self) -> half::bf16 {
            half::bf16::from_f64(self)
        }
    }
    impl To<f32> for half::f16 {
        #[inline(always)]
        fn to(self) -> f32 {
            self.to_f32()
        }
    }
    impl To<f32> for half::bf16 {
        #[inline(always)]
        fn to(self) -> f32 {
            self.to_f32()
        }
    }
    impl To<f64> for half::f16 {
        #[inline(always)]
        fn to(self) -> f64 {
            self.to_f64()
        }
    }
    impl To<f64> for half::bf16 {
        #[inline(always)]
        fn to(self) -> f64 {
            self.to_f64()
        }
    }
}
