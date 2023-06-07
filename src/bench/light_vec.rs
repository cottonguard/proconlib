extern crate test;

use crate::light_vec::*;
use crate::random::*;
use test::Bencher;

#[bench]
fn many_small_bytes_light_vec_16(b: &mut Bencher) {
    many_bytes(b, 16, |a| LightVec::<u8, 16>::from(a));
}

#[bench]
fn many_small_bytes_vec(b: &mut Bencher) {
    many_bytes(b, 16, |a| Vec::from(a));
}

#[bench]
fn many_mixed_bytes_light_vec_16(b: &mut Bencher) {
    many_bytes(b, 32, |a| LightVec::<u8, 16>::from(a));
}

#[bench]
fn many_mixed_bytes_vec(b: &mut Bencher) {
    many_bytes(b, 32, |a| Vec::from(a));
}

fn many_bytes<T: AsRef<[u8]>>(b: &mut Bencher, n: usize, f: impl Fn(&[u8]) -> T) {
    const N: usize = 200000;
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut sum = 0;
    let idx: Vec<u32> = (0..=N)
        .map(|_| {
            let idx = sum;
            sum += rng.range_inclusive(1, n as u32);
            idx
        })
        .collect();
    let buf: Vec<u8> = (0..*idx.last().unwrap()).map(|_| rng.gen()).collect();
    b.iter(|| {
        let mut res = vec![];
        for idx in idx.windows(2) {
            res.push(f(&buf[idx[0] as usize..idx[1] as usize]));
        }
        res
    });
}
