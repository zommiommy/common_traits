/// An generalization of [`core::hash::Hasher`] that doesn't force the output to
/// be [`u64`]
pub trait Hasher {
    type Result;
    fn finish(&self) -> Self::Result;
    fn write(&mut self, bytes: &[u8]);
}

/// An hasher that has extra parameters in initalization
pub trait SeedableHasher {
    type Seed;
    fn new(seed: Self::Seed) -> Self;
}

/// The analog of [`core::hash::Hash`] but that uses [`Hash`]
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
}
