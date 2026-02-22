use crate::{
    Atomic, AtomicFiniteRangeNumber, AtomicNumber, False, FiniteRangeNumber, Float, IsFloat,
    IsInteger, IsNonZero, IsSigned, Number, True,
};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[cfg(feature = "half")]
use core::sync::atomic::AtomicU16;

/// Atomic [`f64`] based on [`AtomicU64`].
#[derive(Debug)]
#[repr(transparent)]
pub struct AtomicF64(AtomicU64);

/// Atomic [`f32`] based on [`AtomicU32`].
#[derive(Debug)]
#[repr(transparent)]
pub struct AtomicF32(AtomicU32);

macro_rules! impl_atomic_float {
    ($ty:ty, $atomic:ty, $inner:ty) => {
        impl core::default::Default for $atomic {
            fn default() -> Self {
                Self::new(<Self as Atomic>::NonAtomicType::ZERO)
            }
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
                core::array::from_fn(|i| Self::new(data[i]))
            }

            #[inline(always)]
            fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicType] {
                unsafe { core::mem::transmute::<&mut [Self], &mut [Self::NonAtomicType]>(this) }
            }

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
            fn fetch_min(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(Ordering::Relaxed, order, |x| {
                    Some(Self::NonAtomicType::min(x, value))
                })
                .unwrap()
            }

            fn fetch_max(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(Ordering::Relaxed, order, |x| {
                    Some(Self::NonAtomicType::max(x, value))
                })
                .unwrap()
            }

            fn fetch_add(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(Ordering::Relaxed, order, |x| Some(x + value))
                    .unwrap()
            }

            fn fetch_sub(
                &self,
                value: Self::NonAtomicType,
                order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(Ordering::Relaxed, order, |x| Some(x - value))
                    .unwrap()
            }
        }
        impl AtomicFiniteRangeNumber for $atomic {
            #[inline(always)]
            fn fetch_saturating_add(
                &self,
                value: Self::NonAtomicType,
                set_order: Ordering,
                fetch_order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(set_order, fetch_order, |x| Some(x.saturating_add(value)))
                    .unwrap()
            }
            #[inline(always)]
            fn fetch_saturating_sub(
                &self,
                value: Self::NonAtomicType,
                set_order: Ordering,
                fetch_order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(set_order, fetch_order, |x| Some(x.saturating_sub(value)))
                    .unwrap()
            }
            #[inline(always)]
            fn fetch_saturating_mul(
                &self,
                value: Self::NonAtomicType,
                set_order: Ordering,
                fetch_order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(set_order, fetch_order, |x| Some(x.saturating_mul(value)))
                    .unwrap()
            }
            #[inline(always)]
            fn fetch_saturating_div(
                &self,
                value: Self::NonAtomicType,
                set_order: Ordering,
                fetch_order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(set_order, fetch_order, |x| Some(x.saturating_div(value)))
                    .unwrap()
            }
            #[cfg(feature = "std")]
            #[inline(always)]
            fn fetch_saturating_pow(
                &self,
                value: Self::NonAtomicType,
                set_order: Ordering,
                fetch_order: Ordering,
            ) -> Self::NonAtomicType {
                self.fetch_update(set_order, fetch_order, |x| Some(x.saturating_pow(value)))
                    .unwrap()
            }
        }
    };
}

impl_atomic_float!(f64, AtomicF64, AtomicU64);
impl_atomic_float!(f32, AtomicF32, AtomicU32);

/// Atomic [`half::f16`] based on [`AtomicU16`].
#[cfg(feature = "half")]
#[derive(Debug)]
#[repr(transparent)]
pub struct AtomicF16(pub(crate) AtomicU16);

/// Atomic [`half::bf16`] based on [`AtomicU16`].
#[cfg(feature = "half")]
#[derive(Debug)]
#[repr(transparent)]
pub struct AtomicBF16(pub(crate) AtomicU16);

/// An atomic float type.
pub trait AtomicFloat:
    AtomicFiniteRangeNumber
    + IsFloat<Float = True>
    + IsInteger<Integer = False>
    + IsSigned<Signed = True>
    + IsNonZero<NonZero = False>
where
    Self::NonAtomicType: Float,
{
    /// Returns `true` if this value is NaN.
    fn is_nan(&self, order: Ordering) -> bool;

    /// Returns `true` if this value is positive infinity or negative infinity,
    /// and `false` otherwise.
    fn is_infinite(&self, order: Ordering) -> bool;

    /// Returns `true` if this number is neither infinite nor NaN.
    fn is_finite(&self, order: Ordering) -> bool;

    /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Subnormal_number).
    fn is_subnormal(&self, order: Ordering) -> bool;

    /// Returns `true` if the number is neither zero, infinite, [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or NaN.
    fn is_normal(&self, order: Ordering) -> bool;

    /// Returns `true` if `self` has a positive sign, including +0.0, NaNs with
    /// positive sign bit and positive infinity. Note that IEEE 754 doesn’t
    /// assign any meaning to the sign bit in case of a NaN, and as Rust doesn’t
    /// guarantee that the bit pattern of NaNs are conserved over arithmetic
    /// operations, the result of `is_sign_positive` on a NaN might produce an
    /// unexpected result in some cases. See explanation of NaN as a special
    /// value for more info.
    fn is_sign_positive(&self, order: Ordering) -> bool;

    /// Returns `true` if `self` has a negative sign, including -0.0, NaNs with
    /// negative sign bit and negative infinity. Note that IEEE 754 doesn’t
    /// assign any meaning to the sign bit in case of a NaN, and as Rust doesn’t
    /// guarantee that the bit pattern of NaNs are conserved over arithmetic
    /// operations, the result of `is_sign_negative` on a NaN might produce an
    /// unexpected result in some cases. See explanation of NaN as a special
    /// value for more info.
    fn is_sign_negative(&self, order: Ordering) -> bool;

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    fn classify(&self, order: Ordering) -> core::num::FpCategory;

    /// Atomically sets `self` to the reciprocal (inverse) of a number, `1/self`.
    fn fetch_recip(&self, order: Ordering);

    /// Converts radians to degrees.
    fn fetch_to_degrees(&self, order: Ordering);

    /// Converts degrees to radians.
    fn fetch_to_radians(&self, order: Ordering);

    /// Atomically performs Euclidean division.
    #[cfg(feature = "std")]
    fn fetch_div_euclid(&self, rhs: Self::NonAtomicType, order: Ordering);

    /// Atomically calculates the least non-negative remainder of `self (mod rhs)`.
    #[cfg(feature = "std")]
    fn fetch_rem_euclid(&self, rhs: Self::NonAtomicType, order: Ordering);

    /// Returns the largest integer less than or equal to `self`.
    #[cfg(feature = "std")]
    fn fetch_floor(&self, order: Ordering);

    /// Returns the smallest integer greater than or equal to `self`.
    #[cfg(feature = "std")]
    fn fetch_ceil(&self, order: Ordering);

    /// Returns the nearest integer to `self`. Rounds half-way cases away from `0.0`.
    #[cfg(feature = "std")]
    fn fetch_round(&self, order: Ordering);

    /// Returns the integer part of `self`. This means that non-integer numbers
    /// are always truncated towards zero.
    #[cfg(feature = "std")]
    fn fetch_trunc(&self, order: Ordering);

    /// Returns the fractional part of `self`.
    #[cfg(feature = "std")]
    fn fetch_fract(&self, order: Ordering);

    /// Computes the absolute value of `self`.
    #[cfg(feature = "std")]
    fn fetch_abs(&self, order: Ordering);

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NaN` if the number is `NaN`
    #[cfg(feature = "std")]
    fn fetch_signum(&self, order: Ordering);

    /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal
    /// to `-self`. If `self` is a NaN, then a NaN with the sign bit of `sign` is
    /// returned. Note, however, that conserving the sign bit on NaN across
    /// arithmetical operations is not generally guaranteed. See explanation of
    /// NaN as a special value for more info.
    #[cfg(feature = "std")]
    fn fetch_copysign(&self, sign: Self::NonAtomicType, order: Ordering);

    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`. It might have a
    /// different sequence of rounding operations than `powf`, so the results are
    /// not guaranteed to agree.
    #[cfg(feature = "std")]
    fn fetch_powi(&self, n: isize, order: Ordering);

    /// Raises a number to a floating point power.
    #[cfg(feature = "std")]
    fn fetch_powf(&self, n: Self::NonAtomicType, order: Ordering);

    /// Returns the square root of a number.
    ///
    /// Returns `NaN` if `self` is a negative number other than `-0.0`.
    #[cfg(feature = "std")]
    fn fetch_sqrt(&self, order: Ordering);

    /// Returns `e^(self)`, (the exponential function).
    #[cfg(feature = "std")]
    fn fetch_exp(&self, order: Ordering);

    /// Returns `2^(self)`.
    #[cfg(feature = "std")]
    fn fetch_exp2(&self, order: Ordering);

    /// Returns the natural logarithm of the number.
    #[cfg(feature = "std")]
    fn fetch_ln(&self, order: Ordering);

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation
    /// details; `self.log2()` can produce more accurate results for base 2,
    /// and `self.log10()` can produce more accurate results for base 10.
    #[cfg(feature = "std")]
    fn fetch_log(&self, base: Self::NonAtomicType, order: Ordering);

    /// Returns the base 2 logarithm of the number.
    #[cfg(feature = "std")]
    fn fetch_log2(&self, order: Ordering);

    /// Returns the base 10 logarithm of the number.
    #[cfg(feature = "std")]
    fn fetch_log10(&self, order: Ordering);

    /// Returns the cube root of a number.
    #[cfg(feature = "std")]
    fn fetch_cbrt(&self, order: Ordering);

    /// Computes the sine of a number (in radians).
    #[cfg(feature = "std")]
    fn fetch_sin(&self, order: Ordering);

    /// Computes the cosine of a number (in radians).
    #[cfg(feature = "std")]
    fn fetch_cos(&self, order: Ordering);

    /// Computes the tangent of a number (in radians).
    #[cfg(feature = "std")]
    fn fetch_tan(&self, order: Ordering);

    /// Computes the arcsine of a number. The return value is in radians in the
    /// range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    #[cfg(feature = "std")]
    fn fetch_asin(&self, order: Ordering);

    /// Computes the arccosine of a number. The return value is in radians in the
    /// range [0, pi] or NaN if the number is outside the range [-1, 1].
    #[cfg(feature = "std")]
    fn fetch_acos(&self, order: Ordering);

    /// Computes the arctangent of a number. The return value is in radians in the
    /// range [-pi/2, pi/2].
    #[cfg(feature = "std")]
    fn fetch_atan(&self, order: Ordering);

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is
    /// close to zero.
    #[cfg(feature = "std")]
    fn fetch_exp_m1(&self, order: Ordering);

    /// Returns `ln(1+self)` (natural logarithm) more accurately than if the
    /// operations were performed separately.
    #[cfg(feature = "std")]
    fn fetch_ln_1p(&self, order: Ordering);

    /// Hyperbolic sine function.
    #[cfg(feature = "std")]
    fn fetch_sinh(&self, order: Ordering);

    /// Hyperbolic cosine function.
    #[cfg(feature = "std")]
    fn fetch_cosh(&self, order: Ordering);

    /// Hyperbolic tangent function.
    #[cfg(feature = "std")]
    fn fetch_tanh(&self, order: Ordering);

    /// Inverse hyperbolic sine function.
    #[cfg(feature = "std")]
    fn fetch_asinh(&self, order: Ordering);

    /// Inverse hyperbolic cosine function.
    #[cfg(feature = "std")]
    fn fetch_acosh(&self, order: Ordering);

    /// Inverse hyperbolic tangent function.
    #[cfg(feature = "std")]
    fn fetch_atanh(&self, order: Ordering);
}
