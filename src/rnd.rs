/// A generic Random number generator
pub trait Rng {
    /// Instantiate a new Rng making no assumptions on its seed.
    fn new(seed: u64) -> Self;

    /// automatic dispatching of the implemmentation, no need to re-implement
    #[inline(always)]
    fn next<T>(&mut self) -> T 
        where Self: RngNext<T>
    {
        <Self as RngNext<T>>::next_inner(self)
    }
}

/// Implementation of a specific type generation for a Rng
pub trait RngNext<T> {
    fn next_inner(&mut self) -> T;
}