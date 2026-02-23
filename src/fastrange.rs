use crate::Integer;

/// Fast division, modulo reduction, and an alternative operation
/// that maps a number between 0 and `d`.
///
/// # References
///
/// - [A fast alternative to the modulo reduction][lemire-blog]
/// - [fastmod][lemire-fastmod]
/// - [fastrange][lemire-fastrange]
///
/// [lemire-blog]: <https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/>
/// [lemire-fastmod]: <https://github.com/lemire/fastmod>
/// [lemire-fastrange]: <https://github.com/lemire/fastrange>
pub trait FastRange: Integer {
    /// The type of the precomputed mask.
    type MaskType: Integer + Copy;
    /// Given a value, produces an integer in [0, d) without division.
    /// The function is as fair as possible in the sense that if you
    /// iterate through all possible values, then you will generate
    /// all possible outputs as uniformly as possible.
    ///
    /// This is equivalent to computing `d * (self / 2^w)` in
    /// fixed point.
    fn fast_range(&self, d: Self) -> Self;
    /// Computes `self / d` given precomputed mask for `d > 1`.
    fn fast_div_mask(&self, mask: Self::MaskType) -> Self;
    /// Computes `self % d` given precomputed mask.
    fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self;

    /// Computes `self / d` for `d > 1`.
    #[inline(always)]
    fn fast_div(&self, d: Self) -> Self {
        let mask = d.compute_mask_fast();
        self.fast_div_mask(mask)
    }
    /// Computes `self % d`.
    #[inline(always)]
    fn fast_mod(&self, d: Self) -> Self {
        let mask = d.compute_mask_fast();
        self.fast_mod_mask(d, mask)
    }
    /// Checks whether `self % d == 0`.
    #[inline(always)]
    fn fast_is_divisible(&self, d: Self) -> bool {
        let mask = d.compute_mask_fast();
        self.fast_is_divisible_mask(mask)
    }

    /// Computes the mask needed by [`FastRange::fast_div_mask`] and
    /// [`FastRange::fast_mod_mask`]:
    /// `M = floor( (1 << 2*BITS) / self ) + 1`.
    ///
    /// `self` must be nonzero.
    fn compute_mask_fast(&self) -> Self::MaskType;

    /// Checks whether `self % d == 0` given precomputed mask.
    fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool;
}

impl FastRange for u8 {
    type MaskType = u16;
    #[inline(always)]
    fn fast_range(&self, d: Self) -> Self {
        ((*self as u16 * d as u16) >> 8) as u8
    }
    #[inline(always)]
    fn fast_div_mask(&self, mask: Self::MaskType) -> Self {
        ((*self as u32).wrapping_mul(mask as u32) >> 16) as u8
    }
    #[inline(always)]
    fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self {
        debug_assert_eq!(mask, d.compute_mask_fast());
        let low_bits = (*self as u16).wrapping_mul(mask);
        ((low_bits as u32).wrapping_mul(d as u32) >> 16) as u8
    }
    #[inline(always)]
    fn compute_mask_fast(&self) -> Self::MaskType {
        (u16::MAX / *self as u16).wrapping_add(1)
    }
    #[inline(always)]
    fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool {
        (*self as u16).wrapping_mul(mask) <= mask.wrapping_sub(1)
    }
}

impl FastRange for u16 {
    type MaskType = u32;
    #[inline(always)]
    fn fast_range(&self, d: Self) -> Self {
        ((*self as u32 * d as u32) >> 16) as u16
    }
    #[inline(always)]
    fn fast_div_mask(&self, mask: Self::MaskType) -> Self {
        ((*self as u64).wrapping_mul(mask as u64) >> 32) as u16
    }
    #[inline(always)]
    fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self {
        debug_assert_eq!(mask, d.compute_mask_fast());
        let low_bits = (*self as u32).wrapping_mul(mask);
        ((low_bits as u64).wrapping_mul(d as u64) >> 32) as u16
    }
    #[inline(always)]
    fn compute_mask_fast(&self) -> Self::MaskType {
        (u32::MAX / *self as u32).wrapping_add(1)
    }
    #[inline(always)]
    fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool {
        (*self as u32).wrapping_mul(mask) <= mask.wrapping_sub(1)
    }
}

impl FastRange for u32 {
    type MaskType = u64;
    #[inline(always)]
    fn fast_range(&self, d: Self) -> Self {
        ((*self as u64).wrapping_mul(d as u64) >> 32) as u32
    }
    #[inline(always)]
    fn fast_div_mask(&self, mask: Self::MaskType) -> Self {
        ((*self as u128).wrapping_mul(mask as u128) >> 64) as u32
    }
    #[inline(always)]
    fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self {
        debug_assert_eq!(mask, d.compute_mask_fast());
        let low_bits = (*self as u64).wrapping_mul(mask);
        ((low_bits as u128).wrapping_mul(d as u128) >> 64) as u32
    }
    #[inline(always)]
    fn compute_mask_fast(&self) -> Self::MaskType {
        (u64::MAX / *self as u64).wrapping_add(1)
    }
    #[inline(always)]
    fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool {
        (*self as u64).wrapping_mul(mask) <= mask.wrapping_sub(1)
    }
}

/// Performs a 128-bit multiply and returns the top 64 bits.
#[inline(always)]
fn mul128_u64(low_bits: u128, d: u64) -> u64 {
    let mut bottom_half = (low_bits & (u64::MAX as u128)).wrapping_mul(d as u128); // Won't overflow but avoid check
    bottom_half >>= 64; // Only need the top 64 bits, as we'll shift the lower half away;
    let top_half = (low_bits >> 64).wrapping_mul(d as u128);
    let mut both_halves = bottom_half + top_half; // Both halves are already shifted down by 64
    both_halves >>= 64; // Get top half of both_halves
    both_halves as u64
}

impl FastRange for u64 {
    type MaskType = u128;
    #[inline(always)]
    fn fast_range(&self, d: Self) -> Self {
        ((*self as u128).wrapping_mul(d as u128) >> 64) as u64
    }
    #[inline(always)]
    fn fast_div_mask(&self, mask: Self::MaskType) -> Self {
        mul128_u64(mask, *self)
    }
    #[inline(always)]
    fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self {
        debug_assert_eq!(mask, d.compute_mask_fast());
        let low_bits = (*self as u128).wrapping_mul(mask);
        mul128_u64(low_bits, d)
    }
    #[inline(always)]
    fn compute_mask_fast(&self) -> Self::MaskType {
        // what follows is just ((__uint128_t)0 - 1) / d) + 1 spelled out
        let mut mask: u128 = u64::MAX as u128;
        mask <<= 64;
        mask |= u64::MAX as u128;
        mask /= *self as u128;
        mask = mask.wrapping_add(1);
        mask
    }
    #[inline(always)]
    fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool {
        (*self as u128).wrapping_mul(mask) <= mask.wrapping_sub(1)
    }
}

macro_rules! impl_usize {
    ($ty:ty, $pw:literal, $mask:ty) => {
        #[cfg(target_pointer_width = $pw)]
        impl FastRange for usize {
            type MaskType = $mask;

            #[inline(always)]
            fn fast_range(&self, d: Self) -> Self {
                (*self as $ty).fast_range(d as $ty) as usize
            }

            #[inline(always)]
            fn fast_div_mask(&self, mask: Self::MaskType) -> Self {
                (*self as $ty).fast_div_mask(mask) as usize
            }
            #[inline(always)]
            fn fast_mod_mask(&self, d: Self, mask: Self::MaskType) -> Self {
                (*self as $ty).fast_mod_mask(d as $ty, mask) as usize
            }
            #[inline(always)]
            fn compute_mask_fast(&self) -> Self::MaskType {
                (*self as $ty).compute_mask_fast() as Self::MaskType
            }
            #[inline(always)]
            fn fast_is_divisible_mask(&self, mask: Self::MaskType) -> bool {
                (*self as $ty).fast_is_divisible_mask(mask)
            }
        }
    };
}

impl_usize!(u64, "64", u128);
impl_usize!(u32, "32", u64);
impl_usize!(u16, "16", u32);
