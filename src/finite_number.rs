use crate::Number;

/// A number that has a Max and a Min.
pub trait FiniteNumber: Number {
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Saturating integer addition. Computes self + rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating integer division. Computes self / rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating integer multiplication. Computes self * rhs, saturating at
    /// the numeric bounds instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating integer exponentiation. Computes self.pow(exp), saturating
    /// at the numeric bounds instead of overflowing.
    fn saturating_pow(self, rhs: Self) -> Self;

    /// Saturating integer subtraction. Computes self - rhs, saturating at the
    /// numeric bounds instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;
}
