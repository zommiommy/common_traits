use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! impl_non_max {
    ($ty: ty, $nzty: ty, $nmty: ident) => {
		/// Non-max version of the associated number type.
		///
		/// This is a number type whose maximum value is one less than the
		/// maximum value of the associated number type. [`Option`] values occupy
		/// the same space of the associated number type. It plays the same
		/// role of nonzero types, but with a sometimes more convenient
		/// range.
		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $nmty($nzty);

        impl $nmty {
			pub const MIN: $ty = <$ty>::MIN;
			pub const MAX: $ty = <$ty>::MAX - 1;
			pub const BITS: u32 = <$ty>::BITS;

            #[inline(always)]
            pub const fn new(value: $ty) -> Option<Self> {
				if value == <$ty>::MAX {
					None
				} else {
                	unsafe {Some(Self::new_unchecked(value)) }
				}
            }

            pub const unsafe fn new_unchecked(value: $ty) -> Self {
				Self(unsafe{<$nzty>::new_unchecked(!value)})
			}

			pub const fn get(self) -> $ty {
				! self.0.get()
			}

			pub const fn leading_zeros(self) -> u32 {
				self.get().leading_zeros()
			}

			pub const fn trailing_zeros(self) -> u32 {
				self.get().trailing_zeros()
			}

			pub const fn count_ones(self) -> u32 {
				self.get().count_ones()
			}

			pub fn checked_add(self, other: $ty) -> Option<Self> {
				Self::new(self.get().checked_add(other)?)
			}

			pub fn saturating_add(self, other: $ty) -> Self {
				unsafe { Self::new_unchecked(self.get().saturating_add(other).min(Self::MAX)) }
			}

			pub const fn ilog2(self) -> u32 {
				self.get().ilog2()
			}

			pub const fn ilog10(self) -> u32 {
				self.get().ilog10()
			}

			pub fn checked_mul(self, other: $ty) -> Option<Self> {
				Self::new(self.get().checked_mul(other)?)
			}

			pub fn saturating_mul(self, other: $ty) -> Self {
				unsafe { Self::new_unchecked(self.get().saturating_mul(other).min(Self::MAX)) }
			}

			pub fn checked_pow(self, other: u32) -> Option<Self> {
				Self::new(self.get().checked_pow(other)?)
			}

			pub fn saturating_pow(self, other: u32) -> Self {
				unsafe { Self::new_unchecked(self.get().saturating_pow(other).min(Self::MAX)) }
			}

		}
    };
}

macro_rules! impl_unsigned {
    ($ty: ty, $nzty: ty, $nmty: ident) => {
		impl_non_max!($ty, $nzty, $nmty);

        impl $nmty {
            pub fn checked_next_power_of_two(self) -> Option<Self> {
                Self::new(self.get().checked_next_power_of_two()?)
            }

            pub const fn is_power_of_two(self) -> bool {
                self.get().is_power_of_two()
            }
        }
    };
}

impl_unsigned!(u8, NonZeroU8, NonMaxU8);
impl_unsigned!(u16, NonZeroU16, NonMaxU16);
impl_unsigned!(u32, NonZeroU32, NonMaxU32);
impl_unsigned!(u64, NonZeroU64, NonMaxU64);
impl_unsigned!(u128, NonZeroU128, NonMaxU128);
impl_unsigned!(usize, NonZeroUsize, NonMaxUsize);

impl_non_max!(i8, NonZeroI8, NonMaxI8);
impl_non_max!(i16, NonZeroI16, NonMaxI16);
impl_non_max!(i32, NonZeroI32, NonMaxI32);
impl_non_max!(i64, NonZeroI64, NonMaxI64);
impl_non_max!(i128, NonZeroI128, NonMaxI128);
impl_non_max!(isize, NonZeroIsize, NonMaxIsize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_max() {
        let x = NonMaxU8::new(255);
        assert!(x.is_none());
        let x = NonMaxU8::new(254);
        assert!(x.is_some());
        assert_eq!(x.unwrap().get(), 254);

        assert_eq!(NonMaxI16::MIN, i16::MIN);
        assert_eq!(NonMaxI16::MAX, i16::MAX - 1);

        assert_eq!(
            std::mem::size_of::<usize>(),
            std::mem::size_of::<Option<NonMaxUsize>>(),
        );

        assert_eq!(
            std::mem::size_of::<u8>(),
            std::mem::size_of::<Option<NonMaxU8>>(),
        );
    }
}
