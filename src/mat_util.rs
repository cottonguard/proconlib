use std::{
    fmt::{self, Debug},
    ops::{self, Index, IndexMut},
};

pub trait Matrix<T> {
    fn row(&self, i: usize) -> &[T];
    fn row_mut(&mut self, i: usize) -> &mut [T];
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn size(&self) -> (usize, usize) {
        (self.n(), self.m())
    }
    fn elem(&self, i: usize, j: usize) -> &T {
        &self.row(i)[j]
    }
    fn elem_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.row_mut(i)[j]
    }
    fn set_elem(&mut self, i: usize, j: usize, x: T) {
        *self.elem_mut(i, j) = x;
    }
    fn add_write<A: Matrix<T> + ?Sized, B: Matrix<T> + ?Sized>(&mut self, a: &A, b: &B)
    where
        T: Element,
    {
        matadd(a, b, self);
    }
    fn add<A: Matrix<T>>(&self, other: &A) -> HeapMat<T>
    where
        T: Element,
    {
        let mut res = HeapMat::zeros(self.n(), self.m());
        res.add_write(self, other);
        res
    }
    fn sub_write<A: Matrix<T> + ?Sized, B: Matrix<T> + ?Sized>(&mut self, a: &A, b: &B)
    where
        T: Element,
    {
        matsub(a, b, self);
    }
    fn sub<A: Matrix<T>>(&self, other: &A) -> HeapMat<T>
    where
        T: Element,
    {
        let mut res = HeapMat::zeros(self.n(), self.m());
        res.sub_write(self, other);
        res
    }
    fn mul_write<A: Matrix<T> + ?Sized, B: Matrix<T> + ?Sized>(&mut self, a: &A, b: &B)
    where
        T: Element,
    {
        matmul(a, b, self);
    }
    fn mul<A: Matrix<T>>(&self, other: &A) -> HeapMat<T>
    where
        T: Element,
    {
        let mut res = HeapMat::zeros(self.n(), other.m());
        res.mul_write(self, other);
        res
    }
    fn scala_mul(&mut self, x: T) -> &mut Self
    where
        T: Element,
    {
        for i in 0..self.n() {
            for j in 0..self.m() {
                let e = self.elem_mut(i, j);
                *e = x * *e;
            }
        }
        self
    }
    fn neg(&mut self) -> &mut Self
    where
        T: Copy + ops::Neg<Output = T>,
    {
        for i in 0..self.n() {
            for j in 0..self.m() {
                let e = self.elem_mut(i, j);
                *e = -*e;
            }
        }
        self
    }
    fn transposed(&self) -> HeapMat<T>
    where
        T: Element,
    {
        let mut res = HeapMat::zeros(self.m(), self.n());
        for i in 0..self.n() {
            for j in 0..self.m() {
                res.set_elem(j, i, *self.elem(i, j));
            }
        }
        res
    }
    fn to_heap_mat(&self) -> HeapMat<T>
    where
        T: Copy,
    {
        HeapMat {
            n: self.n(),
            m: self.m(),
            data: (0..self.n())
                .flat_map(|i| (0..self.m()).map(move |j| *self.elem(i, j)))
                .collect(),
        }
    }
    fn pow(&self, mut k: u64) -> HeapMat<T>
    where
        T: Element,
    {
        assert_eq!(self.n(), self.m());
        if k == 0 {
            return HeapMat::id(self.n());
        }
        let mut x = self.to_heap_mat();
        let mut y = self.to_heap_mat();
        let mut temp = HeapMat::zeros(self.n(), self.n());
        while k > 1 {
            temp.mul_write(&x, &x);
            std::mem::swap(&mut x, &mut temp);
            if k & 1 == 1 {
                temp.mul_write(&x, &y);
                std::mem::swap(&mut y, &mut temp);
            }
            k >>= 1;
        }
        x
    }
}

impl<T> Matrix<T> for [Vec<T>] {
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self[i]
    }
    fn n(&self) -> usize {
        self.len()
    }
    fn m(&self) -> usize {
        self[0].len()
    }
}
impl<T, const M: usize> Matrix<T> for [[T; M]] {
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self[i]
    }
    fn n(&self) -> usize {
        self.len()
    }
    fn m(&self) -> usize {
        M
    }
}
impl<T, const N: usize> Matrix<T> for [Vec<T>; N] {
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self[i]
    }
    fn n(&self) -> usize {
        self.len()
    }
    fn m(&self) -> usize {
        self[0].len()
    }
}
impl<T, const N: usize, const M: usize> Matrix<T> for [[T; M]; N] {
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self[i]
    }
    fn n(&self) -> usize {
        self.len()
    }
    fn m(&self) -> usize {
        M
    }
}

pub fn matadd_with<
    T,
    A: Matrix<T> + ?Sized,
    B: Matrix<T> + ?Sized,
    C: Matrix<T> + ?Sized,
    F: FnMut(&T, &T, &mut T),
>(
    a: &A,
    b: &B,
    c: &mut C,
    mut f: F,
) {
    assert_eq!(a.size(), b.size());
    assert_eq!(a.size(), c.size());
    for i in 0..a.n() {
        for j in 0..b.n() {
            f(a.elem(i, j), b.elem(i, j), c.elem_mut(i, j));
        }
    }
}
pub fn matadd<
    T: Copy + ops::Add<Output = T>,
    A: Matrix<T> + ?Sized,
    B: Matrix<T> + ?Sized,
    C: Matrix<T> + ?Sized,
>(
    a: &A,
    b: &B,
    c: &mut C,
) {
    matadd_with(a, b, c, |a, b, c| *c = *a + *b);
}
pub fn matsub<
    T: Copy + ops::Sub<Output = T>,
    A: Matrix<T> + ?Sized,
    B: Matrix<T> + ?Sized,
    C: Matrix<T> + ?Sized,
>(
    a: &A,
    b: &B,
    c: &mut C,
) {
    matadd_with(a, b, c, |a, b, c| *c = *a - *b);
}
pub fn matmul<
    T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>,
    A: Matrix<T> + ?Sized,
    B: Matrix<T> + ?Sized,
    C: Matrix<T> + ?Sized,
>(
    a: &A,
    b: &B,
    c: &mut C,
) {
    assert_eq!(a.m(), b.n());
    assert_eq!(a.n(), c.n());
    assert_eq!(b.m(), c.m());
    for i in 0..c.n() {
        for j in 0..c.m() {
            for k in 0..a.m() {
                *c.elem_mut(i, j) = *c.elem_mut(i, j) + *a.elem(i, k) * *b.elem(k, j);
            }
        }
    }
}

pub struct HeapMat<T> {
    data: Vec<T>,
    n: usize,
    m: usize,
}

impl<T> HeapMat<T> {
    pub fn zeros(n: usize, m: usize) -> Self
    where
        T: Element,
    {
        Self {
            data: vec![T::zero(); n * m],
            n,
            m,
        }
    }
    pub fn id(n: usize) -> Self
    where
        T: Element,
    {
        let mut res = Self::zeros(n, n);
        for i in 0..n {
            res[i][i] = T::one();
        }
        res
    }
    pub fn row_vector(v: Vec<T>) -> Self {
        Self {
            n: 1,
            m: v.len(),
            data: v,
        }
    }
    pub fn col_vector(v: Vec<T>) -> Self {
        Self {
            n: v.len(),
            m: 1,
            data: v,
        }
    }
    pub fn rows(&self) -> Rows<T> {
        Rows { mat: self, i: 0 }
    }
}
impl<T> Matrix<T> for HeapMat<T> {
    fn row(&self, i: usize) -> &[T] {
        &self.data[i * self.n..(i + 1) * self.n]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.data[i * self.n..(i + 1) * self.n]
    }
    fn n(&self) -> usize {
        self.n
    }
    fn m(&self) -> usize {
        self.m
    }
}
impl<T> Index<usize> for HeapMat<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        self.row(i)
    }
}
impl<T> IndexMut<usize> for HeapMat<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.row_mut(i)
    }
}
impl<T: Debug> Debug for HeapMat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}
pub struct Rows<'a, T> {
    mat: &'a HeapMat<T>,
    i: usize,
}
impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i < self.mat.n {
            self.i += 1;
            Some(&self.mat[i])
        } else {
            None
        }
    }
}

pub trait Element:
    Copy + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! element {
    ($ty:ty) => {
        impl Element for $ty {
            fn zero() -> Self {
                0 as Self
            }
            fn one() -> Self {
                1 as Self
            }
        }
    };
}

macro_rules! each {
    ($macro:ident, $($ty:ty),*) => {$(
        $macro!($ty);
    )*};
}

each!(element, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
