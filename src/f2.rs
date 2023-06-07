use std::{
    alloc::{self, Layout},
    fmt::{self, Debug},
    mem::{self, MaybeUninit},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound, Not, RangeBounds,
    },
    ptr::NonNull,
    slice,
};

type Int = usize;
const BITS: usize = 8 * mem::size_of::<Int>();

pub struct F2(NonNull<Header>);

impl F2 {
    pub fn zero(len: usize) -> Self {
        let ptr = Header::alloc_zeroed(len);
        unsafe { Self(NonNull::new_unchecked(ptr)) }
    }

    unsafe fn uninit(len: usize) -> Self {
        unsafe {
            let ptr = Header::alloc(len);
            ptr.write(Header { len });
            Self(NonNull::new_unchecked(ptr))
        }
    }

    pub fn from_chunks<I: IntoIterator<Item = Int>>(len: usize, iter: I) -> Self {
        unsafe {
            let mut res = Self::uninit(len);
            for (c, val) in res.uninit_chunks_mut().iter_mut().zip(iter) {
                c.write(val);
            }
            res
        }
    }

    pub fn len(&self) -> usize {
        self.header().len
    }

    pub fn len_chunks(&self) -> usize {
        (self.len() + BITS - 1) / BITS
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.len());
        let c = unsafe { *self.chunks().get_unchecked(i / BITS) };
        (c >> (i % BITS)) & 1 == 1
    }

    pub fn set(&mut self, i: usize, f: bool) -> bool {
        assert!(i < self.len());
        let c = unsafe { self.chunks_mut().get_unchecked_mut(i / BITS) };
        let orig = (*c >> (i % BITS)) & 1 == 1;
        let mask = 1 << (i % BITS);
        if f {
            *c |= mask;
        } else {
            *c &= !mask;
        }
        orig
    }

    fn range_apply<R: RangeBounds<usize>, F: FnMut(Int) -> Int, G: FnMut(Int, Int) -> Int>(
        &mut self,
        range: R,
        mut f_full: F,
        mut f_masked: G,
    ) {
        let l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l - 1,
        };
        let r = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
        };
        assert!(l <= r);
        assert!(r <= self.len());
        let mut lc = l / BITS;
        let li = l % BITS;
        let rc = r / BITS;
        let ri = r % BITS;
        if lc == rc {
            let c = &mut self.chunks_mut()[lc];
            *c = f_masked(*c, (!0 << li) & (!0 >> (BITS - ri)));
            return;
        }
        if li != 0 {
            let c = &mut self.chunks_mut()[lc];
            *c = f_masked(*c, !0 << li);
            lc += 1;
        }
        for c in &mut self.chunks_mut()[lc..rc] {
            *c = f_full(*c);
        }
        if ri != 0 {
            let c = &mut self.chunks_mut()[lc];
            *c = f_masked(*c, !0 >> (BITS - ri));
        }
    }

    pub fn range_fill<R: RangeBounds<usize>>(&mut self, range: R, f: bool) {
        if f {
            self.range_apply(range, |_| !0, |x, mask| x | mask)
        } else {
            self.range_apply(range, |_| 0, |x, mask| x & !mask)
        }
    }

    pub fn range_not<R: RangeBounds<usize>>(&mut self, range: R) {
        self.range_apply(range, |x| !x, |x, mask| x ^ mask)
    }

    pub fn chunks(&self) -> &[Int] {
        unsafe {
            let ptr = self.0.as_ptr().add(1) as *const Int;
            slice::from_raw_parts(ptr, self.len_chunks())
        }
    }

    pub fn chunks_mut(&mut self) -> &mut [Int] {
        unsafe {
            let chunks_ptr = self.0.as_ptr().add(1) as *mut Int;
            slice::from_raw_parts_mut(chunks_ptr, self.len_chunks())
        }
    }

    fn uninit_chunks_mut(&mut self) -> &mut [MaybeUninit<Int>] {
        unsafe {
            let chunks_ptr = self.0.as_ptr().add(1) as *mut MaybeUninit<Int>;
            slice::from_raw_parts_mut(chunks_ptr, self.len_chunks())
        }
    }

    fn header(&self) -> &Header {
        unsafe { &*self.0.as_ptr() }
    }

    fn zip_map(&self, other: &Self, mut f: impl FnMut(Int, Int) -> Int) -> Self {
        assert_eq!(self.len(), other.len());
        let mut res = unsafe { F2::uninit(self.len()) };
        for i in 0..self.len_chunks() {
            res.uninit_chunks_mut()[i].write(f(self.chunks()[i], other.chunks()[i]));
        }
        res
    }
}

impl PartialEq for F2 {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.chunks() == other.chunks()
    }
}

impl Eq for F2 {}

impl Clone for F2 {
    fn clone(&self) -> Self {
        let mut res = unsafe { F2::uninit(self.len()) };
        for (src, dst) in self.chunks().iter().zip(res.uninit_chunks_mut()) {
            dst.write(*src);
        }
        res
    }
}

impl Debug for F2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("F2 { ... }")
    }
}

impl Drop for F2 {
    fn drop(&mut self) {
        unsafe { Header::dealloc(self.0.as_ptr()) }
    }
}

impl Not for F2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.range_not(..);
        self
    }
}

macro_rules! ops {
    ($Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl $Op for F2 {
            type Output = F2;
            fn $op(mut self, other: Self) -> Self::Output {
                self.$op_assign(&other);
                self
            }
        }
        impl $Op<&Self> for F2 {
            type Output = F2;
            fn $op(mut self, other: &Self) -> Self::Output {
                self.$op_assign(other);
                self
            }
        }
        impl $Op<F2> for &F2 {
            type Output = F2;
            fn $op(self, mut other: F2) -> Self::Output {
                other.$op_assign(self);
                other
            }
        }
        impl $Op for &F2 {
            type Output = F2;
            fn $op(self, other: Self) -> Self::Output {
                self.zip_map(other, Int::$op)
            }
        }
        impl $OpAssign<&Self> for F2 {
            fn $op_assign(&mut self, other: &Self) {
                for (x, y) in self.chunks_mut().iter_mut().zip(other.chunks()) {
                    x.$op_assign(*y);
                }
            }
        }
    };
}

ops!(BitAnd, bitand, BitAndAssign, bitand_assign);
ops!(BitOr, bitor, BitOrAssign, bitor_assign);
ops!(BitXor, bitxor, BitXorAssign, bitxor_assign);

#[repr(C)]
struct Header {
    len: usize,
}

impl Header {
    fn layout(n: usize) -> Layout {
        let len_chunks = (n + BITS - 1) / BITS;
        Layout::from_size_align(
            mem::size_of::<usize>() + len_chunks * mem::size_of::<Int>(),
            mem::align_of::<usize>().max(mem::align_of::<Int>()),
        )
        .unwrap()
    }

    unsafe fn alloc(len: usize) -> *mut Self {
        unsafe {
            let ptr = alloc::alloc(Self::layout(len)) as *mut Header;
            ptr.write(Self { len });
            ptr
        }
    }

    fn alloc_zeroed(len: usize) -> *mut Self {
        unsafe {
            let ptr = alloc::alloc_zeroed(Self::layout(len)) as *mut Header;
            ptr.write(Self { len });
            ptr
        }
    }

    unsafe fn dealloc(ptr: *mut Self) {
        let n = unsafe { (*ptr).len };
        alloc::dealloc(ptr as _, Self::layout(n))
    }
}
