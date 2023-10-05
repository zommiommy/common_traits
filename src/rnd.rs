/// A generic Random number generator
///
/// # Example
///
/// ```rust
/// use common_traits::{Rng, RngNext};
///
/// pub struct Xorshift64(u64);
///
/// impl Rng for Xorshift64 {
///     type Seed = u64;
///
///     fn new(seed: u64) -> Self {
///         Self(seed.saturating_add(1))
///     }
/// }
///
/// impl RngNext<u64> for Xorshift64 {
///     fn next_inner(&mut self) -> u64 {
///         self.0 ^= self.0 << 13;
///         self.0 ^= self.0 >> 7;
///         self.0 ^= self.0 << 17;
///         self.0
///     }
/// }
///
/// impl RngNext<f64> for Xorshift64 {
///     fn next_inner(&mut self) -> f64 {
///         let v: u64 = (self.next::<u64>() >> 11) | (1023 << 52);
///         let r: f64 = f64::from_le_bytes(v.to_le_bytes());
///         r - 1f64
///     }
/// }
/// ```
pub trait Rng {
    type Seed;

    /// Instantiate a new Rng making no assumptions on its seed.
    fn new(seed: Self::Seed) -> Self;

    /// automatic dispatching of the implementation, no need to re-implement
    #[inline(always)]
    fn next<T>(&mut self) -> T
    where
        Self: RngNext<T>,
    {
        <Self as RngNext<T>>::next_inner(self)
    }
}

/// Implementation of a specific type generation for a Rng
///
/// # Example
///
/// ```rust
/// use common_traits::{Rng, RngNext};
///
/// pub struct Xorshift64(u64);
///
/// impl Rng for Xorshift64 {
///     type Seed = u64;
///     fn new(seed: u64) -> Self {
///         Self(seed.saturating_add(1))
///     }
/// }
///
/// impl RngNext<u64> for Xorshift64 {
///     fn next_inner(&mut self) -> u64 {
///         self.0 ^= self.0 << 13;
///         self.0 ^= self.0 >> 7;
///         self.0 ^= self.0 << 17;
///         self.0
///     }
/// }
///
/// impl RngNext<f64> for Xorshift64 {
///     fn next_inner(&mut self) -> f64 {
///         let v: u64 = (self.next::<u64>() >> 11) | (1023 << 52);
///         let r: f64 = f64::from_le_bytes(v.to_le_bytes());
///         r - 1f64
///     }
/// }
/// ```
pub trait RngNext<T> {
    fn next_inner(&mut self) -> T;
}
