use crate::FiniteRangeNumber;
use crate::{False, IsAtomic, IsFloat, IsInteger, IsNonZero, IsSigned, True};
use core::fmt::LowerExp;
use core::ops::Neg;

/// Common operations on floats.
pub trait Float:
    Neg<Output = Self>
    + FiniteRangeNumber
    + LowerExp
    + IsAtomic<Atomic = False>
    + IsFloat<Float = True>
    + IsInteger<Integer = False>
    + IsSigned<Signed = True>
    + IsNonZero<NonZero = False>
{
    /// The radix or base of the internal representation of [`Self`].
    const RADIX: u32;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// This is the difference between `1.0` and the next larger representable number.
    const EPSILON: Self;

    /// Infinity (∞).
    const INFINITY: Self;

    /// Negative infinity (−∞).
    const NEG_INFINITY: Self;

    /// Not a Number (NaN).
    ///
    /// Note that IEEE 754 doesn’t define just a single NaN value; a plethora of
    /// bit patterns are considered to be NaN. Furthermore, the standard makes a
    /// difference between a “signaling” and a “quiet” NaN, and allows
    /// inspecting its “payload” (the unspecified bits in the bit pattern).
    /// This constant isn’t guaranteed to equal any specific NaN bit pattern,
    /// and the stability of its representation over Rust versions and target
    /// platforms isn’t guaranteed.
    const NAN: Self;

    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;
    /// Smallest positive normal value.
    const MIN_POSITIVE: Self;

    /// Returns `true` if this value is NaN.
    fn is_nan(self) -> bool;

    /// Returns `true` if this value is positive infinity or negative infinity,
    /// and `false` otherwise.
    fn is_infinite(self) -> bool;

    /// Returns `true` if this number is neither infinite nor NaN.
    fn is_finite(self) -> bool;

    /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Subnormal_number).
    fn is_subnormal(self) -> bool;

    /// Returns `true` if the number is neither zero, infinite, [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or NaN.
    fn is_normal(self) -> bool;

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    fn classify(self) -> core::num::FpCategory;

    /// Returns `true` if `self` has a positive sign, including +0.0, NaNs with
    /// positive sign bit and positive infinity. Note that IEEE 754 doesn’t
    /// assign any meaning to the sign bit in case of a NaN, and as Rust doesn’t
    /// guarantee that the bit pattern of NaNs are conserved over arithmetic
    /// operations, the result of `is_sign_positive` on a NaN might produce an
    /// unexpected result in some cases. See explanation of NaN as a special
    /// value for more info.
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` has a negative sign, including -0.0, NaNs with
    /// negative sign bit and negative infinity. Note that IEEE 754 doesn’t
    /// assign any meaning to the sign bit in case of a NaN, and as Rust doesn’t
    /// guarantee that the bit pattern of NaNs are conserved over arithmetic
    /// operations, the result of `is_sign_negative` on a NaN might produce an
    /// unexpected result in some cases. See explanation of NaN as a special
    /// value for more info.
    fn is_sign_negative(self) -> bool;

    /// Takes the reciprocal (inverse) of a number, `1/self`.
    fn recip(self) -> Self;

    /// Converts radians to degrees.
    fn to_degrees(self) -> Self;

    /// Converts degrees to radians.
    fn to_radians(self) -> Self;

    /// Returns the ordering between `self` and `other`.
    ///
    /// Unlike the standard partial comparison between floating point numbers,
    /// this comparison always produces an ordering in accordance to the
    /// `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
    /// floating point standard. The values are ordered in the following sequence:
    /// - negative quiet NaN
    /// - negative signaling NaN
    /// - negative infinity
    /// - negative numbers
    /// - negative subnormal numbers
    /// - negative zero
    /// - positive zero
    /// - positive subnormal numbers
    /// - positive numbers
    /// - positive infinity
    /// - positive signaling NaN
    /// - positive quiet NaN.
    ///
    /// The ordering established by this function does not always agree with the
    /// [`PartialOrd`] and [`PartialEq`] implementations of [`Self`].
    /// For example, they consider negative and positive zero equal, while
    /// `total_cmp` doesn’t.
    ///
    /// The interpretation of the signaling NaN bit follows the definition in
    /// the IEEE 754 standard, which may not match the interpretation by some
    /// of the older, non-conformant (e.g. MIPS) hardware implementations.
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering;

    /// Performs Euclidean division.
    #[cfg(feature = "std")]
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least non-negative remainder of `self (mod rhs)`.
    #[cfg(feature = "std")]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Returns the largest integer less than or equal to `self`.
    #[cfg(feature = "std")]
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    #[cfg(feature = "std")]
    fn ceil(self) -> Self;

    /// Returns the nearest integer to `self`. Rounds half-way cases away from `0.0`.
    #[cfg(feature = "std")]
    fn round(self) -> Self;

    /// Returns the integer part of `self`. This means that non-integer numbers
    /// are always truncated towards zero.
    #[cfg(feature = "std")]
    fn trunc(self) -> Self;

    /// Returns the fractional part of `self`.
    #[cfg(feature = "std")]
    fn fract(self) -> Self;

    /// Computes the absolute value of `self`.
    #[cfg(feature = "std")]
    fn abs(self) -> Self;

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NaN` if the number is `NaN`
    #[cfg(feature = "std")]
    fn signum(self) -> Self;

    /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal
    /// to `-self`. If `self` is a NaN, then a NaN with the sign bit of `sign` is
    /// returned. Note, however, that conserving the sign bit on NaN across
    /// arithmetical operations is not generally guaranteed. See explanation of
    /// NaN as a special value for more info.
    #[cfg(feature = "std")]
    fn copysign(self, sign: Self) -> Self;

    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using [`Float::powf`]. It
    /// might have a different sequence of rounding operations, so the results
    /// are not guaranteed to agree.
    #[cfg(feature = "std")]
    fn powi(self, n: isize) -> Self;

    /// Raises a number to a floating point power.
    #[cfg(feature = "std")]
    fn powf(self, n: Self) -> Self;

    /// Returns the square root of a number.
    ///
    /// Returns `NaN` if `self` is a negative number other than `-0.0`.
    #[cfg(feature = "std")]
    fn sqrt(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    #[cfg(feature = "std")]
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    #[cfg(feature = "std")]
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of the number.
    #[cfg(feature = "std")]
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation
    /// details; [`log2`](`Float::log2`) can produce more accurate results
    /// for base 2, and [`log10`](`Float::log10`) can produce more accurate
    /// results for base 10.
    #[cfg(feature = "std")]
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    #[cfg(feature = "std")]
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    #[cfg(feature = "std")]
    fn log10(self) -> Self;

    /// Returns the cube root of a number.
    #[cfg(feature = "std")]
    fn cbrt(self) -> Self;

    /// Calculates the length of the hypotenuse of a right-angle triangle given
    /// legs of length `self` and `other`.
    #[cfg(feature = "std")]
    fn hypot(self, other: Self) -> Self;

    /// Computes the sine of a number (in radians).
    #[cfg(feature = "std")]
    fn sin(self) -> Self;

    /// Computes the cosine of a number (in radians).
    #[cfg(feature = "std")]
    fn cos(self) -> Self;

    /// Computes the tangent of a number (in radians).
    #[cfg(feature = "std")]
    fn tan(self) -> Self;

    /// Computes the arcsine of a number. The return value is in radians in the
    /// range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    #[cfg(feature = "std")]
    fn asin(self) -> Self;

    /// Computes the arccosine of a number. The return value is in radians in the
    /// range [0, pi] or NaN if the number is outside the range [-1, 1].
    #[cfg(feature = "std")]
    fn acos(self) -> Self;

    /// Computes the arctangent of a number. The return value is in radians in the
    /// range [-pi/2, pi/2].
    #[cfg(feature = "std")]
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (y) and `other` (x) in radians.
    ///
    /// - `x = 0`, `y = 0`: `0`
    /// - `x >= 0`: `arctan(y/x) -> [-pi/2, pi/2]`
    /// - `y >= 0`: `arctan(y/x) + pi -> (pi/2, pi]`
    /// - `y < 0`: `arctan(y/x) - pi -> (-pi, -pi/2)`
    #[cfg(feature = "std")]
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `self`. Returns
    /// `(sin(self), cos(self))`.
    #[cfg(feature = "std")]
    fn sin_cos(self) -> (Self, Self);

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is
    /// close to zero.
    #[cfg(feature = "std")]
    fn exp_m1(self) -> Self;

    /// Returns `ln(1+self)` (natural logarithm) more accurately than if the
    /// operations were performed separately.
    #[cfg(feature = "std")]
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine function.
    #[cfg(feature = "std")]
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    #[cfg(feature = "std")]
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    #[cfg(feature = "std")]
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    #[cfg(feature = "std")]
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    #[cfg(feature = "std")]
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    #[cfg(feature = "std")]
    fn atanh(self) -> Self;
}
