use std::{fmt, ops::Deref};

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

#[derive(Clone)]
pub struct SegTree<T>(Vec<T>);

impl<T: Monoid> SegTree<T> {
    pub fn new(n: usize) -> Self {
        Self((0..2 * n - 1).map(|_| T::id()).collect())
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        let mut l = l + self.len();
        let mut r = r + self.len();
        assert!(r <= self.0.len() + 1, "out of range");
        let mut xl = T::id();
        let mut xr = T::id();
        while l < r {
            if r & 1 == 1 {
                r -= 1;
                xr = self.0[r - 1].op(&xr);
            }
            if l & 1 == 1 {
                xl = xl.op(&self.0[l - 1]);
                l += 1;
            }
            l /= 2;
            r /= 2;
        }
        xl.op(&xr)
    }

    pub fn update(&mut self, i: usize, f: impl FnOnce(&mut T)) {
        assert!(i < self.len());
        let mut i = i + self.len();
        f(&mut self.0[i - 1]);
        i /= 2;
        while i >= 1 {
            let res = self.0[2 * i - 1].op(&self.0[2 * i]);
            self.0[i - 1] = res;
            i /= 2;
        }
    }

    pub fn set(&mut self, i: usize, x: T) {
        self.update(i, |val| *val = x);
    }

    pub fn add(&mut self, i: usize, x: T) {
        self.update(i, |val| *val = val.op(&x));
    }

    #[inline]
    fn build(mut a: Vec<T>) -> Self {
        let n = (a.len() + 1) / 2;
        for i in (1..n).rev() {
            a[i - 1] = a[2 * i - 1].op(&a[2 * i]);
        }
        Self(a)
    }
}

impl<T> Deref for SegTree<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0[self.0.len() / 2..]
    }
}

impl<T: Monoid + Clone> From<&[T]> for SegTree<T> {
    fn from(a: &[T]) -> Self {
        let a = (0..a.len() - 1)
            .map(|_| T::id())
            .chain(a.iter().cloned())
            .collect();
        Self::build(a)
    }
}

impl<T: Monoid> From<Vec<T>> for SegTree<T> {
    fn from(mut a: Vec<T>) -> Self {
        if !a.is_empty() {
            a.splice(..0, (0..a.len() - 1).map(|_| T::id()));
        }
        Self::build(a)
    }
}

impl<T: Monoid> std::iter::FromIterator<T> for SegTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<T: fmt::Debug> fmt::Debug for SegTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SegTree").field(&self.deref()).finish()
    }
}
