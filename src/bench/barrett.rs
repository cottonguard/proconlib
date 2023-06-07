extern crate test;

use crate::random::*;
use test::Bencher;

use crate::experimental::BarrettReduction;

#[bench]
fn barrett_small(b: &mut Bencher) {
    rem_small(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            let br = BarrettReduction::new(*m);
            for x in xs {
                sum = sum.wrapping_add(br.rem(*x as u64));
            }
        }
        sum
    })
}

#[bench]
fn native_small(b: &mut Bencher) {
    rem_small(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            for x in xs {
                sum = sum.wrapping_add(x % *m);
            }
        }
        sum
    })
}

#[bench]
fn barrett_u32(b: &mut Bencher) {
    rem_u32(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            let br = BarrettReduction::new(*m);
            for x in xs {
                sum = sum.wrapping_add(br.rem(*x as u64));
            }
        }
        sum
    })
}

#[bench]
fn native_u32(b: &mut Bencher) {
    rem_u32(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            for x in xs {
                sum = sum.wrapping_add(x % *m);
            }
        }
        sum
    })
}

#[bench]
fn barrett_u64(b: &mut Bencher) {
    rem_u64(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            let br = BarrettReduction::new(*m);
            for x in xs {
                sum = sum.wrapping_add(br.rem(*x));
            }
        }
        sum
    })
}

#[bench]
fn native_u64(b: &mut Bencher) {
    rem_u64(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            for x in xs {
                sum = sum.wrapping_add((x % *m as u64) as u32);
            }
        }
        sum
    })
}

#[bench]
fn barrett_global_u64(b: &mut Bencher) {
    rem_u64(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            let br = BarrettReduction::new(*m);
            br.store();
            for x in xs {
                let br = BarrettReduction::load();
                sum = sum.wrapping_add(br.rem(*x));
            }
        }
        sum
    })
}

#[bench]
fn barrett_tls_u64(b: &mut Bencher) {
    rem_u64(b, |ms, xs| {
        let mut sum = 0u32;
        for m in ms {
            let br = BarrettReduction::new(*m);
            br.store_thread();
            for x in xs {
                let br = BarrettReduction::load_thread();
                sum = sum.wrapping_add(br.rem(*x));
            }
        }
        sum
    })
}

fn rem_small(b: &mut Bencher, f: impl Fn(&[u32], &[u32]) -> u32) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let n = 1000;
    let ms: Vec<u32> = (0..n).map(|_| rng.range(2, 1024)).collect();
    let xs: Vec<u32> = (0..n).map(|_| rng.range(0, 1024)).collect();
    b.iter(|| f(&ms, &xs));
}

fn rem_u32(b: &mut Bencher, f: impl Fn(&[u32], &[u32]) -> u32) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let n = 1000;
    let ms: Vec<u32> = (0..n).map(|_| rng.range_inclusive(2, !0)).collect();
    let xs: Vec<u32> = (0..n).map(|_| rng.gen()).collect();
    b.iter(|| f(&ms, &xs));
}

fn rem_u64(b: &mut Bencher, f: impl Fn(&[u32], &[u64]) -> u32) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let n = 1000;
    let ms: Vec<u32> = (0..n).map(|_| rng.range_inclusive(2, !0)).collect();
    let xs: Vec<u64> = (0..n).map(|_| rng.gen()).collect();
    b.iter(|| f(&ms, &xs));
}
