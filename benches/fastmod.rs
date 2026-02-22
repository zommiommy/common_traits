#![feature(test)]
#![allow(deprecated)]
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

extern crate test;
use common_traits::*;
use test::{Bencher, black_box};

#[bench]
fn test_u8_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u8>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u8>::MAX);
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u8_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u8>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u8>::MAX);
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u8_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u8>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u8>::MAX);
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u8_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u8>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u8>::MAX);
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u8_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u8>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u8>::MAX);
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u16_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u16>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u16>::MAX);
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u16_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u16>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u16>::MAX);
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u16_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u16>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u16>::MAX);
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u16_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u16>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u16>::MAX);
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u16_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u16>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u16>::MAX);
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u32_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u32>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u32>::MAX);
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u32_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u32>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u32>::MAX);
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u32_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u32>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u32>::MAX);
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u32_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u32>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u32>::MAX);
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u32_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u32>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u32>::MAX);
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u64_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u64>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u64>::MAX);
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u64_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u64>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u64>::MAX);
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u64_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u64>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u64>::MAX);
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u64_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u64>::MAX);
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.random_range(0..<u64>::MAX);
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u64_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.random_range(2..<u64>::MAX);
    b.iter(|| {
        let a = rng.random_range(0..<u64>::MAX);
        black_box(a).fast_range(black_box(d))
    })
}
