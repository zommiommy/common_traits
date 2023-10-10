use crate::*;
use core::num::*;
use core::sync::atomic::*;

macro_rules! impl_atomic_integer {
    ($aty:ty) => {
        impl AtomicNumber for $aty {
            #[inline(always)]
            fn fetch_add(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_add(self, value, order)
            }

            #[inline(always)]
            fn fetch_sub(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_sub(self, value, order)
            }
        }

        impl AtomicInteger for $aty {}
    };
}

macro_rules! impl_atomic_signed_int {
    ($aty:ty) => {
        impl AtomicSignedInt for $aty {}
    };
}

macro_rules! impl_atomic_unsigned_int {
    ($aty:ty) => {
        impl AtomicUnsignedInt for $aty {}
    };
}

macro_rules! impl_into_atomic {
    ($ty:ty, $aty:ty) => {
        impl IsAtomic for $ty {
            type Atomic = False;
        }
        impl IsAtomic for $aty {
            type Atomic = True;
        }

        impl Scalar for $aty {
            const BITS: usize = <$ty>::BITS as usize;
            const BYTES: usize = <$ty>::BYTES;
            type Bytes = <$ty as Scalar>::Bytes;
        }

        impl IntoAtomic for $ty {
            type AtomicType = $aty;

            #[inline(always)]
            fn to_atomic(self) -> Self::AtomicType {
                Self::AtomicType::new(self)
            }

            #[inline(always)]
            fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N] {
                #[allow(clippy::uninit_assumed_init)]
                let mut res: [Self::AtomicType; N] =
                    unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                    res[i] = Self::AtomicType::new(data[i]);
                }
                res
            }

            #[inline(always)]
            fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N] {
                unsafe { *(data.as_ptr() as *const [Self; N]) }
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
                <$aty>::get_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
                unsafe { core::mem::transmute(this) }
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
                <$aty>::from_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
                unsafe { core::mem::transmute(this) }
            }

            #[inline(always)]
            fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N] {
                unsafe { core::mem::transmute(this) }
            }

            #[inline(always)]
            fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N] {
                unsafe { core::mem::transmute(this) }
            }
        }

        impl Atomic for $aty {
            type NonAtomicType = $ty;

            #[inline(always)]
            fn new(value: Self::NonAtomicType) -> Self {
                <$aty>::new(value)
            }

            #[inline(always)]
            fn load(&self, order: Ordering) -> Self::NonAtomicType {
                <$aty>::load(self, order)
            }

            #[inline(always)]
            fn store(&self, value: Self::NonAtomicType, order: Ordering) {
                <$aty>::store(self, value, order)
            }

            #[inline(always)]
            fn get_mut(&mut self) -> &mut Self::NonAtomicType {
                <$aty>::get_mut(self)
            }

            #[inline(always)]
            fn into_inner(self) -> Self::NonAtomicType {
                <$aty>::into_inner(self)
            }

            #[inline(always)]
            fn into_non_atomic_array<const N: usize>(data: [Self; N]) -> [Self::NonAtomicType; N] {
                unsafe { *(data.as_ptr() as *const [Self::NonAtomicType; N]) }
            }

            #[inline(always)]
            fn from_non_atomic_array<const N: usize>(data: [Self::NonAtomicType; N]) -> [Self; N] {
                let mut res: [Self; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                    res[i] = Self::new(data[i]);
                }
                res
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType] {
                <$aty>::get_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType] {
                unsafe { core::mem::transmute::<&mut [Self], &mut [Self::NonAtomicType]>(this) }
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self] {
                <$aty>::from_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self] {
                unsafe { core::mem::transmute::<&mut [Self::NonAtomicType], &mut [Self]>(this) }
            }

            #[inline(always)]
            fn get_mut_array<const N: usize>(
                this: &mut [Self; N],
            ) -> &mut [Self::NonAtomicType; N] {
                unsafe {
                    core::mem::transmute::<&mut [Self; N], &mut [Self::NonAtomicType; N]>(this)
                }
            }
            #[inline(always)]
            fn from_mut_array<const N: usize>(
                this: &mut [Self::NonAtomicType; N],
            ) -> &mut [Self; N] {
                unsafe {
                    core::mem::transmute::<&mut [Self::NonAtomicType; N], &mut [Self; N]>(this)
                }
            }

            #[inline(always)]
            fn compare_exchange(
                &self,
                current: Self::NonAtomicType,
                new: Self::NonAtomicType,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
                <$aty>::compare_exchange(self, current, new, success, failure)
            }

            #[inline(always)]
            fn compare_exchange_weak(
                &self,
                current: Self::NonAtomicType,
                new: Self::NonAtomicType,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
                <$aty>::compare_exchange_weak(self, current, new, success, failure)
            }

            #[inline(always)]
            fn swap(&self, new: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
                <$aty>::swap(self, new, order)
            }
            #[inline(always)]
            fn fetch_and(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_and(self, value, order)
            }
            #[inline(always)]
            fn fetch_max(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_max(self, value, order)
            }
            #[inline(always)]
            fn fetch_min(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_min(self, value, order)
            }
            #[inline(always)]
            fn fetch_nand(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_nand(self, value, order)
            }
            #[inline(always)]
            fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
                <$aty>::fetch_or(self, value, order)
            }
            #[inline(always)]
            fn fetch_xor(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                <$aty>::fetch_xor(self, value, order)
            }

            #[inline(always)]
            fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
            where
                F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>,
            {
                <$aty>::fetch_update(self, set_order, fetch_order, f)
            }
        }
    };
}

macro_rules! impl_Number {
    ($ty:ty) => {
        impl Scalar for $ty {
            const BITS: usize = <$ty>::BITS as _;
            const BYTES: usize = core::mem::size_of::<$ty>() as _;
            type Bytes = [u8; core::mem::size_of::<$ty>()];
        }

        impl Number for $ty {
            const MIN: Self = <$ty>::MIN as _;
            const MAX: Self = <$ty>::MAX as _;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline(always)]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                <$ty>::from_be_bytes(bytes)
            }
            #[inline(always)]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                <$ty>::from_le_bytes(bytes)
            }
            #[inline(always)]
            fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                <$ty>::from_ne_bytes(bytes)
            }
            #[inline(always)]
            fn to_be_bytes(self) -> Self::Bytes {
                self.to_be_bytes()
            }
            #[inline(always)]
            fn to_le_bytes(self) -> Self::Bytes {
                self.to_le_bytes()
            }
            #[inline(always)]
            fn to_ne_bytes(self) -> Self::Bytes {
                self.to_ne_bytes()
            }
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                (self * a) + b
            }
            #[inline(always)]
            fn max(self, other: Self) -> Self {
                if self >= other {
                    self
                } else {
                    other
                }
            }
            #[inline(always)]
            fn min(self, other: Self) -> Self {
                if self <= other {
                    self
                } else {
                    other
                }
            }
            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self {
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn pow(self, exp: Self) -> Self {
                self.pow(exp as u32)
            }
            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }
            #[inline(always)]
            fn saturating_div(self, rhs: Self) -> Self {
                self.saturating_div(rhs)
            }
            #[inline(always)]
            fn saturating_mul(self, rhs: Self) -> Self {
                self.saturating_mul(rhs)
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn saturating_pow(self, rhs: Self) -> Self {
                self.saturating_pow(rhs as u32)
            }
            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }
        }

        impl Integer for $ty {
            #[inline(always)]
            fn abs_diff(self, rhs: Self) -> Self {
                self.abs_diff(rhs) as Self
            }

            #[inline(always)]
            fn div_euclid(self, rhs: Self) -> Self {
                self.div_euclid(rhs)
            }
            #[inline(always)]
            fn rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
            #[inline(always)]
            fn to_le(self) -> Self {
                self.to_le()
            }
            #[inline(always)]
            fn swap_bytes(self) -> Self {
                self.swap_bytes()
            }
            #[inline(always)]
            fn to_be(self) -> Self {
                self.to_be()
            }
            #[inline(always)]
            fn from_le(rhs: Self) -> Self {
                <$ty>::from_le(rhs)
            }
            #[inline(always)]
            fn from_be(rhs: Self) -> Self {
                <$ty>::from_be(rhs)
            }

            #[inline(always)]
            fn overflow_shl(self, rhs: Self) -> Self {
                self.checked_shl(rhs.try_into().unwrap_or(1024))
                    .unwrap_or(0)
            }

            #[inline(always)]
            fn overflow_shr(self, rhs: Self) -> Self {
                self.checked_shr(rhs.try_into().unwrap_or(1024))
                    .unwrap_or(0)
            }

            #[inline(always)]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
            #[inline(always)]
            fn checked_div(self, rhs: Self) -> Option<Self> {
                self.checked_div(rhs)
            }
            #[inline(always)]
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                self.checked_div_euclid(rhs)
            }
            #[inline(always)]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                self.checked_mul(rhs)
            }
            #[inline(always)]
            fn checked_neg(self) -> Option<Self> {
                self.checked_neg()
            }
            #[inline(always)]
            fn checked_pow(self, exp: u32) -> Option<Self> {
                self.checked_pow(exp)
            }
            #[inline(always)]
            fn checked_rem(self, rhs: Self) -> Option<Self> {
                self.checked_rem(rhs)
            }
            #[inline(always)]
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                self.checked_rem_euclid(rhs)
            }
            #[inline(always)]
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                self.checked_shl(rhs)
            }
            #[inline(always)]
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                self.checked_shr(rhs)
            }
            #[inline(always)]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }
            #[inline(always)]
            fn count_ones(self) -> u32 {
                self.count_ones()
            }
            #[inline(always)]
            fn count_zeros(self) -> u32 {
                self.count_zeros()
            }
            #[inline(always)]
            fn leading_ones(self) -> u32 {
                self.leading_ones()
            }
            #[inline(always)]
            fn leading_zeros(self) -> u32 {
                self.leading_zeros()
            }
            #[inline(always)]
            fn reverse_bits(self) -> Self {
                self.reverse_bits()
            }
            #[inline(always)]
            fn rotate_left(self, rhs: u32) -> Self {
                self.rotate_left(rhs)
            }
            #[inline(always)]
            fn rotate_right(self, rhs: u32) -> Self {
                self.rotate_right(rhs)
            }
            #[inline(always)]
            fn trailing_ones(self) -> u32 {
                self.trailing_ones()
            }
            #[inline(always)]
            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }
            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
            #[inline(always)]
            fn wrapping_div(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }
            #[inline(always)]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                self.wrapping_div_euclid(rhs)
            }
            #[inline(always)]
            fn wrapping_mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
            #[inline(always)]
            fn wrapping_neg(self) -> Self {
                self.wrapping_neg()
            }
            #[inline(always)]
            fn wrapping_pow(self, exp: u32) -> Self {
                self.wrapping_pow(exp)
            }
            #[inline(always)]
            fn wrapping_rem(self, rhs: Self) -> Self {
                self.wrapping_rem(rhs)
            }
            #[inline(always)]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                self.wrapping_rem_euclid(rhs)
            }
            #[inline(always)]
            fn wrapping_shl(self, exp: u32) -> Self {
                self.wrapping_shl(exp)
            }
            #[inline(always)]
            fn wrapping_shr(self, exp: u32) -> Self {
                self.wrapping_shr(exp)
            }
            #[inline(always)]
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
        }
    };
}

macro_rules! impl_UnsignedInt {
    ($ty:ty, $sty:ty, $nzty:ty, $nzsty:ty) => {

impl_Number!($ty);
impl_Number!($sty);

impl IsSigned for $ty {
    type Signed = False;
}
impl IsSigned for $sty {
    type Signed = True;
}
impl IsNonZero for $ty {
    type NonZero = False;
}
impl IsNonZero for $sty {
    type NonZero = False;
}
impl IsNonZero for $nzty {
    type NonZero = True;
}
impl IsNonZero for $nzsty {
    type NonZero = True;
}

impl UnsignedInt for $ty {
    type SignedInt = $sty;
    type NonZeroUnsignedInt = $nzty;


    #[inline(always)]
    fn to_signed(self) -> Self::SignedInt {self as Self::SignedInt}

    #[inline(always)]
    fn checked_next_power_of_two(self) -> Option<Self>{self.checked_next_power_of_two()}

    #[inline(always)]
    fn sign_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (((self << shift_amount) as Self::SignedInt) >> shift_amount) as Self
    }

    #[inline(always)]
    fn zero_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (self << shift_amount) >> shift_amount
    }

    #[inline(always)]
    fn overflow_sar(self, rhs: Self) -> Self {
        let shift_amount = core::cmp::min(rhs, Self::BITS as Self - 1);
        ((self as Self::SignedInt) >> shift_amount) as Self
    }

    #[inline(always)]
    fn ilog2(self) -> Self {
        self.ilog2() as Self
    }

    #[inline(always)]
    fn checked_add_signed(self, rhs: Self::SignedInt) -> Option<Self>{self.checked_add_signed(rhs)}
    #[inline(always)]
    fn saturating_add_signed(self, rhs: Self::SignedInt) -> Self{self.saturating_add_signed(rhs)}
    #[inline(always)]
    fn wrapping_add_signed(self, rhs: Self::SignedInt) -> Self{self.wrapping_add_signed(rhs)}
    #[inline(always)]
    fn is_power_of_two(self) -> bool{self.is_power_of_two()}
    #[inline(always)]
    fn next_power_of_two(self) -> Self{self.next_power_of_two()}
}

impl SignedInt for $sty {
    type UnsignedInt = $ty;
    type NonZeroUnsignedInt = $nzsty;

    #[inline(always)]
    fn to_unsigned(self) -> Self::UnsignedInt {self as Self::UnsignedInt}

    #[inline(always)]
    fn abs(self) -> Self { self.abs()}
    #[inline(always)]
    fn signum(self) -> Self { self.signum()}
    #[inline(always)]
    fn checked_abs(self) -> Option<Self> { self.checked_abs()}
    #[inline(always)]
    fn checked_neg(self) -> Option<Self> { self.checked_neg()}
    #[inline(always)]
    fn checked_sub_unsigned(self, rhs: Self::UnsignedInt) -> Option<Self> { self.checked_sub_unsigned(rhs)}
    #[inline(always)]
    fn saturating_add_unsigned(self, rhs: Self::UnsignedInt) -> Self {self.saturating_add_unsigned(rhs)}
    #[inline(always)]
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedInt) -> Self {self.saturating_sub_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedInt) -> Self {self.wrapping_add_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedInt) -> Self {self.wrapping_sub_unsigned(rhs)}
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

// We implement separately IsAtomic for u128
// because this is done for the rest of the
// scalar types in impl_into_atomic!,
// and there is no AtomicU128 type.

impl IsAtomic for u128 {
    type Atomic = False;
}

impl IsAtomic for i128 {
    type Atomic = False;
}

impl_UnsignedInt!(u8, i8, NonZeroU8, NonZeroI8);
impl_UnsignedInt!(u16, i16, NonZeroU16, NonZeroI16);
impl_UnsignedInt!(u32, i32, NonZeroU32, NonZeroI32);
impl_UnsignedInt!(u64, i64, NonZeroU64, NonZeroI64);
impl_UnsignedInt!(usize, isize, NonZeroUsize, NonZeroIsize);
impl_UnsignedInt!(u128, i128, NonZeroU128, NonZeroI128);

impl_into_atomic!(u8, AtomicU8);
impl_into_atomic!(u16, AtomicU16);
impl_into_atomic!(u32, AtomicU32);
impl_into_atomic!(u64, AtomicU64);
impl_into_atomic!(usize, AtomicUsize);

impl_into_atomic!(i8, AtomicI8);
impl_into_atomic!(i16, AtomicI16);
impl_into_atomic!(i32, AtomicI32);
impl_into_atomic!(i64, AtomicI64);
impl_into_atomic!(isize, AtomicIsize);

impl_atomic_integer!(AtomicI8);
impl_atomic_integer!(AtomicI16);
impl_atomic_integer!(AtomicI32);
impl_atomic_integer!(AtomicI64);
impl_atomic_integer!(AtomicIsize);
impl_atomic_integer!(AtomicU8);
impl_atomic_integer!(AtomicU16);
impl_atomic_integer!(AtomicU32);
impl_atomic_integer!(AtomicU64);
impl_atomic_integer!(AtomicUsize);

impl_atomic_signed_int!(AtomicI8);
impl_atomic_signed_int!(AtomicI16);
impl_atomic_signed_int!(AtomicI32);
impl_atomic_signed_int!(AtomicI64);
impl_atomic_signed_int!(AtomicIsize);
impl_atomic_unsigned_int!(AtomicU8);
impl_atomic_unsigned_int!(AtomicU16);
impl_atomic_unsigned_int!(AtomicU32);
impl_atomic_unsigned_int!(AtomicU64);
impl_atomic_unsigned_int!(AtomicUsize);

impl IsAtomic for bool {
    type Atomic = False;
}
impl IsAtomic for AtomicBool {
    type Atomic = True;
}
impl IsSigned for bool {
    type Signed = False;
}
impl IsNonZero for bool {
    type NonZero = False;
}

impl IntoAtomic for bool {
    type AtomicType = AtomicBool;

    #[inline(always)]
    fn to_atomic(self) -> Self::AtomicType {
        Self::AtomicType::new(self)
    }

    #[inline(always)]
    fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N] {
        #[allow(clippy::uninit_assumed_init)]
        let mut res: [Self::AtomicType; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            res[i] = Self::AtomicType::new(data[i]);
        }
        res
    }

    #[inline(always)]
    fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N] {
        unsafe { *(data.as_ptr() as *const [Self; N]) }
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::Atomic]) -> &mut [Self] {
        <Self>::get_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
        unsafe { core::mem::transmute(this) }
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::Atomic] {
        <Self>::from_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
        unsafe { core::mem::transmute(this) }
    }

    #[inline(always)]
    fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N] {
        unsafe { core::mem::transmute(this) }
    }

    #[inline(always)]
    fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N] {
        unsafe { core::mem::transmute(this) }
    }
}

impl Atomic for AtomicBool {
    type NonAtomicType = bool;

    #[inline(always)]
    fn new(value: Self::NonAtomicType) -> Self {
        <Self>::new(value)
    }

    #[inline(always)]
    fn load(&self, order: Ordering) -> Self::NonAtomicType {
        <Self>::load(self, order)
    }

    #[inline(always)]
    fn store(&self, value: Self::NonAtomicType, order: Ordering) {
        <Self>::store(self, value, order)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut Self::NonAtomicType {
        <Self>::get_mut(self)
    }

    #[inline(always)]
    fn into_inner(self) -> Self::NonAtomicType {
        <Self>::into_inner(self)
    }

    #[inline(always)]
    fn into_non_atomic_array<const N: usize>(data: [Self; N]) -> [Self::NonAtomicType; N] {
        unsafe { *(data.as_ptr() as *const [Self::NonAtomicType; N]) }
    }

    #[inline(always)]
    fn from_non_atomic_array<const N: usize>(data: [Self::NonAtomicType; N]) -> [Self; N] {
        #[allow(clippy::uninit_assumed_init)]
        let mut res: [Self; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            res[i] = Self::new(data[i]);
        }
        res
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType] {
        <Self>::get_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType] {
        unsafe { core::mem::transmute::<&mut [Self], &mut [Self::NonAtomicType]>(this) }
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self] {
        <Self>::from_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self::NonAtomicType]) -> &mut [Self] {
        unsafe { core::mem::transmute::<&mut [Self::NonAtomicType], &mut [Self]>(this) }
    }

    #[inline(always)]
    fn get_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::NonAtomicType; N] {
        unsafe { core::mem::transmute::<&mut [Self; N], &mut [Self::NonAtomicType; N]>(this) }
    }
    #[inline(always)]
    fn from_mut_array<const N: usize>(this: &mut [Self::NonAtomicType; N]) -> &mut [Self; N] {
        unsafe { core::mem::transmute::<&mut [Self::NonAtomicType; N], &mut [Self; N]>(this) }
    }

    #[inline(always)]
    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        <Self>::compare_exchange(self, current, new, success, failure)
    }

    #[inline(always)]
    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        <Self>::compare_exchange_weak(self, current, new, success, failure)
    }

    #[inline(always)]
    fn swap(&self, new: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::swap(self, new, order)
    }
    #[inline(always)]
    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_and(self, value, order)
    }
    #[inline(always)]
    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_or(self, value, order)
    }
    #[inline(always)]
    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_and(self, value, order)
    }
    #[inline(always)]
    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_nand(self, value, order)
    }
    #[inline(always)]
    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_or(self, value, order)
    }
    #[inline(always)]
    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        <Self>::fetch_xor(self, value, order)
    }

    #[inline(always)]
    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>,
    {
        <Self>::fetch_update(self, set_order, fetch_order, f)
    }
}

macro_rules! impl_float {
    ($($ty:ty, $aty:ty, $zero:expr, $one:expr,)*) => {$(

impl Scalar for $aty {
    const BITS: usize = <$ty>::BITS;
    const BYTES: usize = <$ty>::BYTES;
    type Bytes = [u8;  <$ty>::BYTES];
}

impl Scalar for $ty {
    const BITS: usize = Self::BYTES * 8;
    const BYTES: usize = core::mem::size_of::<$ty>();
    type Bytes = [u8; Self::BYTES];
}

impl IsSigned for $ty {
    type Signed = True;
}
impl IsNonZero for $ty {
    type NonZero = False;
}

impl IntoAtomic for $ty {
    type AtomicType = $aty;

    #[inline(always)]
    fn to_atomic(self) -> Self::AtomicType {
        Self::AtomicType::new(self)
    }

    #[inline(always)]
    fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N] {
        #[allow(clippy::uninit_assumed_init)]
        let mut res: [Self::AtomicType; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            res[i] = Self::AtomicType::new(data[i]);
        }
        res
    }

    #[inline(always)]
    fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N] {
        unsafe { *(data.as_ptr() as *const [Self; N]) }
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
        <$aty>::get_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
        unsafe { core::mem::transmute(this) }
    }

    #[cfg(feature = "atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
        <$aty>::from_mut_slice(this)
    }

    #[cfg(not(feature = "atomic_from_mut"))]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
        unsafe { core::mem::transmute(this) }
    }

    #[inline(always)]
    fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N] {
        unsafe { core::mem::transmute(this) }
    }

    #[inline(always)]
    fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N] {
        unsafe { core::mem::transmute(this) }
    }

}

impl Number for $ty {
    const MIN: Self = <Self>::MIN as _;
    const MAX: Self = <Self>::MAX as _;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    #[inline(always)]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {<$ty>::from_be_bytes(bytes)}
    #[inline(always)]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {<$ty>::from_le_bytes(bytes)}
    #[inline(always)]
    fn from_ne_bytes(bytes: Self::Bytes) -> Self {<$ty>::from_ne_bytes(bytes)}
    #[inline(always)]
    fn to_be_bytes(self) -> Self::Bytes{self.to_be_bytes()}
    #[inline(always)]
    fn to_le_bytes(self) -> Self::Bytes{self.to_le_bytes()}
    #[inline(always)]
    fn to_ne_bytes(self) -> Self::Bytes{self.to_ne_bytes()}
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(feature="std")]
        {
            <$ty>::mul_add(self, a, b)
        }
        #[cfg(not(feature="std"))]
        {
            (self * a) + b
        }
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {<$ty>::max(self, other)}
    #[inline(always)]
    fn min(self, other: Self) -> Self {<$ty>::min(self, other)}
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {<$ty>::clamp(self, min, max)}

    #[cfg(feature="std")]
    #[inline(always)]
    fn pow(self, exp: Self) -> Self {
        self.powf(exp)
    }
    #[inline(always)]
    fn saturating_add(self, rhs: Self) -> Self {
        let res = self + rhs;
        if res.is_nan() {
            return <$ty>::NAN;
        }
        if !res.is_finite() {
            if res.is_sign_positive() {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            res
        }
    }
    #[inline(always)]
    fn saturating_div(self, rhs: Self) -> Self {
        let res = self / rhs;
        if res.is_nan() {
            return <$ty>::NAN;
        }
        if !res.is_finite() {
            if res.is_sign_positive() {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            res
        }
    }
    #[inline(always)]
    fn saturating_mul(self, rhs: Self) -> Self {
        let res = self * rhs;
        if res.is_nan() {
            return <$ty>::NAN;
        }
        if !res.is_finite() {
            if res.is_sign_positive() {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            res
        }
    }
    #[cfg(feature="std")]
    #[inline(always)]
    fn saturating_pow(self, rhs: Self) -> Self {
        let res = self.pow(rhs);
        if res.is_nan() {
            return <$ty>::NAN;
        }
        if !res.is_finite() {
            if res.is_sign_positive() {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            res
        }
    }
    #[inline(always)]
    fn saturating_sub(self, rhs: Self) -> Self {
        let res = self - rhs;
        if res.is_nan() {
            return <$ty>::NAN;
        }
        if !res.is_finite() {
            if res.is_sign_positive() {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            res
        }
    }
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
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {<$ty>::total_cmp(self, other)}

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
    fn powi(self, n: isize) -> Self {<$ty>::powi(self, n as _)}
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

#[cfg(feature = "half")]
macro_rules! impl_f16 {
    ($ty:ty, $aty:ty) => {
        impl IsAtomic for $ty {
            type Atomic = False;
        }
        impl IsAtomic for $aty {
            type Atomic = True;
        }

        impl IsSigned for $ty {
            type Signed = True;
        }
        impl IsNonZero for $ty {
            type NonZero = False;
        }

        impl Scalar for $aty {
            const BITS: usize = 16;
            const BYTES: usize = 2;
            type Bytes = [u8; 2];
        }

        impl Scalar for $ty {
            const BITS: usize = 16;
            const BYTES: usize = 2;
            type Bytes = [u8; 2];
        }

        impl IntoAtomic for $ty {
            type AtomicType = $aty;

            #[inline(always)]
            fn to_atomic(self) -> Self::AtomicType {
                Self::AtomicType::new(self)
            }

            #[inline(always)]
            fn into_atomic_array<const N: usize>(data: [Self; N]) -> [Self::AtomicType; N] {
                #[allow(clippy::uninit_assumed_init)]
                let mut res: [Self::AtomicType; N] =
                    unsafe { core::mem::MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                    res[i] = Self::AtomicType::new(data[i]);
                }
                res
            }

            #[inline(always)]
            fn from_atomic_array<const N: usize>(data: [Self::AtomicType; N]) -> [Self; N] {
                unsafe { *(data.as_ptr() as *const [Self; N]) }
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
                <Self::AtomicType>::get_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn get_mut_slice(this: &mut [Self::AtomicType]) -> &mut [Self] {
                unsafe { core::mem::transmute(this) }
            }

            #[cfg(feature = "atomic_from_mut")]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
                <Self::AtomicType>::from_mut_slice(this)
            }

            #[cfg(not(feature = "atomic_from_mut"))]
            #[inline(always)]
            fn from_mut_slice(this: &mut [Self]) -> &mut [Self::AtomicType] {
                unsafe { core::mem::transmute(this) }
            }

            #[inline(always)]
            fn get_mut_array<const N: usize>(this: &mut [Self::AtomicType; N]) -> &mut [Self; N] {
                unsafe { core::mem::transmute(this) }
            }

            #[inline(always)]
            fn from_mut_array<const N: usize>(this: &mut [Self; N]) -> &mut [Self::AtomicType; N] {
                unsafe { core::mem::transmute(this) }
            }
        }

        impl Number for $ty {
            const MIN: Self = <Self>::MIN as _;
            const MAX: Self = <Self>::MAX as _;
            const ZERO: Self = Self::from_f32_const(0.0);
            const ONE: Self = Self::from_f32_const(1.0);

            #[inline(always)]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                <Self>::from_be_bytes(bytes)
            }
            #[inline(always)]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                <Self>::from_le_bytes(bytes)
            }
            #[inline(always)]
            fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                <Self>::from_ne_bytes(bytes)
            }
            #[inline(always)]
            fn to_be_bytes(self) -> Self::Bytes {
                self.to_be_bytes()
            }
            #[inline(always)]
            fn to_le_bytes(self) -> Self::Bytes {
                self.to_le_bytes()
            }
            #[inline(always)]
            fn to_ne_bytes(self) -> Self::Bytes {
                self.to_ne_bytes()
            }
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                (self * a) + b
            }
            #[inline(always)]
            fn max(self, other: Self) -> Self {
                <Self>::max(self, other)
            }
            #[inline(always)]
            fn min(self, other: Self) -> Self {
                <Self>::min(self, other)
            }
            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self {
                <Self>::clamp(self, min, max)
            }

            #[inline(always)]
            #[cfg(feature = "std")]
            fn pow(self, exp: Self) -> Self {
                self.powf(exp)
            }
            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                let res = self + rhs;
                if res.is_nan() {
                    return <Self>::NAN;
                }
                if !res.is_finite() {
                    if res.is_sign_positive() {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                } else {
                    res
                }
            }
            #[inline(always)]
            fn saturating_div(self, rhs: Self) -> Self {
                let res = self / rhs;
                if res.is_nan() {
                    return <Self>::NAN;
                }
                if !res.is_finite() {
                    if res.is_sign_positive() {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                } else {
                    res
                }
            }
            #[inline(always)]
            fn saturating_mul(self, rhs: Self) -> Self {
                let res = self * rhs;
                if res.is_nan() {
                    return <Self>::NAN;
                }
                if !res.is_finite() {
                    if res.is_sign_positive() {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                } else {
                    res
                }
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn saturating_pow(self, rhs: Self) -> Self {
                let res = self.pow(rhs);
                if res.is_nan() {
                    return <Self>::NAN;
                }
                if !res.is_finite() {
                    if res.is_sign_positive() {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                } else {
                    res
                }
            }
            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                let res = self - rhs;
                if res.is_nan() {
                    return <Self>::NAN;
                }
                if !res.is_finite() {
                    if res.is_sign_positive() {
                        Self::MAX
                    } else {
                        Self::MIN
                    }
                } else {
                    res
                }
            }
        }

        impl Float for $ty {
            const RADIX: usize = <Self>::RADIX as _;
            const DIGITS: usize = <Self>::DIGITS as _;

            const EPSILON: Self = <Self>::EPSILON;
            const INFINITY: Self = <Self>::INFINITY;
            const NEG_INFINITY: Self = <Self>::NEG_INFINITY;
            const NAN: Self = <Self>::NAN;
            const MIN_POSITIVE: Self = <Self>::MIN_POSITIVE;

            const MANTISSA_DIGITS: usize = <Self>::MANTISSA_DIGITS as _;
            const MAX_10_EXP: usize = <Self>::MAX_10_EXP as _;
            const MAX_EXP: usize = <Self>::MAX_EXP as _;
            const MIN_10_EXP: usize = <Self>::MIN_10_EXP as _;
            const MIN_EXP: usize = <Self>::MIN_EXP as _;

            #[inline(always)]
            fn is_nan(self) -> bool {
                <Self>::is_nan(self)
            }
            #[inline(always)]
            fn is_infinite(self) -> bool {
                <Self>::is_infinite(self)
            }
            #[inline(always)]
            fn is_finite(self) -> bool {
                <Self>::is_finite(self)
            }
            #[inline(always)]
            fn is_subnormal(self) -> bool {
                !self.is_normal()
            }
            #[inline(always)]
            fn is_normal(self) -> bool {
                <Self>::is_normal(self)
            }
            #[inline(always)]
            fn classify(self) -> FpCategory {
                <Self>::classify(self)
            }
            #[inline(always)]
            fn is_sign_positive(self) -> bool {
                <Self>::is_sign_positive(self)
            }
            #[inline(always)]
            fn is_sign_negative(self) -> bool {
                <Self>::is_sign_negative(self)
            }
            #[inline(always)]
            fn recip(self) -> Self {
                Self::ONE / self
            }
            #[inline(always)]
            fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
                <Self>::total_cmp(self, other)
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn signum(self) -> Self {
                <Self>::signum(self)
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn copysign(self, sign: Self) -> Self {
                <Self>::copysign(self, sign)
            }

            #[inline(always)]
            fn to_degrees(self) -> Self {
                <Self>::from_f32(self.to_f32().to_degrees())
            }
            #[inline(always)]
            fn to_radians(self) -> Self {
                <Self>::from_f32(self.to_f32().to_radians())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn rem_euclid(self, rhs: Self) -> Self {
                <Self>::from_f32(self.to_f32().rem_euclid(rhs.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn div_euclid(self, rhs: Self) -> Self {
                <Self>::from_f32(self.to_f32().div_euclid(rhs.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn floor(self) -> Self {
                <Self>::from_f32(self.to_f32().floor())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn ceil(self) -> Self {
                <Self>::from_f32(self.to_f32().ceil())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn round(self) -> Self {
                <Self>::from_f32(self.to_f32().round())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn trunc(self) -> Self {
                <Self>::from_f32(self.to_f32().trunc())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn fract(self) -> Self {
                <Self>::from_f32(self.to_f32().fract())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn abs(self) -> Self {
                <Self>::from_f32(self.to_f32().abs())
            }
            #[cfg(feature = "std")]
            fn powi(self, n: isize) -> Self {
                <Self>::from_f32(self.to_f32().powi(n as _))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                <Self>::from_f32(self.to_f32().powf(n.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn sqrt(self) -> Self {
                <Self>::from_f32(self.to_f32().sqrt())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn exp(self) -> Self {
                <Self>::from_f32(self.to_f32().exp())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn exp2(self) -> Self {
                <Self>::from_f32(self.to_f32().exp2())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn ln(self) -> Self {
                <Self>::from_f32(self.to_f32().ln())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn log(self, base: Self) -> Self {
                <Self>::from_f32(self.to_f32().log(base.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn log2(self) -> Self {
                <Self>::from_f32(self.to_f32().log2())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn log10(self) -> Self {
                <Self>::from_f32(self.to_f32().log10())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn cbrt(self) -> Self {
                <Self>::from_f32(self.to_f32().cbrt())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn hypot(self, other: Self) -> Self {
                <Self>::from_f32(self.to_f32().hypot(other.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn sin(self) -> Self {
                <Self>::from_f32(self.to_f32().sin())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn cos(self) -> Self {
                <Self>::from_f32(self.to_f32().cos())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn tan(self) -> Self {
                <Self>::from_f32(self.to_f32().tan())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn asin(self) -> Self {
                <Self>::from_f32(self.to_f32().asin())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn acos(self) -> Self {
                <Self>::from_f32(self.to_f32().acos())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn atan(self) -> Self {
                <Self>::from_f32(self.to_f32().atan())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn atan2(self, other: Self) -> Self {
                <Self>::from_f32(self.to_f32().atan2(other.to_f32()))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn sin_cos(self) -> (Self, Self) {
                let (s, c) = self.to_f32().sin_cos();
                (<Self>::from_f32(s), <Self>::from_f32(c))
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn exp_m1(self) -> Self {
                <Self>::from_f32(self.to_f32().exp_m1())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn ln_1p(self) -> Self {
                <Self>::from_f32(self.to_f32().ln_1p())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn sinh(self) -> Self {
                <Self>::from_f32(self.to_f32().sinh())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn cosh(self) -> Self {
                <Self>::from_f32(self.to_f32().cosh())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn tanh(self) -> Self {
                <Self>::from_f32(self.to_f32().tanh())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn asinh(self) -> Self {
                <Self>::from_f32(self.to_f32().asinh())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn acosh(self) -> Self {
                <Self>::from_f32(self.to_f32().acosh())
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn atanh(self) -> Self {
                <Self>::from_f32(self.to_f32().atanh())
            }
        }
    };
}

impl_float!(f32, AtomicF32, 0.0, 1.0, f64, AtomicF64, 0.0, 1.0,);
#[cfg(feature = "half")]
impl_f16!(half::f16, AtomicF16);
#[cfg(feature = "half")]
impl_f16!(half::bf16, AtomicBF16);
