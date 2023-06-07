use std::{
    fmt,
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub struct ArrayVec<T, const CAP: usize> {
    buf: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub fn new() -> Self {
        Self {
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn is_full(&self) -> bool {
        self.len == CAP
    }

    pub fn as_slice(&self) -> &[T] {
        &*self
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut *self
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub fn truncate(&mut self, len: usize) {
        let orig_len = self.len;
        if len >= orig_len {
            return;
        }
        self.len = len;
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                self.as_mut_ptr().add(len),
                orig_len - len,
            ));
        }
    }

    pub fn resize(&mut self, len: usize, value: T)
    where
        T: Clone,
    {
        if len <= self.len {
            self.truncate(len);
        } else if len > CAP {
            capacity_overflow();
        } else {
            unsafe {
                while self.len + 1 < len {
                    self.push_unchecked(value.clone());
                }
                self.push_unchecked(value);
            }
        }
    }

    pub fn push(&mut self, value: T) {
        if self.try_push(value).is_err() {
            capacity_overflow();
        }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            Err(value)
        } else {
            unsafe { self.push_unchecked(value) }
            Ok(())
        }
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        self.as_mut_ptr().add(self.len).write(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            let popped = unsafe { self.as_mut_ptr().add(self.len).read() };
            Some(popped)
        }
    }

    pub fn insert(&mut self, i: usize, value: T) {
        if self.try_insert(i, value).is_err() {
            capacity_overflow();
        }
    }

    pub fn try_insert(&mut self, i: usize, value: T) -> Result<(), T> {
        if i > self.len || self.is_full() {
            Err(value)
        } else {
            unsafe { self.insert_unchecked(i, value) }
            Ok(())
        }
    }

    pub unsafe fn insert_unchecked(&mut self, i: usize, value: T) {
        let ptr = self.as_mut_ptr().add(i);
        ptr.add(1).copy_from(ptr, self.len - i);
        ptr.write(value);
        self.len += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        assert!(
            i < self.len,
            "out of range (len: {}, index: {})",
            self.len,
            i
        );
        unsafe {
            let ptr = self.as_mut_ptr().add(i);
            let removed = ptr.read();
            self.len -= 1;
            ptr.copy_from(ptr.add(1), self.len - i);
            removed
        }
    }

    pub fn retain(&mut self, mut f: impl FnMut(&T) -> bool) {
        struct Guard<'a, T, const CAP: usize> {
            vec: &'a mut ArrayVec<T, CAP>,
            orig_len: usize,
            i: usize,
        }

        impl<'a, T, const CAP: usize> Drop for Guard<'a, T, CAP> {
            fn drop(&mut self) {
                debug_assert!(self.vec.len <= self.i);
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.vec.as_mut_ptr().add(self.i),
                        self.orig_len - self.i,
                    ));
                }
            }
        }

        let ptr = self.as_mut_ptr();
        let orig_len = self.len;
        self.len = 0;
        let mut guard = Guard {
            orig_len,
            vec: self,
            i: 0,
        };
        let mut block = 0;

        unsafe {
            while guard.i < orig_len {
                if f(&*ptr.add(guard.i)) {
                    block += 1;
                } else {
                    if block > 0 {
                        if guard.i != block {
                            ptr.add(guard.vec.len)
                                .copy_from(ptr.add(guard.i - block), block);
                        }
                        guard.vec.len += block;
                        block = 0;
                    }
                    ptr.add(guard.i).read();
                }
                guard.i += 1;
            }
            if block > 0 {
                if orig_len != block {
                    ptr.add(guard.vec.len)
                        .copy_from(ptr.add(guard.i - block), block);
                }
                guard.vec.len += block;
            }
        }

        mem::forget(guard);
    }
}

#[cold]
fn capacity_overflow() {
    panic!("capacity overflow");
}

impl<T, const CAP: usize> Drop for ArrayVec<T, CAP> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const CAP: usize> Deref for ArrayVec<T, CAP> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.buf.as_ptr() as _, self.len) }
    }
}

impl<T, const CAP: usize> DerefMut for ArrayVec<T, CAP> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.buf.as_mut_ptr() as _, self.len) }
    }
}

impl<T, const CAP: usize> AsRef<[T]> for ArrayVec<T, CAP> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const CAP: usize> AsMut<[T]> for ArrayVec<T, CAP> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const CAP: usize> Default for ArrayVec<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq, const CAP: usize> PartialEq for ArrayVec<T, CAP> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const CAP: usize> Eq for ArrayVec<T, CAP> {}

impl<T: PartialOrd, const CAP: usize> PartialOrd for ArrayVec<T, CAP> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord, const CAP: usize> Ord for ArrayVec<T, CAP> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash, const CAP: usize> std::hash::Hash for ArrayVec<T, CAP> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: Clone, const CAP: usize> Clone for ArrayVec<T, CAP> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: fmt::Debug, const CAP: usize> fmt::Debug for ArrayVec<T, CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, const CAP: usize> FromIterator<T> for ArrayVec<T, CAP> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = Self::new();
        arr.extend(iter);
        arr
    }
}

impl<T, const CAP: usize> Extend<T> for ArrayVec<T, CAP> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.push(value);
        }
    }
}

impl<T, const CAP: usize, const N: usize> TryInto<[T; N]> for ArrayVec<T, CAP> {
    type Error = Self;
    fn try_into(self) -> Result<[T; N], Self> {
        if N == self.len {
            let vec = mem::ManuallyDrop::new(self);
            let mut arr = MaybeUninit::uninit();
            unsafe {
                (arr.as_mut_ptr() as *mut T).copy_from_nonoverlapping(vec.as_ptr(), N);
                Ok(arr.assume_init())
            }
        } else {
            Err(self)
        }
    }
}

impl<T, const CAP: usize> From<Vec<T>> for ArrayVec<T, CAP> {
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

impl<T: Clone, const CAP: usize> From<&[T]> for ArrayVec<T, CAP> {
    fn from(slice: &[T]) -> Self {
        slice.iter().cloned().collect()
    }
}

impl<T, const CAP: usize> IntoIterator for ArrayVec<T, CAP> {
    type Item = T;

    type IntoIter = IntoIter<T, CAP>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: ManuallyDrop::new(self),
            i: 0,
        }
    }
}

pub struct IntoIter<T, const CAP: usize> {
    vec: ManuallyDrop<ArrayVec<T, CAP>>,
    i: usize,
}

impl<T, const CAP: usize> IntoIter<T, CAP> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.vec.as_ptr().add(self.i), self.vec.len - self.i) }
    }
}

impl<T, const CAP: usize> IntoIter<T, CAP> {
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.vec.as_mut_ptr().add(self.i), self.vec.len - self.i)
        }
    }
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.i < self.vec.len {
            let res = unsafe { self.vec.as_ptr().add(self.i).read() };
            self.i += 1;
            Some(res)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T, const CAP: usize> DoubleEndedIterator for IntoIter<T, CAP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i < self.vec.len {
            self.vec.pop()
        } else {
            None
        }
    }
}

impl<T, const CAP: usize> ExactSizeIterator for IntoIter<T, CAP> {
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<T, const CAP: usize> Drop for IntoIter<T, CAP> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.as_mut_slice()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a: ArrayVec<i32, 3> = ArrayVec::new();
        assert_eq!(*a, []);
        assert_eq!(a.pop(), None);
        assert_eq!(*a, []);
        a.push(1);
        assert_eq!(*a, [1]);
        assert_eq!(a[0], 1);
        a.push(2);
        assert_eq!(*a, [1, 2]);
        a.push(3);
        assert_eq!(*a, [1, 2, 3]);
        assert_eq!(a.try_push(4), Err(4));
        assert_eq!(a.pop(), Some(3));
        assert_eq!(*a, [1, 2]);
        assert_eq!(*a.clone(), [1, 2]);
        a.truncate(1);
        assert_eq!(*a, [1]);
        a.resize(3, 10);
        assert_eq!(*a, [1, 10, 10]);
    }

    #[test]
    fn drop() {
        use std::cell::RefCell;
        thread_local! {
            static DROPPED: RefCell<Vec<i32>> = RefCell::new(vec![]);
        }
        struct S(i32);
        impl Drop for S {
            fn drop(&mut self) {
                DROPPED.with(|d| d.borrow_mut().push(self.0));
            }
        }

        {
            let _a: ArrayVec<_, 10> = vec![S(1), S(2), S(3)].into();
            DROPPED.with(|d| assert_eq!(*d.borrow(), []));
        }
        DROPPED.with(|d| assert_eq!(*d.borrow(), [1, 2, 3]));
    }

    #[test]
    fn retain() {
        let mut a: ArrayVec<i32, 20> = (0..20).collect();
        a.retain(|x| x % 3 == 0);
        assert_eq!(*a, [0, 3, 6, 9, 12, 15, 18]);

        let mut a: ArrayVec<i32, 20> = (0..20).collect();
        a.retain(|x| x % 10 <= 3);
        assert_eq!(*a, [0, 1, 2, 3, 10, 11, 12, 13]);

        let mut a: ArrayVec<i32, 20> = (0..20).collect();
        a.retain(|x| 4 <= x % 10 && x % 10 <= 6);
        assert_eq!(*a, [4, 5, 6, 14, 15, 16]);

        let mut a: ArrayVec<i32, 20> = (0..20).collect();
        a.retain(|x| *x == 19);
        assert_eq!(*a, [19]);

        let mut a: ArrayVec<String, 5> = ["abc", "x", "ABC", "Y", "abc"]
            .into_iter()
            .map(|s| s.into())
            .collect();
        a.retain(|x| x.len() == 3);
        assert_eq!(*a, ["abc", "ABC", "abc"]);
    }

    #[test]
    fn into_iter() {
        let a: ArrayVec<_, 10> = ('a'..='f').map(|c| c.to_string()).collect();
        let mut it = a.into_iter();
        assert_eq!(it.next(), Some('a'.to_string()));
        assert_eq!(it.next(), Some('b'.to_string()));
        assert_eq!(it.next_back(), Some('f'.to_string()));
        assert_eq!(it.len(), 3);
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.as_slice(), ["c", "d", "e"]);
        assert_eq!(it.as_mut_slice(), ["c", "d", "e"]);
        /*
        assert_eq!(it.next(), Some('c'.to_string()));
        assert_eq!(it.next(), Some('d'.to_string()));
        assert_eq!(it.next(), Some('e'.to_string()));
        assert_eq!(it.next(), None);
        assert_eq!(it.next_back(), None);
        */
    }
}
