pub trait Hasher {
    type Result;
    fn finish(&self) -> Self::Result;
    fn write(&mut self, bytes: &[u8]);
}

pub trait SeedableHasher {
    type Seed;
    fn new(seed: Self::Seed) -> Self;
}

pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
}
