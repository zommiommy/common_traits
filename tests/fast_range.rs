#![cfg(test)]
use common_traits::FastRange;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

#[test]
fn test_fast_mod() {
    let mut rng = SmallRng::seed_from_u64(0);
    macro_rules! impl_test {
        ($ty:ty) => {
            for _ in 0..100_000 {
                let a = rng.random_range(0..<$ty>::MAX) as $ty;
                let d = rng.random_range(2..<$ty>::MAX) as $ty;

                let div_mask = d.compute_mask_fast();

                let div = a / d;
                assert_eq!(div, a.fast_div_mask(div_mask), "failed div with mask");
                assert_eq!(div, a.fast_div(d), "failed div");

                let rem = a % d;
                assert_eq!(rem, a.fast_mod_mask(d, div_mask), "failed mod with mask");
                assert_eq!(rem, a.fast_mod(d), "failed mod");

                assert_eq!(
                    rem == 0,
                    a.fast_is_divisible_mask(div_mask),
                    "failed is_divisible with mask"
                );
                assert_eq!(rem == 0, a.fast_is_divisible(d), "failed is_disivible");

                assert!(a.fast_range(d) < d);
            }
        };
    }
    impl_test!(u8);
    impl_test!(u16);
    impl_test!(u32);
    impl_test!(u64);
    impl_test!(usize);
    //impl_test!(u128);
}
