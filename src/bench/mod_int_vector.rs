extern crate test;

use crate::def_mint;
use crate::mod_int::*;
use crate::mod_int_vector::*;
use crate::random::*;
use test::Bencher;

def_mint!(998244353);

#[bench]
fn sum_simd(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut v = vec![];
    for _ in 0..1000000 {
        v.push(mint(rng.range(0, MintModulo::modulo())));
    }
    dbg!(sum(&v));
    b.iter(|| sum(&v));
}

#[bench]
fn sum_naive(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut v = vec![];
    for _ in 0..1000000 {
        v.push(mint(rng.range(0, MintModulo::modulo())));
    }
    dbg!(v.iter().sum::<Mint>());
    b.iter(|| v.iter().sum::<Mint>());
}

#[bench]
fn product_simd(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut v = vec![];
    for _ in 0..1000000 {
        v.push(mint(rng.range(1, MintModulo::modulo())));
    }
    dbg!(product(&v));
    b.iter(|| product(&v));
}

#[bench]
fn product_naive(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut v = vec![];
    for _ in 0..1000000 {
        v.push(mint(rng.range(1, MintModulo::modulo())));
    }
    dbg!(v.iter().product::<Mint>());
    b.iter(|| v.iter().product::<Mint>());
}

#[bench]
fn dot_simd(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut u = vec![];
    let mut v = vec![];
    for _ in 0..1000000 {
        u.push(mint(rng.range(0, MintModulo::modulo())));
    }
    for _ in 0..1000000 {
        v.push(mint(rng.range(0, MintModulo::modulo())));
    }
    dbg!(dot(&u, &v));
    b.iter(|| dot(&u, &v));
}

#[bench]
fn dot_naive(b: &mut Bencher) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut u = vec![];
    let mut v = vec![];
    for _ in 0..1000000 {
        u.push(mint(rng.range(0, MintModulo::modulo())));
    }
    for _ in 0..1000000 {
        v.push(mint(rng.range(0, MintModulo::modulo())));
    }
    dbg!(dot_naive_impl(&u, &v));
    b.iter(|| dot_naive_impl(&u, &v));
}

fn dot_naive_impl<M: Modulo>(a: &[ModInt<M>], b: &[ModInt<M>]) -> ModInt<M> {
    let sum: u64 = a
        .iter()
        .zip(b)
        .map(|(a, b)| a.get() as u64 * b.get() as u64)
        .sum();
    ModInt::new((sum % M::modulo() as u64) as u32)
}
