use crate::{modint2::*, modint_poly::*};

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
