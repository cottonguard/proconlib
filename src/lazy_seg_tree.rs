pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

pub trait Map<T> {
    fn map(&self, x: T) -> T;
}
pub struct LazySegTree<T, F> {
    ss: Box<[T]>,
    fs: Box<[F]>,
}
impl<T: Monoid, F: Monoid + Map<T>> LazySegTree<T, F> {
    pub fn new(n: usize) -> Self {
        use std::iter::repeat_with;
        let len = 2 * n.next_power_of_two();
        Self {
            ss: repeat_with(T::id).take(len).collect(),
            fs: repeat_with(F::id).take(len).collect(),
        }
    }
    fn len(&self) -> usize {
        self.ss.len() / 2
    }
    fn propagate(&mut self, i: usize) {
        let h = 8 * std::mem::size_of::<usize>() as u32 - i.leading_zeros();
        for k in (1..h).rev() {
            let p = i >> k;
            let l = 2 * p;
            let r = 2 * p + 1;
            self.ss[l] = self.fs[p].map(std::mem::replace(&mut self.ss[l], T::id()));
            self.ss[r] = self.fs[p].map(std::mem::replace(&mut self.ss[r], T::id()));
            self.fs[l] = self.fs[p].op(&self.fs[l]);
            self.fs[r] = self.fs[p].op(&self.fs[r]);
            self.fs[p] = F::id();
        }
    }
    pub fn prod(&mut self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r <= self.len());
        let mut l = l + self.len();
        let mut r = r + self.len();
        self.propagate(l >> l.trailing_zeros());
        self.propagate((r >> r.trailing_zeros()) - 1);
        let mut lv = T::id();
        let mut rv = T::id();
        while l < r {
            if l % 2 == 1 {
                lv = lv.op(&self.ss[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                rv = rv.op(&self.ss[r]);
            }
            l /= 2;
            r /= 2;
        }
        lv.op(&rv)
    }
    pub fn set(&mut self, i: usize, v: T) {
        let mut i = i + self.len();
        self.propagate(i);
        self.ss[i] = v;
        while i > 1 {
            i /= 2;
            self.ss[i] = self.ss[2 * i].op(&self.ss[2 * i + 1]);
        }
    }
    pub fn apply(&mut self, l: usize, r: usize, f: &F) {
        assert!(l <= r);
        assert!(r <= self.len());
        let mut li = l + self.len();
        let mut ri = r + self.len();
        let ln = li >> li.trailing_zeros();
        let rn = ri >> ri.trailing_zeros();
        self.propagate(ln);
        self.propagate(rn - 1);
        while li < ri {
            if li % 2 == 1 {
                self.fs[li] = f.op(&self.fs[li]);
                self.ss[li] = f.map(std::mem::replace(&mut self.ss[li], T::id()));
                li += 1;
            }
            if ri % 2 == 1 {
                ri -= 1;
                self.fs[ri] = f.op(&self.fs[ri]);
                self.ss[ri] = f.map(std::mem::replace(&mut self.ss[ri], T::id()));
            }
            li /= 2;
            ri /= 2;
        }
        let mut l = (l + self.len()) / 2;
        let mut r = (r + self.len() - 1) / 2;
        while l > 0 {
            if l < ln {
                self.ss[l] = self.ss[2 * l].op(&self.ss[2 * l + 1]);
            }
            if
            /*l != r && */
            r < rn - 1 {
                self.ss[r] = self.ss[2 * r].op(&self.ss[2 * r + 1]);
            }
            l /= 2;
            r /= 2;
        }
    }
}

impl<T: Monoid, F: Monoid + Map<T>> std::iter::FromIterator<T> for LazySegTree<T, F> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ss: Vec<_> = iter.into_iter().collect();
        let iter_n = ss.len();
        let n = iter_n.next_power_of_two();
        ss.splice(..0, std::iter::repeat_with(T::id).take(n));
        ss.extend(std::iter::repeat_with(T::id).take(n - iter_n));
        debug_assert_eq!(ss.len(), 2 * n);
        for i in (1..n).rev() {
            ss[i] = ss[2 * i].op(&ss[2 * i + 1]);
        }
        Self {
            ss: ss.into(),
            fs: std::iter::repeat_with(F::id).take(2 * n).collect(),
        }
    }
}
