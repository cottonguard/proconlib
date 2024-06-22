use std::{
    alloc::{self, dealloc, Layout},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Index, IndexMut},
    ptr::{self, NonNull},
    slice,
};

pub trait Element: Copy + Add<Output = Self> {
    const ZERO: Self;
    const ONE: Self;
}

pub struct Matrix<T> {
    n: usize,
    m: usize,
    ptr: NonNull<T>,
    marker: PhantomData<T>,
}

impl<T> Matrix<T> {
    pub fn uninit(n: usize, m: usize) -> Matrix<MaybeUninit<T>> {
        let ptr = unsafe { NonNull::new_unchecked(alloc::alloc(Self::layout(n, m)).cast()) };
        Matrix {
            n,
            m,
            ptr,
            marker: PhantomData,
        }
    }

    fn layout(n: usize, m: usize) -> Layout {
        Layout::array::<T>(n * m).unwrap()
    }

    pub fn as_flattened(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.n * self.m) }
    }

    pub fn as_flattened_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.n * self.m) }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: Element> Matrix<T> {
    pub fn zero(n: usize, m: usize) -> Self {
        Self::filled(n, m, T::ZERO)
    }

    pub fn identity(n: usize) -> Self {
        let mut a = Self::zero(n, n);
        for i in 0..n {
            a[i][i] = T::ONE;
        }
        a
    }

    pub fn filled(n: usize, m: usize, value: T) -> Self {
        let mut a = Self::uninit(n, m);
        for e in a.as_flattened_mut() {
            e.write(value);
        }
        unsafe { a.assume_init() }
    }
}

impl<T> Matrix<MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> Matrix<T> {
        Matrix {
            n: self.n,
            m: self.m,
            ptr: self.ptr.cast(),
            marker: PhantomData,
        }
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.as_flattened_mut());
            dealloc(self.as_mut_ptr().cast(), Self::layout(self.n, self.m));
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.n, "out of range: n={}, i={i}", self.n);
        unsafe { slice::from_raw_parts(self.ptr.as_ptr().add(self.m * i), self.m) }
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.n, "out of range: n={}, i={i}", self.n);
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr().add(self.m * i), self.m) }
    }
}

#[inline]
fn assert_same_size<T>(a: &Matrix<T>, b: &Matrix<T>) {
    assert_eq!((a.n, a.m), (b.n, b.m));
}

impl<T: Element> Add for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, other: Self) -> Self::Output {
        assert_same_size(self, other);
        let mut res = Matrix::uninit(self.n, self.m);
        for i in 0..self.n * self.m {
            res.as_flattened_mut()[i].write(self.as_flattened()[i] + other.as_flattened()[i]);
        }
        unsafe { res.assume_init() }
    }
}

impl<T: Element> AddAssign<&Self> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Self) {
        assert_same_size(self, rhs);
        for (a, b) in self.as_flattened_mut().iter_mut().zip(rhs.as_flattened()) {
            *a = *a + *b;
        }
    }
}

impl<T, const M: usize> From<&[[T; M]]> for Matrix<T> {
    fn from(a: &[[T; M]]) -> Self {
        let mut mat = Self::uninit(a.len(), M);
        unsafe {
            mat.as_mut_ptr()
                .cast::<T>()
                .copy_from_nonoverlapping(a.as_ptr() as *const T, a.len() * M);
            mat.assume_init()
        }
    }
}

impl<T, const N: usize, const M: usize> From<[[T; M]; N]> for Matrix<T> {
    fn from(a: [[T; M]; N]) -> Self {
        a[..].into()
    }
}
