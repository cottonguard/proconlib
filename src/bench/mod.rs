mod barrett;
mod gcd;
mod input2;
mod light_vec;
// mod mod_int_vector;

extern crate test;

use crate::random::*;
use test::Bencher;

#[bench]
fn atoi_1e6(b: &mut Bencher) {
    use crate::experimental::a_to_u64_be;
    do_atoi(b, 1000000, a_to_u64_be);
}

#[bench]
fn atoi_le_1e6(b: &mut Bencher) {
    use crate::experimental::a_to_u64_le;
    do_atoi(b, 1000000, a_to_u64_le);
}

#[bench]
fn atoi_naive_1e6(b: &mut Bencher) {
    use crate::experimental::a_to_u64_naive;
    do_atoi(b, 1000000, a_to_u64_naive);
}

#[bench]
fn atoi_1e9(b: &mut Bencher) {
    use crate::experimental::a_to_u64_be;
    do_atoi(b, 1000000000, a_to_u64_be);
}

#[bench]
fn atoi_le_1e9(b: &mut Bencher) {
    use crate::experimental::a_to_u64_le;
    do_atoi(b, 1000000000, a_to_u64_le);
}

#[bench]
fn atoi_naive_1e9(b: &mut Bencher) {
    use crate::experimental::a_to_u64_naive;
    do_atoi(b, 1000000000, a_to_u64_naive);
}

#[bench]
fn atoi_1e18(b: &mut Bencher) {
    use crate::experimental::a_to_u64_be;
    do_atoi(b, 1000000000000000000, a_to_u64_be);
}

#[bench]
fn atoi_le_1e18(b: &mut Bencher) {
    use crate::experimental::a_to_u64_le;
    do_atoi(b, 1000000000000000000, a_to_u64_le);
}

#[bench]
fn atoi_naive_1e18(b: &mut Bencher) {
    use crate::experimental::a_to_u64_naive;
    do_atoi(b, 1000000000000000000, a_to_u64_naive);
}

#[inline(always)]
fn do_atoi(b: &mut Bencher, max: u64, f: impl Fn(&[u8]) -> u64) {
    let mut rng = Xoshiro::seed_from_u64(1);
    let cases: Vec<Vec<u8>> = (0..100000)
        .map(|_| format!("{}", rng.range(0, max)).into())
        .collect();
    b.iter(|| {
        let mut sum = 0u64;
        for s in &cases {
            sum = sum.wrapping_add(f(s));
        }
        sum
    });
}

#[bench]
fn input1(b: &mut Bencher) {
    use crate::input::*;
    do_input(b, |n, s| {
        let mut ss = SplitWs::new(s);
        ss.seq::<u64>()
            .take(n)
            .fold(0u64, |acc, x| acc.wrapping_add(x))
    });
}

#[bench]
fn input1_bytes(b: &mut Bencher) {
    use crate::input::*;
    do_input(b, |n, s| {
        let mut ss = SplitWs::new(s);
        let mut sum = 0;
        for _ in 0..n {
            sum += ss.bytes().len();
        }
        sum
    });
}

#[bench]
fn input2(b: &mut Bencher) {
    use crate::input2::*;
    do_input(b, |n, s| {
        let mut ss = Input::new(s);
        ss.seq::<u64>(n).fold(0u64, |acc, x| acc.wrapping_add(x))
    });
}

#[bench]
fn input2_bytes(b: &mut Bencher) {
    use crate::input2::*;
    do_input(b, |n, s| {
        let mut ss = Input::new(s);
        let mut sum = 0;
        for _ in 0..n {
            sum += ss.bytes().len();
        }
        sum
    });
}

fn do_input<T>(b: &mut Bencher, f: impl Fn(usize, &[u8]) -> T) {
    use std::io::Write as _;
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut case = vec![];
    let n = 1000000;
    for i in 0..n {
        if i > 0 {
            case.push(b' ');
        }
        write!(
            &mut case,
            "{}",
            rng.range(0, 1000000000000000000u64) >> rng.range(0u32, 60)
        )
        .unwrap();
    }
    b.iter(|| f(n, &case));
}
