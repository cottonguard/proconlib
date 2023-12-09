extern crate test;

use crate::input2;
use crate::random::*;
use test::Bencher;

#[bench]
fn find_ws(b: &mut Bencher) {
    bench(b, |s| input2::find_ws(s))
}

/*
#[bench]
fn find_ws_naive(b: &mut Bencher) {
    bench(b, |s| input2::find_ws_naive(s))
}
 */

#[cfg(feature = "nightly")]
#[bench]
fn find_ws_simd(b: &mut Bencher) {
    bench(b, |s| crate::experimental::find_ws_simd(s))
}

fn bench(b: &mut Bencher, f: impl Fn(&[u8]) -> Option<usize>) {
    const N: usize = 1000000;
    const P: f64 = 0.1;
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut ws = false;
    let s: Vec<u8> = (0..N)
        .map(|_| {
            if !ws && rng.gen_bool(P) {
                ws = true;
                b' '
            } else {
                ws = false;
                rng.range_inclusive(b'a', b'z')
            }
        })
        .collect();
    b.iter(|| {
        let mut i = 0;
        let mut x = 0;
        while let Some(j) = f(&s[i..]) {
            i += j + 1;
            x ^= j;
        }
        x
    });
}
