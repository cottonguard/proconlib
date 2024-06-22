use std::{mem, ops::Deref};

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

#[derive(Clone, Debug)]
pub struct SegTree<T>(Vec<T>);

impl<T: Monoid> SegTree<T> {
    pub fn new(len: usize) -> Self {
        Self((0..2 * len).map(|_| T::id()).collect())
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
                sum_l = sum_l.op(&self.0[l]);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                sum_r = self.0[r].op(&sum_r);
                r >>= r.trailing_zeros();
            }
            if l == r {
                break;
            }
        }
        sum_l.op(&sum_r)
    }

    pub fn sum_all(&self) -> T {
        self.0[1].op(&T::id())
    }

    pub fn set(&mut self, i: usize, value: T) -> T {
        self.update(i, |_| value)
    }

    pub fn add_left(&mut self, i: usize, value: &T) -> T {
        self.update(i, |orig| value.op(orig))
    }

    pub fn add_right(&mut self, i: usize, value: &T) -> T {
        self.update(i, |orig| orig.op(value))
    }

    pub fn update<F: FnOnce(&T) -> T>(&mut self, i: usize, f: F) -> T {
        let mut i = i + self.0.len() / 2;
        let value = f(&self.0[i]);
        let orig = mem::replace(&mut self.0[i], value);
        while i > 1 {
            i /= 2;
            self.0[i] = self.0[2 * i].op(&self.0[2 * i + 1]);
        }
        orig
    }

    pub fn clear(&mut self) {
        for a in &mut self.0 {
            *a = T::id();
        }
    }
}

impl<T> Deref for SegTree<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0[self.0.len() / 2..]
    }
}

impl<T: Monoid> From<Vec<T>> for SegTree<T> {
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
        Self(a)
    }
}

impl<T: Monoid> FromIterator<T> for SegTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut a = Vec::with_capacity(2 * iter.size_hint().0);
        a.extend(iter);
        Self::from(a)
    }
}
