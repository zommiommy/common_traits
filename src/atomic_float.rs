use crate::*;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[cfg(feature = "half")]
use core::sync::atomic::AtomicU16;

#[derive(Debug)]
#[repr(transparent)]
/// Atomic [`f64`] based on [`AtomicU64`]
pub struct AtomicF64(AtomicU64);

#[derive(Debug)]
#[repr(transparent)]
/// Atomic [`f32`] based on [`AtomicU32`]
pub struct AtomicF32(AtomicU32);

macro_rules! impl_atomic_float {
    ($ty:ty, $atomic:ty, $inner:ty) => {
        impl IsAtomic for $ty {
            type Atomic = False;
        }
        impl IsAtomic for $atomic {
            type Atomic = True;
        }

        impl Atomic for $atomic {
            type NonAtomicType = $ty;

            fn new(value: Self::NonAtomicType) -> Self {
                Self(<$inner>::new(value.to_bits()))
            }

            fn load(&self, order: Ordering) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.load(order))
            }

            fn store(&self, value: Self::NonAtomicType, order: Ordering) {
                self.0.store(value.to_bits(), order)
            }

            fn get_mut(&mut self) -> &mut Self::NonAtomicType {
                unsafe { &mut *(self as *mut Self as *mut Self::NonAtomicType) }
            }

            fn into_inner(self) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.into_inner())
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

            fn compare_exchange(
                &self,
                current: Self::NonAtomicType,
                new: Self::NonAtomicType,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
                self.0
                    .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
                    .map(Self::NonAtomicType::from_bits)
                    .map_err(Self::NonAtomicType::from_bits)
            }

            fn compare_exchange_weak(
                &self,
                current: Self::NonAtomicType,
                new: Self::NonAtomicType,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
                self.0
                    .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
                    .map(Self::NonAtomicType::from_bits)
                    .map_err(Self::NonAtomicType::from_bits)
            }

            fn swap(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.swap(value.to_bits(), order))
            }

            fn fetch_and(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_and(value.to_bits(), order))
            }

            fn fetch_nand(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_nand(value.to_bits(), order))
            }

            fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_or(value.to_bits(), order))
            }

            fn fetch_xor(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_xor(value.to_bits(), order))
            }

            fn fetch_min(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_min(value.to_bits(), order))
            }

            fn fetch_max(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_max(value.to_bits(), order))
            }

            fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                mut f: F,
            ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
            where
                F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>,
            {
                self.0
                    .fetch_update(set_order, fetch_order, |x| {
                        f(Self::NonAtomicType::from_bits(x)).map(Self::NonAtomicType::to_bits)
                    })
                    .map(Self::NonAtomicType::from_bits)
                    .map_err(Self::NonAtomicType::from_bits)
            }
        }
        impl AtomicNumber for $atomic {
            fn fetch_add(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_add(value.to_bits(), order))
            }

            fn fetch_sub(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                Self::NonAtomicType::from_bits(self.0.fetch_sub(value.to_bits(), order))
            }
        }
    };
}

impl_atomic_float!(f64, AtomicF64, AtomicU64);
impl_atomic_float!(f32, AtomicF32, AtomicU32);

#[cfg(feature = "half")]
#[derive(Debug)]
#[repr(transparent)]
/// Atomic [`half::f16`] based on [`AtomicU16`]
pub struct AtomicF16(AtomicU16);

#[cfg(feature = "half")]
#[derive(Debug)]
#[repr(transparent)]
/// Atomic [`half::bf16`] based on [`AtomicU16`]
pub struct AtomicBF16(AtomicU16);

#[cfg(feature = "half")]
impl AtomicNumber for AtomicF16 {
    fn fetch_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_add(value.to_bits(), order))
    }

    fn fetch_sub(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_sub(value.to_bits(), order))
    }
}

#[cfg(feature = "half")]
impl AtomicNumber for AtomicBF16 {
    fn fetch_add(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_add(value.to_bits(), order))
    }

    fn fetch_sub(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_sub(value.to_bits(), order))
    }
}

#[cfg(feature = "half")]
impl Atomic for AtomicF16 {
    type NonAtomicType = half::f16;

    fn new(value: Self::NonAtomicType) -> Self {
        Self(<AtomicU16>::new(value.to_bits()))
    }

    fn load(&self, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.load(order))
    }

    fn store(&self, value: Self::NonAtomicType, order: Ordering) {
        self.0.store(value.to_bits(), order)
    }

    fn get_mut(&mut self) -> &mut Self::NonAtomicType {
        unsafe { &mut *(self as *mut Self as *mut Self::NonAtomicType) }
    }

    fn into_inner(self) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.into_inner())
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

    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        self.0
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }

    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        self.0
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }

    fn swap(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.swap(value.to_bits(), order))
    }

    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_and(value.to_bits(), order))
    }

    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_nand(value.to_bits(), order))
    }

    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_or(value.to_bits(), order))
    }

    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_xor(value.to_bits(), order))
    }

    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_min(value.to_bits(), order))
    }

    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_max(value.to_bits(), order))
    }

    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>,
    {
        self.0
            .fetch_update(set_order, fetch_order, |x| {
                f(Self::NonAtomicType::from_bits(x)).map(Self::NonAtomicType::to_bits)
            })
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }
}

#[cfg(feature = "half")]
impl Atomic for AtomicBF16 {
    type NonAtomicType = half::bf16;

    fn new(value: Self::NonAtomicType) -> Self {
        Self(<AtomicU16>::new(value.to_bits()))
    }

    fn load(&self, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.load(order))
    }

    fn store(&self, value: Self::NonAtomicType, order: Ordering) {
        self.0.store(value.to_bits(), order)
    }

    fn get_mut(&mut self) -> &mut Self::NonAtomicType {
        unsafe { &mut *(self as *mut Self as *mut Self::NonAtomicType) }
    }

    fn into_inner(self) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.into_inner())
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

    fn compare_exchange(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        self.0
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }

    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicType,
        new: Self::NonAtomicType,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType> {
        self.0
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }

    fn swap(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.swap(value.to_bits(), order))
    }

    fn fetch_and(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_and(value.to_bits(), order))
    }

    fn fetch_nand(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_nand(value.to_bits(), order))
    }

    fn fetch_or(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_or(value.to_bits(), order))
    }

    fn fetch_xor(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_xor(value.to_bits(), order))
    }

    fn fetch_min(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_min(value.to_bits(), order))
    }

    fn fetch_max(&self, value: Self::NonAtomicType, order: Ordering) -> Self::NonAtomicType {
        Self::NonAtomicType::from_bits(self.0.fetch_max(value.to_bits(), order))
    }

    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<Self::NonAtomicType, Self::NonAtomicType>
    where
        F: FnMut(Self::NonAtomicType) -> Option<Self::NonAtomicType>,
    {
        self.0
            .fetch_update(set_order, fetch_order, |x| {
                f(Self::NonAtomicType::from_bits(x)).map(Self::NonAtomicType::to_bits)
            })
            .map(Self::NonAtomicType::from_bits)
            .map_err(Self::NonAtomicType::from_bits)
    }
}
