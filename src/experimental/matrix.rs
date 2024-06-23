use std::{
    alloc::{self, Layout},
    fmt::{self, Debug},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops::{Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, Sub},
    ptr::{self, NonNull},
    slice,
};

pub trait Scalar:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! prim_int {
    ($ty:ident) => {
        impl Scalar for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

macro_rules! prim_float {
    ($ty:ident) => {
        impl Scalar for $ty {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        }
    };
}

prim_int!(usize);
prim_int!(u8);
prim_int!(u16);
prim_int!(u32);
prim_int!(u64);
prim_int!(u128);
prim_int!(isize);
prim_int!(i8);
prim_int!(i16);
prim_int!(i32);
prim_int!(i64);
prim_int!(i128);
prim_float!(f32);
prim_float!(f64);

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

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn m(&self) -> usize {
        self.m
    }

    pub fn size(&self) -> (usize, usize) {
        (self.n, self.m)
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

    pub unsafe fn row_unchecked(&self, i: usize) -> &Vector<T> {
        Vector::from_slice(slice::from_raw_parts(
            self.ptr.as_ptr().add(self.m * i),
            self.m,
        ))
    }

    pub fn rows(&self) -> Rows<T> {
        Rows {
            mat: self,
            i: 0,
            j: self.n,
        }
    }
}

impl<T: Scalar> Matrix<T> {
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
        let res = Matrix {
            n: self.n,
            m: self.m,
            ptr: self.ptr.cast(),
            marker: PhantomData,
        };
        mem::forget(self);
        res
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.as_flattened_mut());
            alloc::dealloc(self.as_mut_ptr().cast(), Self::layout(self.n, self.m));
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vector<T>;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.n, "out of range: n={}, i={i}", self.n);
        Vector::from_slice(unsafe {
            slice::from_raw_parts(self.ptr.as_ptr().add(self.m * i), self.m)
        })
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.n, "out of range: n={}, i={i}", self.n);
        Vector::from_mut_slice(unsafe {
            slice::from_raw_parts_mut(self.ptr.as_ptr().add(self.m * i), self.m)
        })
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.m == other.m && self.as_flattened() == other.as_flattened()
    }
}

impl<T: Eq> Eq for Matrix<T> {}

#[inline]
fn assert_same_size<T>(a: &Matrix<T>, b: &Matrix<T>) {
    assert_eq!((a.n, a.m), (b.n, b.m));
}

impl<T: Scalar> Add for &Matrix<T> {
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

impl<T: Scalar> AddAssign<&Self> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Self) {
        assert_same_size(self, rhs);
        for (a, b) in self.as_flattened_mut().iter_mut().zip(rhs.as_flattened()) {
            *a = *a + *b;
        }
    }
}

impl<T: Scalar> Mul for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.m, other.n);
        let mut res = Matrix::zero(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                for k in 0..self.m {
                    res[i][j] = res[i][j] + self[i][k] * other[k][j];
                }
            }
        }
        res
    }
}

impl<T: Clone, const M: usize> From<&[[T; M]]> for Matrix<T> {
    fn from(a: &[[T; M]]) -> Self {
        let mut mat = Self::uninit(a.len(), M);
        unsafe {
            let ptr = mat.as_mut_ptr().cast::<[T; M]>();
            for i in 0..a.len() {
                ptr.add(i).write(a[i].clone());
            }
            mat.assume_init()
        }
    }
}

impl<T, const N: usize, const M: usize> From<[[T; M]; N]> for Matrix<T> {
    fn from(a: [[T; M]; N]) -> Self {
        let mut mat = Self::uninit(N, M);
        unsafe {
            mat.as_mut_ptr()
                .cast::<T>()
                .copy_from_nonoverlapping(a.as_ptr() as *const T, N * M);
            mat.assume_init()
        }
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}

pub struct Rows<'a, T> {
    mat: &'a Matrix<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a Vector<T>;
    fn next(&mut self) -> Option<Self::Item> {
        (self.i < self.j).then(|| {
            let i = self.i;
            self.i += 1;
            Vector::from_slice(unsafe { self.mat.row_unchecked(i) })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.i < self.j).then(|| {
            self.j -= 1;
            Vector::from_slice(unsafe { self.mat.row_unchecked(self.j) })
        })
    }
}

impl<'a, T> ExactSizeIterator for Rows<'a, T> {
    fn len(&self) -> usize {
        self.j - self.i
    }
}

#[repr(transparent)]
pub struct Vector<T>([T]);

impl<T> Vector<T> {
    pub fn from_slice(v: &[T]) -> &Self {
        unsafe { mem::transmute(v) }
    }

    pub fn from_mut_slice(v: &mut [T]) -> &mut Self {
        unsafe { mem::transmute(v) }
    }

    pub fn from_boxed(v: Box<[T]>) -> Box<Self> {
        unsafe { mem::transmute(v) }
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(&self, other: &Self) -> T {
        assert_eq!(self.len(), other.len());
        let mut res = T::ZERO;
        for (a, b) in self.iter().zip(other.iter()) {
            res = res + *a * *b;
        }
        res
    }

    pub fn norm(&self) -> T {
        self.dot(self)
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(&mut self.0) }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Scalar> Mul<&Vector<T>> for &Matrix<T> {
    type Output = Box<Vector<T>>;
    fn mul(self, v: &Vector<T>) -> Self::Output {
        assert_eq!(self.m, v.len());
        let res: Box<[T]> = self.rows().map(|u| u.dot(v)).collect();
        Vector::from_boxed(res)
    }
}
