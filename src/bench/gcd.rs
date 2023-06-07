extern crate test;

use crate::experimental::*;
use crate::random::*;
use test::Bencher;

#[bench]
fn gcd_binary_random(b: &mut Bencher) {
    gcd_random(b, |x, y| gcd_u32(x, y));
}

#[bench]
fn gcd_euclid_random(b: &mut Bencher) {
    gcd_random(b, |x, y| gcd_u32_euclid(x, y));
}

#[bench]
fn gcd_binary_gcd_around_1e5(b: &mut Bencher) {
    gcd_random_gcd_around_1e5(b, |x, y| gcd_u32(x, y));
}

#[bench]
fn gcd_euclid_gcd_around_1e5(b: &mut Bencher) {
    gcd_random_gcd_around_1e5(b, |x, y| gcd_u32_euclid(x, y));
}

fn gcd_random(b: &mut Bencher, f: impl Fn(u32, u32) -> u32) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let n = 1000000;
    let cases: Vec<(u32, u32)> = (0..n).map(|_| (rng.gen(), rng.gen())).collect();
    b.iter(|| {
        cases
            .iter()
            .map(|&(x, y)| f(x, y))
            .fold(0, |acc, x| acc ^ x)
    });
}

fn gcd_random_gcd_around_1e5(b: &mut Bencher, f: impl Fn(u32, u32) -> u32) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let n = 1000000;
    let cases: Vec<(u32, u32)> = (0..n)
        .map(|_| {
            let d = rng.range(1, 100000);
            let a = u32::MAX / d;
            (d * rng.range(0, a), d * rng.range(0, a))
        })
        .collect();
    b.iter(|| {
        cases
            .iter()
            .map(|&(x, y)| f(x, y))
            .fold(0, |acc, x| acc ^ x)
    });
}
