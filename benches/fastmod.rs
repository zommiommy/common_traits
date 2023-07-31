#![feature(test)]
#![allow(deprecated)]
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

extern crate test;
use common_traits::*;
use test::{black_box, Bencher};

#[bench]
fn test_div_u8(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        let d = rng.gen_range(2..<u8>::MAX) as u8;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_mod_u8(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        let d = rng.gen_range(2..<u8>::MAX) as u8;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_fast_div_u8(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        let d = rng.gen_range(2..<u8>::MAX) as u8;
        black_box(a).fast_div(black_box(d))
    })
}
#[bench]
fn test_fast_mod_u8(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        let d = rng.gen_range(2..<u8>::MAX) as u8;
        black_box(a).fast_mod(black_box(d))
    })
}
#[bench]
fn test_fast_range_u8(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u8>::MAX) as u8;
        let d = rng.gen_range(2..<u8>::MAX) as u8;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_div_u16(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        let d = rng.gen_range(2..<u16>::MAX) as u16;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_mod_u16(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        let d = rng.gen_range(2..<u16>::MAX) as u16;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_fast_div_u16(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        let d = rng.gen_range(2..<u16>::MAX) as u16;
        black_box(a).fast_div(black_box(d))
    })
}
#[bench]
fn test_fast_mod_u16(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        let d = rng.gen_range(2..<u16>::MAX) as u16;
        black_box(a).fast_mod(black_box(d))
    })
}
#[bench]
fn test_fast_range_u16(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u16>::MAX) as u16;
        let d = rng.gen_range(2..<u16>::MAX) as u16;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_div_u32(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        let d = rng.gen_range(2..<u32>::MAX) as u32;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_mod_u32(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        let d = rng.gen_range(2..<u32>::MAX) as u32;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_fast_div_u32(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        let d = rng.gen_range(2..<u32>::MAX) as u32;
        black_box(a).fast_div(black_box(d))
    })
}
#[bench]
fn test_fast_mod_u32(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        let d = rng.gen_range(2..<u32>::MAX) as u32;
        black_box(a).fast_mod(black_box(d))
    })
}
#[bench]
fn test_fast_range_u32(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u32>::MAX) as u32;
        let d = rng.gen_range(2..<u32>::MAX) as u32;
        black_box(a).fast_range(black_box(d))
    })
}

#[bench]
fn test_div_u64(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        let d = rng.gen_range(2..<u64>::MAX) as u64;
        black_box(a) / black_box(d)
    })
}
#[bench]
fn test_mod_u64(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        let d = rng.gen_range(2..<u64>::MAX) as u64;
        black_box(a) % black_box(d)
    })
}

#[bench]
fn test_fast_div_u64(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        let d = rng.gen_range(2..<u64>::MAX) as u64;
        black_box(a).fast_div(black_box(d))
    })
}
#[bench]
fn test_fast_mod_u64(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        let d = rng.gen_range(2..<u64>::MAX) as u64;
        black_box(a).fast_mod(black_box(d))
    })
}
#[bench]
fn test_fast_range_u64(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(0);
    b.iter(|| {
        let a = rng.gen_range(0..<u64>::MAX) as u64;
        let d = rng.gen_range(2..<u64>::MAX) as u64;
        black_box(a).fast_range(black_box(d))
    })
}
