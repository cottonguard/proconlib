use std::{
    alloc::{self, Layout},
    cell::Cell,
    marker::PhantomData,
    mem, ptr, slice,
};

pub struct ArenaSlices<T> {
    head: Cell<*mut Header<T>>,
    marker: PhantomData<T>,
}

struct Chunk<T> {
    header: Header<T>,
    buf: [T],
}

struct Header<T> {
    next: *mut Header<T>,
    cap: usize,
    len: usize,
}

impl<T> Header<T> {
    #[inline]
    fn remaining(&self) -> usize {
        self.cap - self.len
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.remaining() == 0
    }

    fn body(&mut self) -> *mut T {
        unsafe { (self as *mut Self as *mut u8).add(Self::size_with_pad()) as *mut T }
    }

    fn vacant(&mut self) -> *mut [T] {
        unsafe { ptr::slice_from_raw_parts_mut(self.body().add(self.len), self.remaining()) }
    }

    fn size_with_pad() -> usize {
        let align = mem::align_of::<Self>().max(mem::align_of::<T>());
        let mask = align - 1;
        (mem::size_of::<Self>() + mask) & !mask
    }

    fn layout(&self) -> Layout {
        ArenaSlices::<T>::chunk_layout(self.cap)
    }
}

const MIN_CAP: usize = 1024;

impl<T> ArenaSlices<T> {
    pub fn new() -> Self {
        Self {
            head: Cell::new(ptr::null_mut()),
            marker: PhantomData,
        }
    }

    #[inline]
    fn remaining(&self) -> usize {
        if self.head.get().is_null() {
            0
        } else {
            unsafe { (*self.head.get()).remaining() }
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        if self.head.get().is_null() {
            true
        } else {
            unsafe { (*self.head.get()).is_full() }
        }
    }

    #[inline]
    fn header_size_with_pad() -> usize {
        let align = mem::align_of::<Header<T>>().max(mem::align_of::<T>());
        let mask = align - 1;
        (mem::size_of::<Header<T>>() + mask) & !mask
    }

    #[inline]
    fn chunk_layout(cap: usize) -> Layout {
        let align = mem::align_of::<Header<T>>().max(mem::align_of::<T>());
        let size = Self::header_size_with_pad() + cap * mem::size_of::<T>();
        Layout::from_size_align(size, align).unwrap()
    }

    fn alloc_chunk(&self, len: usize) {
        let cap = len.max(MIN_CAP);
        let layout = Self::chunk_layout(cap);

        unsafe {
            let ptr = alloc::alloc(layout) as *mut Header<T>;
            *ptr = Header {
                next: self.head.get(),
                cap,
                len: 0,
            };
            self.head.set(ptr);
        }
    }

    #[inline]
    fn ensure(&self, len: usize) {
        if len > self.remaining() {
            self.alloc_chunk(len);
        }
    }

    #[inline]
    fn buf_ptr(&self) -> *mut T {
        unsafe {
            let begin = (self.head.get() as *mut u8).add(Self::header_size_with_pad()) as *mut T;
            begin.add((*self.head.get()).len)
        }
    }

    pub fn alloc(&self, len: usize) -> *mut [T] {
        self.ensure(len);
        unsafe {
            let ptr = self.buf_ptr();
            (*self.head.get()).len += len;
            ptr::slice_from_raw_parts_mut(ptr, len)
        }
    }

    pub fn alloc_one(&self, val: T) -> &mut T {
        let ptr = self.alloc(1) as *mut T;
        unsafe {
            ptr.write(val);
            &mut *ptr
        }
    }

    pub fn alloc_from_slice(&self, s: &[T]) -> &mut [T]
    where
        T: Clone,
    {
        let ptr = self.alloc(s.len());
        unsafe {
            for (i, val) in s.iter().enumerate() {
                (ptr as *mut T).add(i).write(val.clone());
            }
            &mut *ptr
        }
    }

    pub fn alloc_from_iter<I: IntoIterator<Item = T>>(&self, iter: I) -> &mut [T] {
        let iter = iter.into_iter();
        self.ensure(iter.size_hint().0);
        let mut ptr = self.buf_ptr();
        let mut i = 0;
        unsafe {
            for val in iter {
                if i >= self.remaining() {
                    self.alloc_chunk(2 * (i + 1));
                    self.buf_ptr().copy_from(ptr, i);
                    ptr = self.buf_ptr();
                }
                ptr.add(i).write(val);
                i += 1;
            }
            &mut *self.alloc(i)
        }
    }
}

impl<T> Drop for ArenaSlices<T> {
    fn drop(&mut self) {
        let mut ptr = self.head.get();
        while !ptr.is_null() {
            unsafe {
                let Header { next, len, cap } = *ptr;
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    ptr.add(Self::header_size_with_pad()),
                    len,
                ));
                let next = (*ptr).next;
                alloc::dealloc(ptr as _, Self::chunk_layout((*ptr).cap));
                ptr = next;
            }
        }
    }
}
