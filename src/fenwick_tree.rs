pub struct FenwickTree<T, E, F> {
    a: Vec<T>,
    e: E,
    f: F,
}

impl<T, E: Fn() -> T, F: Fn(&T, &T) -> T> FenwickTree<T, E, F> {
    pub fn new(n: usize, e: E, f: F) -> Self {
        Self {
            a: (0..n).map(|_| e()).collect(),
            e,
            f,
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
        let mut x = (self.e)();
        while i > 0 {
            x = (self.f)(&self.a[i - 1], &x);
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
            self.a[i - 1] = (self.f)(&self.a[i - 1], &x);
            i += i & (!i + 1);
        }
    }
}
