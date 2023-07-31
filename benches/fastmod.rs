#![feature(test)]
#![allow(deprecated)]
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

extern crate test;
use common_traits::*;
use test::{black_box, Bencher};

#[bench]
fn test_u8_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u8>::MAX) as u8;
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u8_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u8>::MAX) as u8;
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u8_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u8>::MAX) as u8;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u8_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u8>::MAX) as u8;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u8_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u8>::MAX) as u8;
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u16_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u16>::MAX) as u16;
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u16_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u16>::MAX) as u16;
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u16_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u16>::MAX) as u16;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u16_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u16>::MAX) as u16;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u16_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u16>::MAX) as u16;
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u32_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u32>::MAX) as u32;
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u32_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u32>::MAX) as u32;
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u32_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u32>::MAX) as u32;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u32_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u32>::MAX) as u32;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u32_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u32>::MAX) as u32;
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_u64_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u64>::MAX) as u64;
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_u64_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u64>::MAX) as u64;
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_u64_fast_div(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u64>::MAX) as u64;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        black_box(a).fast_div_mask(black_box(mask))
    })
}
#[bench]
fn test_u64_fast_mod(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u64>::MAX) as u64;
    let mask = d.compute_mask_fast();
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        black_box(a).fast_mod_mask(black_box(d), black_box(mask))
    })
}
#[bench]
fn test_u64_fast_range(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    let d = rng.gen_range(2..<u64>::MAX) as u64;
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        black_box(a).fast_range(black_box(d))
    })
}
