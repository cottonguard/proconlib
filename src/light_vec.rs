use std::{
    fmt::{self, Debug},
    hash::Hash,
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr, slice, vec,
};

#[macro_export]
macro_rules! light_vec {
    ($($elem:expr),* $(,)?) => {{
        let mut res = LightVec::new();
        let count = light_vec!(@count $($elem),*);
        if count <= res.inline_size() {
            res = [$($elem),*].into();
        } else {
            unsafe {
                res = LightVec::heap_from_vec(vec![$($elem),*]);
            }
        }
        res
    }};
    ($elem:expr; $count:expr) => {{
        LightVec::from_elem($elem, $count)
    }};
    (@count) => {
        0
    };
    (@count $x:expr $(,$rest:expr)*) => {
        1 + light_vec!(@count $($rest),*)
    };
}

pub struct LightVec<T, const N: usize> {
    cap: usize,
    data: Data<T, N>,
}

union Data<T, const N: usize> {
    inline: ManuallyDrop<[MaybeUninit<T>; N]>,
    heap: (*mut T, usize),
}

impl<T, const N: usize> LightVec<T, N> {
    pub const fn new() -> Self {
        Self {
            cap: 0,
            data: Data {
                inline: ManuallyDrop::new(unsafe { MaybeUninit::uninit().assume_init() }),
            },
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        if cap <= N {
            Self::new()
        } else {
            unsafe { Self::heap_from_vec(Vec::with_capacity(cap)) }
        }
    }
    #[doc(hidden)]
    pub const fn inline_size(&self) -> usize {
        N
    }
    pub fn capacity(&self) -> usize {
        self.cap.max(N)
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        if self.cap <= N {
            unsafe { (*self.data.inline).as_ptr() as _ }
        } else {
            unsafe { self.with_heap(|vec| vec.as_ptr()) }
        }
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        if self.cap <= N {
            unsafe { (*self.data.inline).as_mut_ptr() as _ }
        } else {
            unsafe { self.with_heap_mut(|vec| vec.as_mut_ptr()) }
        }
    }
    /*
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        if self.cap <= N {
            unsafe { &mut (*self.data.inline)[self.cap..] }
        } else {
            unsafe { self.with_heap_mut(|vec| vec.spare_capacity_mut()) }
        }
    }
     */
    pub fn reserve(&mut self, additional: usize) {
        if self.cap <= N {
            self.inline_to_heap_with_capacity(N + additional)
        } else {
            unsafe { self.with_heap_mut(|vec| vec.reserve(additional)) }
        }
    }
    pub fn clear(&mut self) {
        if self.cap <= N {
            unsafe {
                ptr::drop_in_place((&mut (*self.data.inline)[..self.cap]) as *mut _ as *mut [T]);
            }
            self.cap = 0;
        } else {
            unsafe { self.with_heap_mut(|vec| vec.clear()) }
        }
    }
    pub fn truncate(&mut self, len: usize) {
        if self.cap <= N {
            if len < self.cap {
                let orig_len = self.cap;
                self.cap = len;
                unsafe {
                    ptr::drop_in_place(
                        &mut (*self.data.inline)[len..orig_len] as *mut _ as *mut [T],
                    );
                }
            }
        } else {
            unsafe { self.with_heap_mut(|vec| vec.truncate(len)) }
        }
    }
    pub fn push(&mut self, value: T) {
        if self.cap < N {
            unsafe {
                (*self.data.inline)[self.cap].write(value);
                self.cap += 1;
            }
        } else {
            if self.cap == N {
                self.inline_to_heap();
            }
            unsafe {
                self.with_heap_mut(|vec| vec.push(value));
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.cap == 0 {
            None
        } else if self.cap <= N {
            unsafe {
                self.cap -= 1;
                Some((*self.data.inline)[self.cap].assume_init_read())
            }
        } else {
            unsafe { self.with_heap_mut(|vec| vec.pop()) }
        }
    }
    pub fn insert(&mut self, i: usize, value: T) {
        if self.cap < N {
            assert!(i < self.cap, "out of range");
            unsafe {
                let inline = &mut *self.data.inline;
                let start = inline.as_mut_ptr().add(i);
                ptr::copy(start, start.add(1), self.cap - i);
                inline[i].write(value);
            }
            self.cap += 1;
        } else {
            if self.cap == N {
                self.inline_to_heap();
            }
            unsafe {
                self.with_heap_mut(|vec| vec.insert(i, value));
            }
        }
    }
    pub fn remove(&mut self, i: usize) -> T {
        if self.cap <= N {
            assert!(i < self.cap, "out of range");
            let res = unsafe {
                let inline = &mut *self.data.inline;
                let res = inline[i].assume_init_read();
                let start = inline.as_mut_ptr().add(i);
                ptr::copy(start.add(1), start, self.cap - i);
                res
            };
            self.cap -= 1;
            res
        } else {
            unsafe { self.with_heap_mut(|vec| vec.remove(i)) }
        }
    }
    pub fn resize(&mut self, len: usize, value: T)
    where
        T: Clone,
    {
        if len <= self.len() {
            if len < self.len() {
                self.truncate(len);
            }
            return;
        }
        if self.cap <= N {
            if len <= N {
                for _ in 0..len - self.cap - 1 {
                    self.push(value.clone());
                }
                self.push(value);
                return;
            }
            self.inline_to_heap_with_capacity(len);
        }
        unsafe { self.with_heap_mut(|vec| vec.resize(len, value)) }
    }
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        if self.cap <= N {
            let len = self.cap;
            let mut g = Guard {
                vec: self,
                dst: 0,
                cur: 0,
            };
            let ptr = g.vec.as_mut_ptr();
            while g.cur < len {
                unsafe {
                    let src = ptr.add(g.cur);
                    if f(&*src) {
                        ptr.add(g.dst).copy_from(src, 1);
                        g.dst += 1;
                    } else {
                        ptr::drop_in_place(src);
                    }
                }
                g.cur += 1;
            }
            g.vec.cap = g.dst;
            mem::forget(g);
        } else {
            unsafe { self.with_heap_mut(|vec| vec.retain(f)) }
        }
    }
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.dedup_by(|x, y| *x == *y)
    }
    pub fn dedup_by<F: FnMut(&mut T, &mut T) -> bool>(&mut self, mut f: F) {
        if self.cap <= N {
            if self.cap <= 1 {
                return;
            }
            let len = self.cap;
            let mut g = Guard {
                vec: self,
                dst: 1,
                cur: 1,
            };
            let ptr = g.vec.as_mut_ptr();
            while g.cur < len {
                unsafe {
                    if f(&mut *ptr.add(g.cur), &mut *ptr.add(g.dst - 1)) {
                        ptr::drop_in_place(ptr.add(g.cur));
                    } else {
                        ptr.add(g.dst).copy_from(ptr.add(g.cur), 1);
                        g.dst += 1;
                    }
                }
                g.cur += 1;
            }
            g.vec.cap = g.dst;
            mem::forget(g);
        } else {
            unsafe { self.with_heap_mut(|vec| vec.dedup_by(f)) }
        }
    }
    pub fn dedup_by_key<K: PartialEq, F: FnMut(&mut T) -> K>(&mut self, mut key: F) {
        self.dedup_by(|x, y| key(x) == key(y))
    }
    #[inline]
    unsafe fn with_heap<U>(&self, f: impl FnOnce(&Vec<T>) -> U) -> U {
        let (ptr, len) = self.data.heap;
        let vec = ManuallyDrop::new(Vec::from_raw_parts(ptr, len, self.cap));
        f(&vec)
    }
    #[inline]
    unsafe fn with_heap_mut<U>(&mut self, f: impl FnOnce(&mut Vec<T>) -> U) -> U {
        let (ptr, len) = self.data.heap;
        let mut vec = ManuallyDrop::new(Vec::from_raw_parts(ptr, len, self.cap));
        let res = f(&mut vec);
        let (ptr, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
        self.cap = cap;
        self.data.heap = (ptr, len);
        res
    }
    unsafe fn into_heap(self) -> Vec<T> {
        let (ptr, len) = self.data.heap;
        let vec = Vec::from_raw_parts(ptr, len, self.cap);
        mem::forget(self);
        vec
    }
    fn inline_to_heap(&mut self) {
        self.inline_to_heap_with_capacity(N + 1)
    }
    fn inline_to_heap_with_capacity(&mut self, cap: usize) {
        let mut vec = Vec::<T>::with_capacity(cap);
        unsafe {
            vec.as_mut_ptr()
                .copy_from_nonoverlapping(self.data.inline.as_ptr() as *const T, self.cap);
            vec.set_len(self.cap);
            // let (ptr, len, cap) = vec.into_raw_parts();
            let (ptr, len, cap) = (vec.as_mut_ptr(), vec.len(), vec.capacity());
            std::mem::forget(vec);
            self.cap = cap;
            self.data.heap = (ptr, len);
        }
    }
    #[doc(hidden)]
    pub unsafe fn heap_from_vec(vec: Vec<T>) -> Self {
        let mut vec = ManuallyDrop::new(vec);
        Self {
            cap: vec.capacity(),
            data: Data {
                heap: (vec.as_mut_ptr(), vec.len()),
            },
        }
    }
    #[doc(hidden)]
    pub fn from_elem(elem: T, n: usize) -> Self
    where
        T: Clone,
    {
        if n <= N {
            let mut res = Self::new();
            let mut i = 0;
            unsafe {
                while i + 1 < n {
                    res.as_mut_ptr().add(i).write(elem.clone());
                    i += 1;
                }
                res.as_mut_ptr().add(i).write(elem);
            }
            res.cap = n;
            res
        } else {
            unsafe { Self::heap_from_vec(vec![elem; n]) }
        }
    }
}

struct Guard<'a, T, const N: usize> {
    vec: &'a mut LightVec<T, N>,
    dst: usize,
    cur: usize,
}
impl<'a, T, const N: usize> Drop for Guard<'a, T, N> {
    fn drop(&mut self) {
        let ptr = self.vec.as_mut_ptr();
        let left = self.vec.cap - self.cur;
        unsafe {
            ptr.add(self.dst).copy_from(ptr.add(self.cur), left);
        }
        self.vec.cap = self.dst + left;
    }
}

unsafe impl<T: Sync, const N: usize> Sync for LightVec<T, N> {}
unsafe impl<T: Send, const N: usize> Send for LightVec<T, N> {}

impl<T, const N: usize> Drop for LightVec<T, N> {
    fn drop(&mut self) {
        if self.cap <= N {
            unsafe {
                ptr::drop_in_place((&mut (*self.data.inline)[..self.cap]) as *mut _ as *mut [T]);
            }
        } else {
            unsafe {
                let (ptr, len) = self.data.heap;
                Vec::from_raw_parts(ptr, len, self.cap);
            }
        }
    }
}

impl<T, const N: usize> Deref for LightVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.cap <= N {
            unsafe {
                let slice = &(*self.data.inline)[..self.cap];
                &*(slice as *const _ as *const [T])
            }
        } else {
            unsafe {
                let (ptr, len) = self.data.heap;
                slice::from_raw_parts(ptr, len)
            }
        }
    }
}

impl<T, const N: usize> DerefMut for LightVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.cap <= N {
            unsafe {
                let slice = &mut (*self.data.inline)[..self.cap];
                &mut *(slice as *mut _ as *mut [T])
            }
        } else {
            unsafe {
                let (ptr, len) = self.data.heap;
                slice::from_raw_parts_mut(ptr, len)
            }
        }
    }
}

impl<T, const N: usize> AsRef<[T]> for LightVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> AsMut<[T]> for LightVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T: Clone, const N: usize> Clone for LightVec<T, N> {
    fn clone(&self) -> Self {
        if self.cap <= N {
            let mut res = Self::new();
            unsafe {
                for (src, dst) in self.iter().zip(&mut *res.data.inline) {
                    dst.write(src.clone());
                }
            }
            res.cap = self.cap;
            res
        } else {
            unsafe {
                let cloned_vec = self.with_heap(|vec| vec.clone());
                Self::heap_from_vec(cloned_vec)
            }
        }
    }
}

impl<T, const N: usize> Default for LightVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq, const N: usize> PartialEq for LightVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq, const N: usize> Eq for LightVec<T, N> {}

impl<T: PartialOrd, const N: usize> PartialOrd for LightVec<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Ord, const N: usize> Ord for LightVec<T, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash, const N: usize> Hash for LightVec<T, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T: Debug, const N: usize> Debug for LightVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T, const N: usize> From<Vec<T>> for LightVec<T, N> {
    fn from(mut vec: Vec<T>) -> Self {
        if vec.len() <= N {
            let mut res = Self::new();
            let len = vec.len();
            unsafe {
                vec.set_len(0);
                (*res.data.inline)
                    .as_mut_ptr()
                    .cast::<T>()
                    .copy_from_nonoverlapping(vec.as_ptr(), len);
            }
            res.cap = len;
            res
        } else {
            unsafe { Self::heap_from_vec(vec) }
        }
    }
}

impl<T, const N: usize> From<LightVec<T, N>> for Vec<T> {
    fn from(mut v: LightVec<T, N>) -> Self {
        if v.cap <= N {
            v.inline_to_heap();
        }
        unsafe { v.into_heap() }
    }
}

impl<T, const N: usize, const M: usize> From<[T; M]> for LightVec<T, N> {
    fn from(value: [T; M]) -> Self {
        if M <= N {
            let mut res = Self::new();
            unsafe {
                ptr::copy_nonoverlapping(value.as_ptr(), res.as_mut_ptr(), M);
            }
            mem::forget(value);
            res.cap = M;
            res
        } else {
            unsafe { Self::heap_from_vec(value.into()) }
        }
    }
}

impl<T: Clone, const N: usize> From<&[T]> for LightVec<T, N> {
    fn from(slice: &[T]) -> Self {
        if slice.len() <= N {
            let mut res = Self::new();
            for (i, elem) in slice.iter().enumerate() {
                unsafe {
                    res.as_mut_ptr().add(i).write(elem.clone());
                }
            }
            res.cap = slice.len();
            res
        } else {
            unsafe { Self::heap_from_vec(Vec::from(slice)) }
        }
    }
}

impl<T, const N: usize> IntoIterator for LightVec<T, N> {
    type IntoIter = IntoIter<T, N>;
    type Item = T;
    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter(if self.cap <= N {
            let len = self.cap;
            let iter = IntoIterInner::Inline {
                data: unsafe { ManuallyDrop::take(&mut self.data.inline) },
                i: 0,
                len,
            };
            mem::forget(self);
            iter
        } else {
            IntoIterInner::Heap(unsafe { self.into_heap().into_iter() })
        })
    }
}

pub struct IntoIter<T, const N: usize>(IntoIterInner<T, N>);
enum IntoIterInner<T, const N: usize> {
    Inline {
        data: [MaybeUninit<T>; N],
        i: usize,
        len: usize,
    },
    Heap(vec::IntoIter<T>),
}

impl<T: Clone, const N: usize> Clone for IntoIter<T, N> {
    fn clone(&self) -> Self {
        Self(match self.0 {
            IntoIterInner::Inline { ref data, i, len } => unsafe {
                let mut cloned_data: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
                for (src, dst) in data[i..len].iter().zip(cloned_data[..len - i].iter_mut()) {
                    dst.write(src.assume_init_ref().clone());
                }
                IntoIterInner::Inline {
                    data: cloned_data,
                    i: 0,
                    len: len - i,
                }
            },
            IntoIterInner::Heap(ref vec) => IntoIterInner::Heap(vec.clone()),
        })
    }
}
impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        if let IntoIterInner::Inline {
            ref mut data,
            i,
            len,
        } = self.0
        {
            unsafe { ptr::drop_in_place(data.get_unchecked_mut(i..len) as *mut _ as *mut [T]) }
        }
    }
}
impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IntoIterInner::Inline { data, i, len } => {
                if *i < *len {
                    let res = unsafe { data[*i].assume_init_read() };
                    *i += 1;
                    Some(res)
                } else {
                    None
                }
            }
            IntoIterInner::Heap(iter) => iter.next(),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match &mut self.0 {
            IntoIterInner::Inline { data, i, len } => unsafe {
                if n > 0 {
                    let new_i = (*i).saturating_add(n).min(*len);
                    ptr::drop_in_place((&mut data[*i..new_i]) as *mut _ as *mut [T]);
                    *i = new_i;
                }
                self.next()
            },
            IntoIterInner::Heap(iter) => iter.nth(n),
        }
    }
}
impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IntoIterInner::Inline { data, i, len } => {
                if *i < *len {
                    *len -= 1;
                    let res = unsafe { data[*len].assume_init_read() };
                    Some(res)
                } else {
                    None
                }
            }
            IntoIterInner::Heap(iter) => iter.next_back(),
        }
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match &mut self.0 {
            IntoIterInner::Inline { data, i, len } => unsafe {
                if n > 0 {
                    let new_len = (*len).saturating_sub(n).max(*i);
                    ptr::drop_in_place((&mut data[new_len..*len]) as *mut _ as *mut [T]);
                    *len = new_len;
                }
                self.next_back()
            },
            IntoIterInner::Heap(iter) => iter.nth_back(n),
        }
    }
}
impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    fn len(&self) -> usize {
        match &self.0 {
            IntoIterInner::Inline { i, len, .. } => len - i,
            IntoIterInner::Heap(iter) => iter.len(),
        }
    }
}

impl<'a, T: 'a, const N: usize> IntoIterator for &'a LightVec<T, N> {
    type IntoIter = slice::Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a, const N: usize> IntoIterator for &'a mut LightVec<T, N> {
    type IntoIter = slice::IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> FromIterator<T> for LightVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = Self::new();
        res.extend(iter);
        res
    }
}

impl<T, const N: usize> Extend<T> for LightVec<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        if self.cap <= N && self.cap + iter.size_hint().0 > N {
            self.inline_to_heap_with_capacity(self.cap + iter.size_hint().0);
        }
        while self.cap <= N {
            if let Some(item) = iter.next() {
                if self.cap == N {
                    self.inline_to_heap();
                    self.push(item);
                    break;
                }
                self.push(item);
            } else {
                return;
            }
        }
        unsafe {
            self.with_heap_mut(|vec| vec.extend(iter));
        }
    }
}
