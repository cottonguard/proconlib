use crate::{modint2::*, modint_poly::*, random::*};

#[test]
fn test() {
    type M = ConstMod<998244353>;
    let f = Poly::<M>::from(vec![1, 2, 3]);
    let g = Poly::<M>::from(vec![4, 5]);
    assert_eq!(&f + &g, Poly::<M>::from(vec![5, 7, 3]));
    assert_eq!(&g + &f, Poly::<M>::from(vec![5, 7, 3]));
    assert_eq!(&f - &g, Poly::<M>::from(vec![-3, -3, 3]));
    assert_eq!(&f * &g, Poly::<M>::from(vec![4, 13, 22, 15]));
}

#[test]
fn mul() {
    type M = ConstMod<998244353>;
    let short = Poly::<M>::from(vec![1; 2]);
    let long = Poly::<M>::from(vec![1; 100]);

    let mut short_mul_long = Poly::<M>::from(vec![2; 101]);
    short_mul_long[0] = ModInt::new(1);
    short_mul_long[100] = ModInt::new(1);
    assert_eq!(&short * &long, short_mul_long);
    assert_eq!(&long * &short, short_mul_long);

    let long_pow2 = Poly::<M>::from((1..=100).chain((1..100).rev()).collect::<Vec<i32>>());
    assert_eq!(&long * &long, long_pow2);
}

#[test]
fn div_rem_random() {
    type M = ConstMod<998244353>;
    let mut rng = Xoshiro::seed_from_u64(1);
    for _ in 0..100 {
        loop {
            let n = rng.range(0, 100);
            let m = rng.range(1, 100);
            let f: Poly<M> = (0..n).map(|_| ModInt::from(rng.range(-10, 10))).collect();
            let g: Poly<M> = (0..m).map(|_| ModInt::from(rng.range(-10, 10))).collect();
            if g[0].get() == 0 {
                continue;
            }
            let (q, r) = f.clone().div_rem(g.clone());
            assert!(
                r.deg() == !0 || r.deg() < g.deg(),
                "f = {f}\n g = {g}\n q = {q}\n r = {r}",
            );
            assert_eq!(f, g * q + r);
            break;
        }
    }
}
