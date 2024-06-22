use std::mem;

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

pub trait Map<T> {
    fn map(&self, value: &T) -> T;
}

pub struct LazySegTree<T, F> {
    a: Vec<T>,
    f: Vec<F>,
}

impl<T: Monoid, F: Map<T> + Monoid> LazySegTree<T, F> {
    pub fn new(len: usize) -> Self {
        Self {
            a: (0..2 * len).map(|_| T::id()).collect(),
            f: (0..len).map(|_| F::id()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.a.len() / 2
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        if l == r {
            return T::id();
        }
        let mut l = l + self.len();
        l >>= l.trailing_zeros();
        let mut r = r + self.len();
        r >>= r.trailing_zeros();
        let mut sum_l = T::id();
        let mut sum_r = T::id();
        loop {
            if l >= r {
                let mut i = l / 2;
                sum_l = sum_l.op(&self.a[l]);
                l += 1;
                l >>= l.trailing_zeros();
                while i > l / 2 {
                    sum_l = self.f[i].map(&sum_l);
                    i /= 2;
                }
            } else {
                let mut i = r / 2;
                r -= 1;
                sum_r = self.a[r].op(&sum_r);
                r >>= r.trailing_zeros();
                while i > r / 2 {
                    sum_r = self.f[i].map(&sum_r);
                    i /= 2;
                }
            }
            if l == r {
                break;
            }
        }
        let mut sum = sum_l.op(&sum_r);
        let mut i = l / 2;
        while i > 0 {
            sum = self.f[i].map(&sum);
            i /= 2;
        }
        sum
    }

    pub fn apply(&mut self, l: usize, r: usize, f: F) {
        if l == r {
            return;
        }
        let mut l = l + self.len();
        l >>= l.trailing_zeros();
        let mut r = r + self.len();
        r >>= r.trailing_zeros();
        let pl = l / 2;
        let pr = (r - 1) / 2;
        let (p0, p1) = if pl < pr { (pl, pr) } else { (pr, pl) };
        let d0 = usize::BITS - p0.leading_zeros();
        let d1 = usize::BITS - p1.leading_zeros();
        // let d0 = usize::BITS - ((p1 >> (d1 - d0)) ^ p0).leading_zeros();
        self.propagate(p0, d0);
        self.propagate(p1, d1);
        loop {
            if l >= r {
                self.a[l] = f.map(&self.a[l]);
                if let Some(fl) = self.f.get_mut(l) {
                    *fl = f.op(fl);
                }
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                self.a[r] = f.map(&self.a[r]);
                if let Some(fr) = self.f.get_mut(r) {
                    *fr = f.op(fr);
                }
                r >>= r.trailing_zeros();
            }
            if l == r {
                break;
            }
        }
        self.update_path(p0, d0);
        self.update_path(p1, d1);
    }

    fn propagate(&mut self, i: usize, d: u32) {
        for k in (0..d).rev() {
            let p = i >> k;
            let l = 2 * p;
            let r = 2 * p + 1;
            let f = mem::replace(&mut self.f[p], F::id());
            self.a[l] = f.map(&self.a[l]);
            if let Some(fl) = self.f.get_mut(l) {
                *fl = f.op(fl);
            }
            self.a[r] = f.map(&self.a[r]);
            if let Some(fr) = self.f.get_mut(r) {
                *fr = f.op(fr);
            }
        }
    }

    fn update_path(&mut self, mut i: usize, d: u32) {
        for _ in 0..d {
            self.a[i] = self.a[2 * i].op(&self.a[2 * i + 1]);
            i /= 2;
        }
    }
}

impl<T: Monoid, F: Monoid> From<Vec<T>> for LazySegTree<T, F> {
    fn from(mut a: Vec<T>) -> Self {
        let len = a.len();
        a.reserve(len);
        let ptr = a.as_mut_ptr();
        unsafe {
            ptr.copy_to(ptr.add(len), len);
            for i in (1..len).rev() {
                ptr.add(i)
                    .write(T::op(&*ptr.add(2 * i), &*ptr.add(2 * i + 1)));
            }
            ptr.write(T::id());
            a.set_len(2 * len);
        }
        let f = (0..len).map(|_| F::id()).collect();
        Self { a, f }
    }
}

/*
|_______________________________|
|_______________|_______________|
|_______|_______|_______|_______|
|___|___|___|___|___|___|___|___|
|_|_|_|_|_|_|_|_|_|_|_|_|_|_|_|_|
*/
/*
|+++++++++++++++++++++++++++++++|
|+++++++++++++++|+++++++++++++++|
|+++++++|=======|=======|+++++++|
|+++|===|___|___|___|___|+++|___|
|_|=|_|_|_|_|_|_|_|_|_|_|=|_|_|_|
*/

/*
   1
 2   3
4 5
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modint2::*;
    use crate::random::*;

    #[test]
    fn affine_sum_simple() {
        let init: Vec<Mint<M>> = vec![mint(1), mint(2), mint(3)];
        affine_sum_impl(
            init,
            [
                Cmd::Sum { l: 0, r: 1 },
                Cmd::Sum { l: 0, r: 2 },
                Cmd::Sum { l: 0, r: 3 },
                Cmd::Sum { l: 1, r: 2 },
                Cmd::Sum { l: 1, r: 3 },
                Cmd::Apply {
                    l: 0,
                    r: 1,
                    f: affine(mint(1), mint(100)),
                },
                Cmd::Sum { l: 0, r: 1 },
                Cmd::Sum { l: 0, r: 2 },
                Cmd::Sum { l: 0, r: 3 },
                Cmd::Sum { l: 1, r: 2 },
                Cmd::Sum { l: 1, r: 3 },
                Cmd::Apply {
                    l: 0,
                    r: 3,
                    f: affine(mint(2), mint(10000)),
                },
                Cmd::Sum { l: 0, r: 1 },
                Cmd::Sum { l: 0, r: 2 },
                Cmd::Sum { l: 0, r: 3 },
                Cmd::Sum { l: 1, r: 2 },
                Cmd::Sum { l: 1, r: 3 },
                Cmd::Apply {
                    l: 1,
                    r: 2,
                    f: affine(mint(3), mint(1000000)),
                },
                Cmd::Sum { l: 0, r: 1 },
                Cmd::Sum { l: 0, r: 2 },
                Cmd::Sum { l: 0, r: 3 },
                Cmd::Sum { l: 1, r: 2 },
                Cmd::Sum { l: 1, r: 3 },
            ],
        )
    }

    #[test]
    fn affine_sum_random() {
        const N: usize = 13;
        let mut rng = Xoshiro::seed_from_u64(123);
        let init: Vec<Mint<M>> = (0..N).map(|_| mint(rng.range(0, M))).collect();
        affine_sum_impl(
            init,
            (0..1000).map(|_| {
                let ty = rng.range(0, 2);
                match ty {
                    0 => {
                        let i = rng.range_inclusive(0, N);
                        let j = rng.range_inclusive(0, N);
                        let (l, r) = if i <= j { (i, j) } else { (j, i) };
                        Cmd::Sum { l, r }
                    }
                    _ => {
                        let i = rng.range_inclusive(0, N);
                        let j = rng.range_inclusive(0, N);
                        let (l, r) = if i <= j { (i, j) } else { (j, i) };
                        let a = rng.range(0, M);
                        let b = rng.range(0, M);
                        let f = affine(mint(a), mint(b));
                        Cmd::Apply { l, r, f }
                    }
                }
            }),
        )
    }

    fn affine_sum_impl(init: Vec<Mint<M>>, cmd: impl IntoIterator<Item = Cmd>) {
        let mut naive = init;
        let mut st = LazySegTree::from(naive.iter().map(|x| (*x, mint(1))).collect::<Vec<_>>());
        for cmd in cmd {
            match cmd {
                Cmd::Sum { l, r } => {
                    let mut sum_naive = mint(0);
                    for a in &naive[l..r] {
                        sum_naive += a;
                    }
                    assert_eq!(st.sum(l, r).0, sum_naive, "l={l} r={r}");
                }
                Cmd::Apply { l, r, f } => {
                    st.apply(l, r, f);
                    for x in &mut naive[l..r] {
                        *x = f.a * *x + f.b;
                    }
                }
            }
        }
    }

    enum Cmd {
        Sum { l: usize, r: usize },
        Apply { l: usize, r: usize, f: Affine },
    }

    const M: u32 = 998244353;
    impl Monoid for (Mint<M>, Mint<M>) {
        fn id() -> Self {
            (mint(0), mint(0))
        }
        fn op(&self, other: &Self) -> Self {
            (self.0 + other.0, self.1 + other.1)
        }
    }
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
    impl Map<(Mint<M>, Mint<M>)> for Affine {
        fn map(&self, x: &(Mint<M>, Mint<M>)) -> (Mint<M>, Mint<M>) {
            (self.a * x.0 + self.b * x.1, x.1)
        }
    }
}
