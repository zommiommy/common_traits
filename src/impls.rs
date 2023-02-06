use crate::*;
use crate::simd::*;
use core::sync::atomic::*;

macro_rules! impl_Number {
    ($ty:ty) => {
        
impl ISimd for $ty {}

impl Number for $ty {
    const BITS: usize = <$ty>::BITS as _;
    const BYTES: usize = core::mem::size_of::<$ty>() as _;
    type BytesForm = [u8; core::mem::size_of::<$ty>()];
    const MIN: Self = <$ty>::MIN as _;
    const MAX: Self = <$ty>::MAX as _;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline(always)]
    fn from_be_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_be_bytes(bytes)}
    #[inline(always)]
    fn from_le_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_le_bytes(bytes)}
    #[inline(always)]
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_ne_bytes(bytes)}
    #[inline(always)]
    fn to_be_bytes(self) -> Self::BytesForm{self.to_be_bytes()}
    #[inline(always)]
    fn to_le_bytes(self) -> Self::BytesForm{self.to_le_bytes()}
    #[inline(always)]
    fn to_ne_bytes(self) -> Self::BytesForm{self.to_ne_bytes()}
}

impl Integer for $ty {
    #[inline(always)]
    fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs)}
    #[inline(always)]
    fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs)}
    #[inline(always)]
    fn to_le(self) -> Self{self.to_le()}
    #[inline(always)]
    fn swap_bytes(self) -> Self{self.swap_bytes()}
    #[inline(always)]
    fn to_be(self) -> Self{self.to_be()}
    #[inline(always)]
    fn from_le(rhs: Self) -> Self {<$ty>::from_le(rhs)}
    #[inline(always)]
    fn from_be(rhs: Self) -> Self {<$ty>::from_be(rhs)}

    #[inline(always)]
    fn extract_bit(&self, bit: usize) -> bool {
        debug_assert!(bit < core::mem::size_of::<$ty>() * 8);
        let mask: $ty = 1 << bit;
        (self & mask) != 0
    }

    #[inline(always)]
    fn extract_bitfield(&self, start_bit: usize, end_bit: usize) -> Self {
        debug_assert!(start_bit < end_bit);
        debug_assert!(end_bit <= core::mem::size_of::<$ty>() * 8);
        let n_bits = core::mem::size_of::<$ty>() * 8;
        let mask: $ty = <$ty>::MAX >> (n_bits - (end_bit - start_bit));
        (self >> start_bit) & mask
    }

    #[inline(always)]
    fn checked_add(self, rhs: Self) -> Option<Self>{self.checked_add(rhs)}
    #[inline(always)]
    fn checked_div(self, rhs: Self) -> Option<Self>{self.checked_div(rhs)}
    #[inline(always)]
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>{self.checked_div_euclid(rhs)}
    #[inline(always)]
    fn checked_mul(self, rhs: Self) -> Option<Self>{self.checked_mul(rhs)}
    #[inline(always)]
    fn checked_neg(self) -> Option<Self>{self.checked_neg()}
    #[inline(always)]
    fn checked_pow(self, exp: u32) -> Option<Self>{self.checked_pow(exp)}
    #[inline(always)]
    fn checked_rem(self, rhs: Self) -> Option<Self>{self.checked_rem(rhs)}
    #[inline(always)]
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>{self.checked_rem_euclid(rhs)}
    #[inline(always)]
    fn checked_shl(self, rhs: u32) -> Option<Self>{self.checked_shl(rhs)}
    #[inline(always)]
    fn checked_shr(self, rhs: u32) -> Option<Self>{self.checked_shr(rhs)}
    #[inline(always)]
    fn checked_sub(self, rhs: Self) -> Option<Self>{self.checked_sub(rhs)}
    #[inline(always)]
    fn count_ones(self) -> u32{self.count_ones()}
    #[inline(always)]
    fn count_zeros(self) -> u32{self.count_zeros()}
    #[inline(always)]
    fn leading_ones(self) -> u32{self.leading_ones()}
    #[inline(always)]
    fn leading_zeros(self) -> u32{self.leading_zeros()}
    #[inline(always)]
    fn pow(self, exp: u32) -> Self{self.pow(exp)}
    #[inline(always)]
    fn reverse_bits(self) -> Self{self.reverse_bits()}
    #[inline(always)]
    fn rotate_left(self, rhs: u32) -> Self { self.rotate_left(rhs)}
    #[inline(always)]
    fn rotate_right(self, rhs: u32) -> Self { self.rotate_right(rhs)}
    #[inline(always)]
    fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs)}
    #[inline(always)]
    fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs)}
    #[inline(always)]
    fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs)}
    #[inline(always)]
    fn saturating_pow(self, rhs: u32) -> Self { self.saturating_pow(rhs)}
    #[inline(always)]
    fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs)}
    #[inline(always)]
    fn trailing_ones(self) -> u32{self.trailing_ones()}
    #[inline(always)]
    fn trailing_zeros(self) -> u32{self.trailing_zeros()}


    #[inline(always)]
    fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs)}
    #[inline(always)]
    fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs)}
    #[inline(always)]
    fn wrapping_div_euclid(self, rhs: Self) -> Self { self.wrapping_div_euclid(rhs)}
    #[inline(always)]
    fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs)}
    #[inline(always)]
    fn wrapping_neg(self) -> Self { self.wrapping_neg()}
    #[inline(always)]
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp)}
    #[inline(always)]
    fn wrapping_rem(self, rhs: Self) -> Self { self.wrapping_rem(rhs)}
    #[inline(always)]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self { self.wrapping_rem_euclid(rhs)}
    #[inline(always)]
    fn wrapping_shl(self, exp: u32) -> Self { self.wrapping_shl(exp)}
    #[inline(always)]
    fn wrapping_shr(self, exp: u32) -> Self { self.wrapping_shr(exp)}
    #[inline(always)]
    fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs)}
}

    };
}

macro_rules! impl_word {
    ($ty:ty, $sty:ty, $aty:ty, $saty:ty, $nzty:ty, $nzsty:ty) => {

impl_Number!($ty);
impl_Number!($sty);

impl WSimd for $ty {}

impl Word for $ty {
    type SignedWord = $sty;
    type AtomicWord = $aty;
    type NonZeroWord = $nzty;


    #[inline(always)]
    fn to_signed(self) -> Self::SignedWord {self as Self::SignedWord}
    #[inline(always)]
    fn to_atomic(self) -> Self::AtomicWord {Self::AtomicWord::new(self)}

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::Atomic]) -> &mut [Self]{
        <$aty>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::Atomic]{
        <$aty>::from_mut_slice(this)
    }

    #[inline(always)]
    fn abs_diff(self, rhs: Self) -> Self { self.abs_diff(rhs)}

    #[inline(always)]
    fn checked_next_power_of_two(self) -> Option<Self>{self.checked_next_power_of_two()}

    #[inline(always)]
    fn overflow_shl(self, rhs: Self) -> Self { 
        self.checked_shl(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    #[inline(always)]
    fn overflow_shr(self, rhs: Self) -> Self {
        self.checked_shr(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    #[inline(always)]
    fn overflow_sar(self, rhs: Self) -> Self {
        let shift_amount = core::cmp::min(rhs, Self::BITS as Self - 1);
        ((self as Self::SignedWord) >> shift_amount) as Self
    }

    #[inline(always)]
    fn sign_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (((self << shift_amount) as Self::SignedWord) >> shift_amount) as Self
    }

    #[inline(always)]
    fn zero_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (self << shift_amount) >> shift_amount
    }

    #[inline(always)]
    fn checked_add_signed(self, rhs: Self::SignedWord) -> Option<Self>{self.checked_add_signed(rhs)}
    #[inline(always)]
    fn saturating_add_signed(self, rhs: Self::SignedWord) -> Self{self.saturating_add_signed(rhs)}
    #[inline(always)]
    fn wrapping_add_signed(self, rhs: Self::SignedWord) -> Self{self.wrapping_add_signed(rhs)}
    #[inline(always)]
    fn is_power_of_two(self) -> bool{self.is_power_of_two()}
    #[inline(always)]
    fn next_power_of_two(self) -> Self{self.next_power_of_two()}
}

impl SSimd for $sty {}

impl SignedWord for $sty {
    type UnsignedWord = $ty;
    type NonZeroWord = $nzsty;

    #[inline(always)]
    fn to_unsigned(self) -> Self::UnsignedWord {self as Self::UnsignedWord}

    #[inline(always)]
    fn abs(self) -> Self { self.abs()}
    #[inline(always)]
    fn checked_abs(self) -> Option<Self> { self.checked_abs()}
    #[inline(always)]
    fn checked_neg(self) -> Option<Self> { self.checked_neg()}
    #[inline(always)]
    fn checked_sub_unsigned(self, rhs: Self::UnsignedWord) -> Option<Self> { self.checked_sub_unsigned(rhs)}
    #[inline(always)]
    fn saturating_add_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.saturating_add_unsigned(rhs)}
    #[inline(always)]
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.saturating_sub_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.wrapping_add_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.wrapping_sub_unsigned(rhs)}

    #[inline(always)]
    fn abs_diff(self, rhs: Self) -> Self::UnsignedWord { self.abs_diff(rhs)}
}

impl AtomicWord for $aty {
    type NonAtomicWord = $ty;

    #[inline(always)]
    fn new(value: Self::NonAtomicWord) -> Self {
        <$aty>::new(value)
    }

    #[inline(always)]
    fn load(&self, order: Ordering) -> Self::NonAtomicWord {
        <$aty>::load(self, order)
    }

    #[inline(always)]
    fn store(&self, value: Self::NonAtomicWord, order: Ordering) {
        <$aty>::store(self, value, order)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut Self::NonAtomicWord {
        <$aty>::get_mut(self)
    }

    #[inline(always)]

    fn into_inner(self) -> Self::NonAtomicWord {
        <$aty>::into_inner(self)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicWord]{
        <$aty>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self::NonAtomicWord]) -> &mut [Self]{
        <$aty>::from_mut_slice(this)
    }

    #[inline(always)]
    fn compare_exchange(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord> {
        <$aty>::compare_exchange(
            self,
            current,
            new,
            success,
            failure,
        )
    }


    #[inline(always)]    
    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>{
        <$aty>::compare_exchange_weak(
            self,
            current,
            new,
            success,
            failure,
        )
    }

    #[inline(always)]
    fn swap(
        &self,
        new: Self::NonAtomicWord,
        order: Ordering,
    ) -> Self::NonAtomicWord{
        <$aty>::swap(
            self,
            new,
            order,
        )
    }

    #[inline(always)]
    fn fetch_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_add(self, value, order)
    }
    
    #[inline(always)]
    fn fetch_saturating_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        let mut base = <$aty>::load(self, order);
        loop {
            let new = base.saturating_add(value);
            let res = <$aty>::compare_exchange_weak(
                self,
                base,
                new,
                order,
                order,
            );
            match res {
                Ok(val) => {return val},
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline(always)]
    fn fetch_and(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_and(self, value, order)
    }
    #[inline(always)]
    fn fetch_max(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_max(self, value, order)
    }
    #[inline(always)]
    fn fetch_min(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_min(self, value, order)
    }
    #[inline(always)]
    fn fetch_nand(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_nand(self, value, order)
    }
    #[inline(always)]
    fn fetch_or(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_or(self, value, order)
    }
    #[inline(always)]
    fn fetch_sub(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_sub(self, value, order)
    }
    #[inline(always)]
    fn fetch_xor(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_xor(self, value, order)
    }

    #[inline(always)]
    fn fetch_update<F>(
        &self, 
        set_order: Ordering, 
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>
    where
        F: FnMut(Self::NonAtomicWord) -> Option<Self::NonAtomicWord> {
        <$aty>::fetch_update(self, set_order, fetch_order, f)
    }
}


impl NonZero for $nzty {
    type BaseType = $ty;

    unsafe fn new_unchecked(n: Self::BaseType) -> Self {
        <$nzty>::new_unchecked(n)
    }

    fn new(n: Self::BaseType) -> Option<Self>{
        <$nzty>::new(n)
    }

    fn get(self) -> Self::BaseType{
        <$nzty>::get(self)
    }
}


impl NonZero for $nzsty {
    type BaseType = $sty;

    unsafe fn new_unchecked(n: Self::BaseType) -> Self {
        <$nzsty>::new_unchecked(n)
    }

    fn new(n: Self::BaseType) -> Option<Self>{
        <$nzsty>::new(n)
    }

    fn get(self) -> Self::BaseType{
        <$nzsty>::get(self)
    }
}

    };
}

impl_word!(u8, i8, AtomicU8, AtomicI8, NonZeroU8, NonZeroI8);
impl_word!(u16, i16, AtomicU16, AtomicI16, NonZeroU16, NonZeroI16);
impl_word!(u32, i32, AtomicU32, AtomicI32, NonZeroU32, NonZeroI32);
impl_word!(u64, i64, AtomicU64, AtomicI64, NonZeroU64, NonZeroI64);
impl_word!(usize, isize, AtomicUsize, AtomicIsize, NonZeroUsize, NonZeroIsize);
//impl_word!(u128, i128, AtomicU128, AtomicI128);

macro_rules! impl_float {
    ($($ty:ty,)*) => {$(

impl FSimd for $ty {}
        
impl Number for $ty {
    const BITS: usize = core::mem::size_of::<$ty>() * 8;
    const BYTES: usize = core::mem::size_of::<$ty>() as _;
    type BytesForm = [u8; core::mem::size_of::<$ty>()];
    const MIN: Self = <$ty>::MIN as _;
    const MAX: Self = <$ty>::MAX as _;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    #[inline(always)]
    fn from_be_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_be_bytes(bytes)}
    #[inline(always)]
    fn from_le_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_le_bytes(bytes)}
    #[inline(always)]
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_ne_bytes(bytes)}
    #[inline(always)]
    fn to_be_bytes(self) -> Self::BytesForm{self.to_be_bytes()}
    #[inline(always)]
    fn to_le_bytes(self) -> Self::BytesForm{self.to_le_bytes()}
    #[inline(always)]
    fn to_ne_bytes(self) -> Self::BytesForm{self.to_ne_bytes()}
}

impl Float for $ty {
    const RADIX: usize = <$ty>::RADIX as _;
    const DIGITS: usize = <$ty>::DIGITS as _;

    const EPSILON: Self = <$ty>::EPSILON;
    const INFINITY: Self = <$ty>::INFINITY;
    const NEG_INFINITY: Self = <$ty>::NEG_INFINITY;
    const NAN: Self = <$ty>::NAN;
    const MIN_POSITIVE: Self = <$ty>::MIN_POSITIVE;

    const MANTISSA_DIGITS: usize = <$ty>::MANTISSA_DIGITS as _;
    const MAX_10_EXP: usize = <$ty>::MAX_10_EXP as _;
    const MAX_EXP: usize = <$ty>::MAX_EXP as _;
    const MIN_10_EXP: usize = <$ty>::MIN_10_EXP as _;
    const MIN_EXP: usize = <$ty>::MIN_EXP as _;

    #[inline(always)]
    fn is_nan(self) -> bool {<$ty>::is_nan(self)}
    #[inline(always)]
    fn is_infinite(self) -> bool {<$ty>::is_infinite(self)}
    #[inline(always)]
    fn is_finite(self) -> bool {<$ty>::is_finite(self)}
    #[inline(always)]
    fn is_subnormal(self) -> bool {<$ty>::is_subnormal(self)}
    #[inline(always)]
    fn is_normal(self) -> bool {<$ty>::is_normal(self)}
    #[inline(always)]
    fn classify(self) -> FpCategory {<$ty>::classify(self)}
    #[inline(always)]
    fn is_sign_positive(self) -> bool {<$ty>::is_sign_positive(self)}
    #[inline(always)]
    fn is_sign_negative(self) -> bool {<$ty>::is_sign_negative(self)}
    #[inline(always)]
    fn recip(self) -> Self {<$ty>::recip(self)}
    #[inline(always)]
    fn to_degrees(self) -> Self {<$ty>::to_degrees(self)}
    #[inline(always)]
    fn to_radians(self) -> Self {<$ty>::to_radians(self)}
    #[inline(always)]
    fn max(self, other: Self) -> Self {<$ty>::max(self, other)}
    #[inline(always)]
    fn min(self, other: Self) -> Self {<$ty>::min(self, other)}
    #[inline(always)]
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {<$ty>::total_cmp(self, other)}
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {<$ty>::clamp(self, min, max)}

    #[cfg(feature="std")]
    #[inline(always)]
    fn rem_euclid(self, rhs: Self) -> Self { <$ty>::rem_euclid(self, rhs)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn div_euclid(self, rhs: Self) -> Self { <$ty>::div_euclid(self, rhs)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn floor(self) -> Self {<$ty>::floor(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn ceil(self) -> Self {<$ty>::ceil(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn round(self) -> Self {<$ty>::round(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn trunc(self) -> Self {<$ty>::trunc(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn fract(self) -> Self {<$ty>::fract(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn abs(self) -> Self {<$ty>::abs(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn signum(self) -> Self {<$ty>::signum(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {<$ty>::copysign(self, sign)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {<$ty>::mul_add(self, a, b)}
    #[cfg(feature="std")]
    fn powi(self, n: isize) -> Self {<$ty>::powi(self, n as i32)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn powf(self, n: Self) -> Self {<$ty>::powf(self, n)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn sqrt(self) -> Self {<$ty>::sqrt(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn exp(self) -> Self {<$ty>::exp(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn exp2(self) -> Self {<$ty>::exp2(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn ln(self) -> Self {<$ty>::ln(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn log(self, base: Self) -> Self {<$ty>::log(self, base)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn log2(self) -> Self {<$ty>::log2(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn log10(self) -> Self {<$ty>::log10(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn cbrt(self) -> Self {<$ty>::cbrt(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn hypot(self, other: Self) -> Self {<$ty>::hypot(self, other)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn sin(self) -> Self {<$ty>::sin(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn cos(self) -> Self {<$ty>::cos(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn tan(self) -> Self {<$ty>::tan(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn asin(self) -> Self {<$ty>::asin(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn acos(self) -> Self {<$ty>::acos(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn atan(self) -> Self {<$ty>::atan(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {<$ty>::atan2(self, other)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {<$ty>::sin_cos(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn exp_m1(self) -> Self {<$ty>::exp_m1(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn ln_1p(self) -> Self {<$ty>::ln_1p(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn sinh(self) -> Self {<$ty>::sinh(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn cosh(self) -> Self {<$ty>::cosh(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn tanh(self) -> Self {<$ty>::tanh(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn asinh(self) -> Self {<$ty>::asinh(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn acosh(self) -> Self {<$ty>::acosh(self)}
    #[cfg(feature="std")]
    #[inline(always)]
    fn atanh(self) -> Self {<$ty>::atanh(self)}
}

    )*};
}

impl_float!(f32, f64,);

//#[cfg(feature="half")]
//impl_float!(half::f16,);
