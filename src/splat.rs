/// Take a smaller value and broadcast it to all positions.
///
/// (Thanks to B3NNY for the more readable code, this should compile to
/// the original multiplication by 0x0101010101010101).
pub trait Splat<T> {
    fn splat(value: T) -> Self;
}

/// Blanket implementation that ensures that a reflexive splat is a no-operation
impl<T> Splat<T> for T {
    #[inline(always)]
    fn splat(value: T) -> Self {
        value
    }
}

macro_rules! impl_broadcast {
    ($($ty1:ty => $ty2:ty,)*) => {
$(
impl Splat<$ty1> for $ty2 {
    #[inline(always)]
    fn splat(value: $ty1) -> Self {
        const SIZE: usize = core::mem::size_of::<$ty2>() / core::mem::size_of::<$ty1>();
        #[allow(clippy::useless_transmute)]
        <$ty2>::from_ne_bytes(unsafe{
            core::mem::transmute::<[$ty1; SIZE], [u8; core::mem::size_of::<$ty2>()]>([value; SIZE])
        })
    }
}
)*
    };
}

impl_broadcast!(
    u8 => u16,
    u8 => u32,
    u8 => u64,
    u8 => usize,
    u8 => u128,

    u16 => u32,
    u16 => u64,
    u16 => u128,

    u32 => u64,
    u32 => u128,

    u64 => u128,
    // TODO add simd splat
);
