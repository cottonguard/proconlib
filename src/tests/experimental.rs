mod matrix;

use crate::experimental::*;

#[test]
fn n_ones_test() {
    const N: usize = 8;
    let mut binom = [[0; N]; N];
    for i in 0..N {
        binom[i][0] = 1;
        for j in 1..=i {
            binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
        }
    }
    for i in 0..N {
        for j in 0..N {
            let mut prev = None;
            let mut count = 0;
            for x in n_ones(i as _, j as _) {
                count += 1;
                assert_eq!(x.count_ones(), i as _);
                assert_eq!(x & (1 << j), 0);
                if let Some(prev) = prev {
                    assert!(x > prev);
                }
                prev = Some(x);
            }
            assert_eq!(count, binom[j][i]);
        }
    }
}

#[test]
fn atoi() {
    let pats = [
        &b"0"[..],
        &b"1"[..],
        &b"100"[..],
        &b"123456789"[..],
        &b"123456789012345"[..],
    ];
    for p in pats {
        assert_eq!(a_to_u64_be(p), a_to_u64_naive(p));
        assert_eq!(a_to_u64_le(p), a_to_u64_naive(p));
    }
}

#[test]
fn gcd() {
    assert_eq!(gcd_u32(1, 1), 1);
    assert_eq!(gcd_u32(2, 3), 1);
    assert_eq!(gcd_u32(3, 3), 3);
    assert_eq!(gcd_u32(2, 4), 2);
    assert_eq!(gcd_u32(4, 6), 2);
    assert_eq!(gcd_u32(0, 5), 5);
    assert_eq!(gcd_u32(5, 0), 5);
    assert_eq!(gcd_u32(0, 0), 0);
    assert_eq!(gcd_u32(3000, 2000), 1000);
    assert_eq!(gcd_u32(111111, 111), 111);
    assert_eq!(gcd_u32(111111111, 111), 111);
    assert_eq!(gcd_u32(3 << 20, 2 << 20), 1 << 20);
}

#[test]
fn burrett_reduction() {
    for m in 2..=100 {
        let br = BarrettReduction::new(m);
        for x in 1..=100 {
            assert_eq!(br.div(x), x / m as u64);
            assert_eq!(br.rem(x), x as u32 % m);
        }
    }
}

#[test]
fn burrett_reduction_random() {
    use crate::random::*;
    let mut rng = Xoshiro::seed_from_u64(1);
    for _ in 0..200 {
        let m = rng.gen();
        let br = BarrettReduction::new(m);
        for _ in 0..200 {
            let x = rng.gen();
            assert_eq!(br.div(x), x / m as u64, "x={} m={}", x, m);
            assert_eq!(br.rem(x), (x % m as u64) as u32, "x={} m={}", x, m,);
        }
    }
}

#[test]
fn burrett_reduction_edge() {
    use crate::random::*;
    let mut rng = Xoshiro::seed_from_u64(1);
    let ms = [
        2,
        1 << 30,
        1 << 31,
        3 << 30,
        1 << 16,
        (1 << 16) - 1,
        !0,
        !0 - 1,
    ];
    for m in ms {
        let br = BarrettReduction::new(m);
        for e in 0..500 {
            let x = rng.range_inclusive(0, !0 >> (e % 64));
            assert_eq!(br.div(x), x / m as u64, "x={} m={}", x, m);
            assert_eq!(br.rem(x), (x % m as u64) as u32, "x={} m={}", x, m,);
        }
    }
}
