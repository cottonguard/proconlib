use std::{marker::PhantomData, ops::Add};

pub struct MatrixBuf<T> {
    buf: Box<[T]>,
    rows: usize,
    cols: usize,
}

impl<'a, T> IntoMatrixRef<'a> for &'a MatrixBuf<T> {
    type Elem = T;
    fn into_matrix_ref(self) -> MatrixRef<'a, T> {
        unsafe { self.buf.matrix_unchecked(self.rows, self.cols) }
    }
}

pub struct MatrixRef<'a, T> {
    ptr: *const T,
    rows: usize,
    cols: usize,
    marker: PhantomData<&'a T>,
}

impl<'a, T: IntoMatrixRef<'a>> Add for T {
    type Output = MatrixBuf<T::Elem>;
}

pub struct MatrixMut<'a, T> {
    ptr: *mut T,
    rows: usize,
    cols: usize,
    marker: PhantomData<&'a mut T>,
}

pub trait IntoMatrixRef<'a> {
    type Elem;
    fn into_matrix_ref(self) -> MatrixRef<'a, Self::Elem>;
}

impl<'a, T> IntoMatrixRef<'a> for MatrixMut<'a, T> {
    type Elem = T;
    fn into_matrix_ref(self) -> MatrixRef<'a, T> {
        MatrixRef {
            ptr: self.ptr,
            rows: self.rows,
            cols: self.cols,
            marker: PhantomData,
        }
    }
}

pub trait SliceToMatrix<T> {
    fn matrix(&self, rows: usize, cols: usize) -> MatrixRef<T>;
    unsafe fn matrix_unchecked(&self, rows: usize, cols: usize) -> MatrixRef<T>;
    fn matrix_mut(&mut self, rows: usize, cols: usize) -> MatrixMut<T>;
    unsafe fn matrix_mut_unchecked(&mut self, rows: usize, cols: usize) -> MatrixMut<T>;
}

impl<T> SliceToMatrix<T> for [T] {
    fn matrix(&self, rows: usize, cols: usize) -> MatrixRef<T> {
        assert!(rows * cols <= self.len());
        unsafe { self.matrix_unchecked(rows, cols) }
    }

    unsafe fn matrix_unchecked(&self, rows: usize, cols: usize) -> MatrixRef<T> {
        MatrixRef {
            ptr: self.as_ptr(),
            rows,
            cols,
            marker: PhantomData,
        }
    }

    fn matrix_mut(&mut self, rows: usize, cols: usize) -> MatrixMut<T> {
        assert!(rows * cols <= self.len());
        unsafe { self.matrix_mut_unchecked(rows, cols) }
    }

    unsafe fn matrix_mut_unchecked(&mut self, rows: usize, cols: usize) -> MatrixMut<T> {
        MatrixMut {
            ptr: self.as_mut_ptr(),
            rows,
            cols,
            marker: PhantomData,
        }
    }
}
