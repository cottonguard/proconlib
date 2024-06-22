use crate::modint2::*;
use crate::random::*;
use crate::segtree::*;

#[test]
fn affine_random() {
    const N: usize = 13;
    let mut rng = Xoshiro::seed_from_u64(123);
    let mut naive: Vec<Affine> = (0..N)
        .map(|_| {
            let a = rng.range(0, M);
            let b = rng.range(0, M);
            affine(mint(a), mint(b))
        })
        .collect();
    let mut st = SegTree::from(naive.clone());
    for _ in 0..1000 {
        let ty = rng.range(0, 2);
        match ty {
            0 => {
                let i = rng.range_inclusive(0, N);
                let j = rng.range_inclusive(0, N);
                let (l, r) = if i <= j { (i, j) } else { (j, i) };
                let mut sum_naive = Affine::id();
                for a in &naive[l..r] {
                    sum_naive = sum_naive.op(a);
                }
                assert_eq!(st.sum(l, r), sum_naive);
            }
            _ => {
                let i = rng.range(0, N);
                let a = rng.range(0, M);
                let b = rng.range(0, M);
                let v = affine(mint(a), mint(b));
                st.set(i, v);
                naive[i] = v;
            }
        }
    }
}

const M: u32 = 998244353;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Affine {
    a: Mint<M>,
    b: Mint<M>,
}
fn affine(a: Mint<M>, b: Mint<M>) -> Affine {
    Affine { a, b }
}
impl Monoid for Affine {
    fn id() -> Self {
        affine(mint(1), mint(0))
    }
    fn op(&self, other: &Self) -> Self {
        // a_l(a_r x + b_r) + b_l
        affine(self.a * other.a, self.a * other.b + self.b)
    }
}
