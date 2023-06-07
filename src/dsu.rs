#[derive(Clone)]
pub struct Dsu(Vec<isize>);

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self(vec![-1; n])
    }
    pub fn root(&self, mut u: usize) -> usize {
        while self.0[u] >= 0 {
            u = self.0[u] as usize;
        }
        u
    }
    pub fn is_root(&self, u: usize) -> bool {
        self.0[u] < 0
    }
    pub fn unite(&mut self, u: usize, v: usize) -> UniteResult {
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return UniteResult {
                root: ru,
                united_root: None,
                size: -self.0[ru] as _,
            };
        }
        let (r, c) = if -self.0[ru] >= -self.0[rv] {
            (ru, rv)
        } else {
            (rv, ru)
        };
        self.0[r] += self.0[c];
        self.0[c] = r as isize;
        UniteResult {
            root: r,
            united_root: Some(c),
            size: -self.0[r] as _,
        }
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
    pub fn size(&self, u: usize) -> usize {
        -self.0[self.root(u)] as usize
    }
    pub fn reset(&mut self) {
        todo!();
        // self.0.fill(-1);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UniteResult {
    pub root: usize,
    pub united_root: Option<usize>,
    pub size: usize,
}

impl UniteResult {
    pub fn is_united(&self) -> bool {
        self.united_root.is_some()
    }
}

use std::mem::ManuallyDrop;
pub struct DsuMerge<T, F> {
    inner: Dsu,
    data: Vec<ManuallyDrop<T>>,
    merge: F,
}

impl<T, F: FnMut(&mut T, T)> DsuMerge<T, F> {
    pub fn new(n: usize, init: T, merge: F) -> Self
    where
        T: Clone,
    {
        Self::from_iterator((0..n).map(|_| init.clone()), merge)
    }
    pub fn from_fn(n: usize, init: impl FnMut(usize) -> T, merge: F) -> Self {
        Self::from_iterator((0..n).map(init), merge)
    }
    pub fn from_iterator(iter: impl IntoIterator<Item = T>, merge: F) -> Self {
        let data: Vec<_> = iter.into_iter().map(|x| ManuallyDrop::new(x)).collect();
        Self {
            inner: Dsu::new(data.len()),
            data,
            merge,
        }
    }
    pub fn root(&self, u: usize) -> usize {
        self.inner.root(u)
    }
    pub fn is_root(&self, u: usize) -> bool {
        self.inner.is_root(u)
    }
    pub fn unite(&mut self, u: usize, v: usize) -> (UniteResult, &mut T) {
        let res = self.inner.unite(u, v);
        if let Some(c) = res.united_root {
            let taken = unsafe { ManuallyDrop::take(&mut self.data[c]) };
            (self.merge)(&mut self.data[res.root], taken);
        }
        (res, &mut self.data[res.root])
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.inner.is_same(u, v)
    }
    pub fn size(&self, u: usize) -> usize {
        self.inner.size(u)
    }
    pub fn data(&self, u: usize) -> &T {
        &self.data[self.root(u)]
    }
    pub fn data_mut(&mut self, u: usize) -> &mut T {
        &mut self.data[self.inner.root(u)]
    }
}

impl<T, F> Drop for DsuMerge<T, F> {
    fn drop(&mut self) {
        for (u, data) in self.data.iter_mut().enumerate() {
            if self.inner.is_root(u) {
                unsafe {
                    ManuallyDrop::drop(data);
                }
            }
        }
    }
}
