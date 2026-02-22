/// A generalization of [`core::hash::Hasher`] that doesn't force the output to
/// be [`u64`].
pub trait Hasher {
    /// The type of the hash output.
    type Result;
    /// Returns the hash value for the values written so far.
    fn finish(&self) -> Self::Result;
    /// Writes some data into this [`Hasher`].
    fn write(&mut self, bytes: &[u8]);
}

/// A hasher with additional initialization parameters.
pub trait SeedableHasher {
    /// The type of the seed.
    type Seed;
    /// Creates a new hasher with the given seed.
    fn new(seed: Self::Seed) -> Self;
}

/// Analog of [`core::hash::Hash`], using [`Hasher`].
pub trait Hash {
    /// Feeds this value into the given [`Hasher`].
    fn hash<H: Hasher>(&self, state: &mut H);
}
