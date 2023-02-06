/// Associated constants to get the maximum number of lanes usable for
/// different SIMD environments
pub trait MaxLanes {
    /// Maximum number of lanes supported by `portable_simd`
    const MAX_LANES: usize;
    /// Maximum number of lanes for AVX512 instructions (512 bit -> 64 bytes)
    const AVX512_LANES: usize;
    /// Maximum number of lanes for AVX2 instructions (256 bit -> 32 bytes)
    const AVX2_LANES: usize;
    /// Maximum number of lanes for SSE instructions (128 bit -> 16 bytes)
    const SSE_LANES: usize;
}

macro_rules! impl_max_lanes {
    ($($ty:ty,)*) => {      
$(
impl MaxLanes for $ty {
    const MAX_LANES: usize = 64;
    const AVX512_LANES: usize = 64 / core::mem::size_of::<$ty>();
    const AVX2_LANES: usize = 32 / core::mem::size_of::<$ty>();
    const SSE_LANES: usize = 16 / core::mem::size_of::<$ty>();
}
)*
    };
}

impl_max_lanes!(
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    f32,
    f64,
);
