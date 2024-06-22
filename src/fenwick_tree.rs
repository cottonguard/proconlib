#[derive(Clone)]
pub struct FenwickTree<T> {
    a: Vec<T>,
}

impl<T: Monoid> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        Self {
            a: (0..n).map(|_| T::id()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.a.len()
    }

    pub fn is_empty(&self) -> bool {
        self.a.is_empty()
    }

    pub fn sum(&self, i: usize) -> T {
        let mut i = i.min(self.a.len());
        let mut x = T::id();
        while i > 0 {
            x = T::op(&self.a[i - 1], &x);
            i = i & i - 1;
        }
        x
    }

    pub fn sum_all(&self) -> T {
        self.sum(self.len())
    }

    pub fn add(&mut self, i: usize, x: T) {
        assert!(
            i < self.a.len(),
            "out of range (len: {}, index {})",
            self.a.len(),
            i
        );
        let mut i = i + 1;
        while i <= self.a.len() {
            self.a[i - 1] = T::op(&self.a[i - 1], &x);
            i += i & (!i + 1);
        }
    }
}

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}
