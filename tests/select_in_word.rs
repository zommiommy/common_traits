#![cfg(test)]
use common_traits::SelectInWord;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::{Rng, RngCore};

#[test]
fn test_select_in_word_sparse() {
    let mut rng = SmallRng::seed_from_u64(0);
    macro_rules! impl_test {
        ($ty:ty) => {
            for _ in 0..100 {
                let ones = rng.gen_range(1..10);
                let mut word: $ty = 0;
                for _ in 0..ones {
                    word |= 1 << rng.gen_range(0..<$ty>::BITS);
                }
                let mut new_word = word;

                for i in 0..word.count_ones() {
                    assert_eq!(
                        word.select_in_word(i as _),
                        new_word.trailing_zeros() as usize,
                    );
                    new_word &= new_word - 1;
                }
            }
        };
    }
    impl_test!(u8);
    impl_test!(u16);
    impl_test!(u32);
    impl_test!(u64);
    impl_test!(usize);
    impl_test!(u128);
}

#[test]
fn test_select_in_word() {
    let mut rng = SmallRng::seed_from_u64(0);
    macro_rules! impl_test {
        ($ty:ty) => {
            for _ in 0..100 {
                let word: $ty = (((rng.next_u64() as u128) << 64) | rng.next_u64() as u128) as $ty;
                let mut new_word = word;
                for i in 0..word.count_ones() {
                    assert_eq!(
                        word.select_in_word(i as _),
                        new_word.trailing_zeros() as usize,
                    );
                    new_word &= new_word - 1;
                }
            }
        };
    }
    impl_test!(u8);
    impl_test!(u16);
    impl_test!(u32);
    impl_test!(u64);
    impl_test!(usize);
    impl_test!(u128);
}

#[test]
fn test_select_zero_in_word() {
    let mut rng = SmallRng::seed_from_u64(0);
    macro_rules! impl_test {
        ($ty:ty) => {
            for _ in 0..100 {
                let word: $ty = (((rng.next_u64() as u128) << 64) | rng.next_u64() as u128) as $ty;
                let mut new_word = !word;
                for i in 0..word.count_zeros() {
                    assert_eq!(
                        word.select_zero_in_word(i as _),
                        new_word.trailing_zeros() as usize,
                    );
                    new_word &= new_word - 1;
                }
            }
        };
    }
    impl_test!(u8);
    impl_test!(u16);
    impl_test!(u32);
    impl_test!(u64);
    impl_test!(usize);
    impl_test!(u128);
}
