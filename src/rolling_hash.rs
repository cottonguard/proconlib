use std::ops::{Bound, RangeBounds};

pub struct RollingHash {
    base: Vec<u64>,
    hash: Vec<u64>,
}

impl RollingHash {
    pub fn new<T: Int>(a: &[T], base: u64) -> Self {
        let base = rem(base);
        let mut bases = vec![0; a.len() + 1];
        bases[0] = 1;
        for i in 0..a.len() {
            bases[i + 1] = mul_rem(base, bases[i]);
        }
        let mut hash = vec![0; a.len() + 1];
        for i in 0..a.len() {
            let h = mul_rem(base, hash[i]) + a[i].hash();
            hash[i + 1] = if h < MOD { h } else { h - MOD };
        }
        Self { base: bases, hash }
    }

    pub fn len(&self) -> usize {
        self.hash.len() - 1
    }

    #[inline]
    pub fn hash<R: RangeBounds<usize>>(&self, r: R) -> u64 {
        let l = match r.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i - 1,
            Bound::Unbounded => 0,
        };
        let r = match r.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.hash.len(),
        };

        assert!(l <= r);
        assert!(r <= self.hash.len());

        if l != 0 {
            self.hash_substr(l, r)
        } else {
            self.hash_prefix(r)
        }
    }

    #[inline]
    pub fn hash_cyclic<R: RangeBounds<usize>>(&self, r: R) -> u64 {
        let l = match r.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i - 1,
            Bound::Unbounded => 0,
        };
        let r = match r.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => panic!("the end bound cannot be unbounded"),
        };

        assert!(l <= r);

        self.hash_cyclic_impl(l, r)
    }

    fn hash_prefix(&self, r: usize) -> u64 {
        self.hash[r]
    }

    fn hash_substr(&self, l: usize, r: usize) -> u64 {
        if self.hash.len() != self.base.len() {
            unsafe { std::hint::unreachable_unchecked() }
        }
        let h = self.hash[r] as i64 - mul_rem(self.base[r - l], self.hash[l]) as i64;
        (if h >= 0 { h } else { h + MOD as i64 }) as u64
    }

    fn hash_cyclic_impl(&self, l: usize, r: usize) -> u64 {
        let orig_l = l;
        let l = l % self.len();
        let r = r - (orig_l - l);

        if r <= self.len() {
            return self.hash_substr(l, r);
        }

        let mut hash = self.hash_substr(l, self.len());
        let mut l = self.len();
        while l + self.len() <= r {
            hash = mul_rem(hash, self.base[self.len()]);
            hash = add_rem(hash, self.hash[self.len()]);
            l += self.len();
        }
        let r = r - l;
        hash = mul_rem(hash, self.base[r]);
        hash = add_rem(hash, self.hash[r]);
        hash
    }

    pub fn base(&self, i: usize) -> u64 {
        self.base[i]
    }

    pub fn iter_hash<T: Int, I: IntoIterator<Item = T>>(&self, a: I) -> u64 {
        let base = self.base[1];
        let mut hash = 0;
        for a in a {
            let h = mul_rem(base, hash) + a.hash();
            hash = if h < MOD { h } else { h - MOD };
        }
        hash
    }

    pub fn find<T: Int, I: IntoIterator<Item = T>>(&self, a: I) -> Find
    where
        I::IntoIter: ExactSizeIterator,
    {
        let iter = a.into_iter();
        Find {
            rh: self,
            pat_len: iter.len(),
            pat_hash: self.iter_hash(iter),
            i: 0,
        }
    }
}

const EXP: u32 = 61;
const MOD: u64 = (1 << EXP) - 1;

macro_rules! rem {
    ($ty:ty, $name:ident) => {
        #[inline]
        fn $name(x: $ty) -> u64 {
            let r = ((x & MOD as $ty) + (x >> EXP)) as u64;
            if r < MOD {
                r
            } else {
                r - MOD
            }
        }
    };
}

rem!(u64, rem);
rem!(u128, rem128);

#[inline]
fn mul_rem(x: u64, y: u64) -> u64 {
    rem128(x as u128 * y as u128)
}

#[inline]
fn add_rem(x: u64, y: u64) -> u64 {
    let sum = x + y;
    if sum < MOD {
        sum
    } else {
        sum - MOD
    }
}

pub struct Find<'a> {
    rh: &'a RollingHash,
    pat_hash: u64,
    pat_len: usize,
    i: usize,
}

impl<'a> Iterator for Find<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i + self.pat_len <= self.rh.len() {
            if self.pat_hash == self.rh.hash(self.i..self.i + self.pat_len) {
                let res = self.i;
                self.i += 1;
                return Some(res);
            }
            self.i += 1;
        }

        None
    }
}

pub trait Int: Copy {
    fn hash(self) -> u64;
}

macro_rules! int {
    ($ty:ty, $ity:ty) => {
        impl Int for $ty {
            fn hash(self) -> u64 {
                self as u64 + 1
            }
        }

        impl Int for $ity {
            fn hash(self) -> u64 {
                (self as $ty).hash()
            }
        }
    };
}

// int!(usize, isize);
int!(u8, i8);
int!(u16, i16);
int!(u32, i32);
// int!(u64, i64);

/*
#[repr(transparent)]
pub struct Hash(u64);

impl std::ops::Add for
 */
