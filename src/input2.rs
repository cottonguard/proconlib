use std::{
    convert::TryInto,
    io::{self, Read},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr, slice,
};

pub struct Input<R: ?Sized> {
    buf: Vec<u8>,
    pos: usize,
    len: usize,
    src: R,
}

macro_rules! def_input2 {
    ($f:ident, $ty:ty) => {
        pub fn $f(&mut self) -> $ty {
            self.input()
        }
    };
}

macro_rules! def_input {
    ($ty:ident) => {
        def_input2!($ty, $ty);
    };
}

impl<R: Read> Input<R> {
    pub fn new(src: R) -> Self {
        Self::with_capacity(src, 1 << 20)
    }
    pub fn with_capacity(src: R, cap: usize) -> Self {
        Self {
            src,
            buf: vec![0; cap],
            pos: 0,
            len: 0,
        }
    }
    pub fn input<T: Parse>(&mut self) -> T {
        T::parse(self)
    }
    pub fn map<T, F: MapOnce<T>>(&mut self, f: F) -> F::Output {
        f.map(self)
    }
    pub fn seq<T: Parse>(&mut self, n: usize) -> Seq<T, R> {
        Seq {
            src: self,
            n,
            marker: PhantomData,
        }
    }
    pub fn seq_map<T, F: Map<T>>(&mut self, n: usize, f: F) -> SeqMap<T, F, R> {
        SeqMap {
            src: self,
            n,
            f,
            marker: PhantomData,
        }
    }
    pub fn vec<T: Parse>(&mut self, n: usize) -> Vec<T> {
        self.seq(n).collect()
    }
    pub fn map_vec<T, F: Map<T>>(&mut self, n: usize, f: F) -> Vec<F::Output> {
        self.seq_map(n, f).collect()
    }
    pub fn str(&mut self) -> &str {
        std::str::from_utf8(self.bytes()).expect("utf8 error")
    }
    pub fn bytes(&mut self) -> &[u8] {
        let range = self.bytes_inner();
        unsafe { self.buf.get_unchecked(range) }
    }
    pub fn byte_vec(&mut self) -> Vec<u8> {
        let range = self.bytes_inner();
        if range.start == 0 && 2 * range.end >= self.buf.len() {
            let buf_len = self.buf.len();
            let mut new_buf = vec![0; buf_len];
            new_buf[..self.len].copy_from_slice(self.remaining());
            let mut res = mem::replace(&mut self.buf, new_buf);
            self.pos = 0;
            res.truncate(range.end);
            res
        } else {
            self.buf[range].to_vec()
        }
    }
    #[inline]
    fn bytes_inner(&mut self) -> std::ops::Range<usize> {
        let mut i = 0;
        loop {
            if self.len > 0 {
                if let Some(d) =
                    find_ws(unsafe { self.buf.get_unchecked(self.pos + i..self.pos + self.len) })
                {
                    let del = i + d;
                    let range = self.pos..self.pos + del;
                    self.pos += del + 1;
                    self.len -= del + 1;
                    if del == 0 {
                        continue;
                    }
                    return range;
                }
                i = self.len;
            }
            if self.read() == 0 {
                let range = self.pos..self.pos + self.len;
                self.pos = 0;
                self.len = 0;
                return range;
            }
        }
    }
    #[cold]
    fn read(&mut self) -> usize {
        if self.pos != 0 {
            self.buf.copy_within(self.pos..self.pos + self.len, 0);
            self.pos = 0;
        }
        if self.len == self.buf.len() {
            self.buf.resize((2 * self.buf.len()).max(1 << 13), 0);
        }
        loop {
            match self
                .src
                .read(unsafe { self.buf.get_unchecked_mut(self.len..) })
            {
                Ok(n) => {
                    self.len += n;
                    return n;
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => panic!("io error: {}", e),
            }
        }
    }
    #[inline]
    fn remaining(&self) -> &[u8] {
        unsafe { self.buf.get_unchecked(self.pos..self.pos + self.len) }
    }
    def_input!(usize);
    def_input!(u8);
    def_input!(u16);
    def_input!(u32);
    def_input!(u64);
    def_input!(isize);
    def_input!(i8);
    def_input!(i16);
    def_input!(i32);
    def_input!(i64);
    def_input!(f32);
    def_input!(f64);
    def_input!(char);
    def_input2!(string, String);
}

pub trait Map<T> {
    type Output;
    fn map<R: Read>(&mut self, src: &mut Input<R>) -> Self::Output;
}
pub trait MapOnce<T> {
    type Output;
    fn map<R: Read>(self, src: &mut Input<R>) -> Self::Output;
}

macro_rules! map {
    ($($T:ident),*) => {
        impl<$($T: Parse,)* O, F: FnMut($($T),+) -> O> Map<fn($($T),+)> for F {
            type Output = O;
            fn map<R: Read>(&mut self, src: &mut Input<R>) -> O {
                self($(src.input::<$T>()),*)
            }
        }
        impl<$($T: Parse,)* O, F: FnOnce($($T),+) -> O> MapOnce<fn($($T),+)> for F {
            type Output = O;
            fn map<R: Read>(self, src: &mut Input<R>) -> O {
                self($(src.input::<$T>()),*)
            }
        }
    };
}

map!(A);
map!(A, B);
map!(A, B, C);
map!(A, B, C, D);
map!(A, B, C, D, E);

/*
#[inline]
pub(crate) fn find_ws_naive(s: &[u8]) -> Option<usize> {
    for (i, c) in s.iter().enumerate() {
        if *c <= b' ' {
            return Some(i);
        }
    }
    None
}
 */

const CHUNK_SIZE: usize = mem::size_of::<usize>();

#[inline]
pub(crate) fn find_ws(s: &[u8]) -> Option<usize> {
    let offset = (32 + s.as_ptr().align_offset(CHUNK_SIZE)).min(s.len());
    let mut i = 0;
    while i < offset {
        if s[i] <= b' ' {
            return Some(i);
        }
        i += 1;
    }
    if i < s.len() {
        find_ws_long(s, i)
    } else {
        None
    }
}

fn find_ws_long(s: &[u8], mut i: usize) -> Option<usize> {
    while i + CHUNK_SIZE <= s.len() {
        if let Some(j) = find_ws_usize(usize::from_le_bytes(
            unsafe { s.get_unchecked(i..i + CHUNK_SIZE) }
                .try_into()
                .unwrap(),
        )) {
            return Some(i + j);
        }
        i += CHUNK_SIZE;
    }
    while i < s.len() {
        if s[i] <= b' ' {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[inline]
fn find_ws_usize(s: usize) -> Option<usize> {
    const SUB: usize = 0x2121212121212121;
    const MASK: usize = 0x8080808080808080;
    let t = s.wrapping_sub(SUB) & MASK;
    (t != 0).then(|| (t.trailing_zeros() / 8) as usize)
}

pub struct Seq<'a, T, R> {
    src: &'a mut Input<R>,
    n: usize,
    marker: PhantomData<*const T>,
}

impl<'a, T: Parse, R: Read> Iterator for Seq<'a, T, R> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            self.n -= 1;
            Some(self.src.input())
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T: Parse, R: Read> ExactSizeIterator for Seq<'a, T, R> {
    fn len(&self) -> usize {
        self.n
    }
}

pub struct SeqMap<'a, T, F, R> {
    src: &'a mut Input<R>,
    n: usize,
    f: F,
    marker: PhantomData<*const T>,
}

impl<'a, T, F: Map<T>, R: Read> Iterator for SeqMap<'a, T, F, R> {
    type Item = F::Output;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            self.n -= 1;
            Some(self.f.map(self.src))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, F: Map<T>, R: Read> ExactSizeIterator for SeqMap<'a, T, F, R> {
    fn len(&self) -> usize {
        self.n
    }
}

pub trait Parse {
    fn parse<T: Read>(src: &mut Input<T>) -> Self;
}

impl Parse for Vec<u8> {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        src.byte_vec()
    }
}

impl Parse for String {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        String::from_utf8(src.byte_vec()).unwrap()
    }
}

pub trait ParseBytes {
    fn parse_bytes(s: &[u8]) -> Self;
}

macro_rules! parse_int {
    ($ty:ident, $ity:ident) => {
        impl ParseBytes for $ty {
            fn parse_bytes(s: &[u8]) -> Self {
                $ty(s, 0)
            }
        }

        impl ParseBytes for $ity {
            fn parse_bytes(s: &[u8]) -> Self {
                let (minus, s) = if let Some((b'-', s)) = s.split_first() {
                    (true, s)
                } else {
                    (false, s)
                };
                let x = $ty(s, 0);
                (if minus { (!x).wrapping_add(1) } else { x }) as $ity
            }
        }
    };
}

parse_int!(usize, isize);
parse_int!(u8, i8);
parse_int!(u16, i16);
parse_int!(u32, i32);
parse_int!(u64, i64);

macro_rules! parse {
    ($ty:ident) => {
        impl Parse for $ty {
            fn parse<T: Read>(src: &mut Input<T>) -> Self {
                Self::parse_bytes(src.bytes())
            }
        }
    };
}

parse!(usize);
parse!(u8);
parse!(u16);
parse!(u32);
parse!(u64);
parse!(isize);
parse!(i8);
parse!(i16);
parse!(i32);
parse!(i64);
parse!(f32);
parse!(f64);

macro_rules! tuple {
    ($($T:ident),+) => {
        impl<$($T: Parse),+> Parse for ($($T,)+) {
            fn parse<T: Read>(src: &mut Input<T>) -> Self {
                ($($T::parse(src),)+)
            }
        }
    };
}

tuple!(A);
tuple!(A, B);
tuple!(A, B, C);
tuple!(A, B, C, D);
tuple!(A, B, C, D, E);
tuple!(A, B, C, D, E, F);
tuple!(A, B, C, D, E, F, G);
tuple!(A, B, C, D, E, F, G, H);

impl<T: Parse, const N: usize> Parse for [T; N] {
    fn parse<R: Read>(src: &mut Input<R>) -> Self {
        struct Guard<T> {
            ptr: *mut T,
            i: usize,
        }
        impl<T> Drop for Guard<T> {
            fn drop(&mut self) {
                unsafe {
                    ptr::drop_in_place(slice::from_raw_parts_mut(self.ptr, self.i));
                }
            }
        }
        let mut res: MaybeUninit<[T; N]> = MaybeUninit::uninit();
        let mut g = Guard {
            ptr: res.as_mut_ptr() as *mut T,
            i: 0,
        };
        unsafe {
            while g.i < N {
                g.ptr.add(g.i).write(src.input());
                g.i += 1;
            }
            mem::forget(g);
            res.assume_init()
        }
    }
}

#[inline]
fn toi8bytes(s: &[u8]) -> (u32, &[u8]) {
    let (p, rest) = s.split_at(8);
    let x = u64::from_le_bytes(p.try_into().unwrap());
    const MASK1: u64 = 0x000f000f000f000f;
    let hi = (x >> 8) & MASK1;
    let lo = x & MASK1;
    let x = 10 * lo + hi;
    const MASK2: u64 = 0x0000ffff0000ffff;
    let hi = (x >> 16) & MASK2;
    let lo = x & MASK2;
    let x = 100 * lo + hi;
    let hi = (x >> 32) as u32;
    let lo = x as u32;
    let x = 10000 * lo + hi;
    (x, rest)
}

#[inline]
fn toi4bytes(s: &[u8]) -> (u32, &[u8]) {
    let (p, rest) = s.split_at(4);
    let x = u32::from_le_bytes(p.try_into().unwrap());
    const MASK: u32 = 0x000f000f;
    let hi = (x >> 8) & MASK;
    let lo = x & MASK;
    let x = 10 * lo + hi;
    let hi = x >> 16;
    let lo = x & 0x0000ffff;
    let x = 100 * lo + hi;
    (x, rest)
}

#[cfg(target_pointer_width = "32")]
fn usize(s: &[u8], pre: usize) -> usize {
    u32(s, pre as u32) as usize
}

#[cfg(target_pointer_width = "64")]
fn usize(s: &[u8], pre: usize) -> usize {
    u64(s, pre as u64) as usize
}

#[inline]
fn u64(mut s: &[u8], pre: u64) -> u64 {
    let mut res = pre;
    while s.len() >= 8 {
        let (x, rest) = toi8bytes(s);
        res = 100000000 * res + x as u64;
        s = rest;
    }
    if s.len() >= 4 {
        let (x, rest) = toi4bytes(s);
        res = 10000 * res + x as u64;
        s = rest;
    }
    for &c in s {
        res = 10 * res + (c & 0xf) as u64;
    }
    res
}

#[inline]
fn u32(mut s: &[u8], pre: u32) -> u32 {
    let mut res = pre;
    if s.len() >= 8 {
        let (x, rest) = toi8bytes(s);
        res = x;
        s = rest;
    }
    if s.len() >= 4 {
        let (x, rest) = toi4bytes(s);
        res = 10000 * res + x;
        s = rest;
    }
    for &c in s {
        res = 10 * res + (c & 0xf) as u32;
    }
    res
}

#[inline]
fn u16(mut s: &[u8], pre: u16) -> u16 {
    let mut res = pre;
    if s.len() >= 4 {
        let (x, rest) = toi4bytes(s);
        res = 10000 * res + x as u16;
        s = rest;
    }
    for &c in s {
        res = 10 * res + (c & 0xf) as u16;
    }
    res
}

#[inline]
fn u8(s: &[u8], pre: u8) -> u8 {
    let mut res = pre;
    for &c in s {
        res = 10 * res + (c & 0xf);
    }
    res
}

macro_rules! float {
    ($ty:ident, $uty:ident) => {
        impl ParseBytes for $ty {
            fn parse_bytes(s: &[u8]) -> Self {
                const TEN: [$ty; 18] = [
                    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14,
                    1e15, 1e16, 1e17,
                ];

                let (minus, s) = if let Some((b'-', s)) = s.split_first() {
                    (true, s)
                } else {
                    (false, s)
                };
                let (int, fract) = if let Some(p) = s.iter().position(|c| *c == b'.') {
                    (&s[..p], &s[p + 1..])
                } else {
                    (s, &s[..0])
                };
                let x = $uty(int, 0);
                let x = if fract.is_empty() {
                    x as $ty
                } else {
                    let ten = TEN
                        .get(fract.len())
                        .copied()
                        .unwrap_or_else(|| $ty::powi(10.0, fract.len() as _));
                    $uty(fract, x) as $ty / ten
                };
                if minus {
                    -x
                } else {
                    x
                }
            }
        }
    };
}

float!(f32, u32);
float!(f64, u64);

impl Parse for char {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        let s = src.str();
        let mut cs = s.chars();
        match cs.next() {
            Some(c) if cs.as_str().is_empty() => c,
            _ => panic!("input is not single char"),
        }
    }
}

pub struct Byte(pub u8);

impl Parse for Byte {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        if let [b] = src.bytes() {
            Byte(*b)
        } else {
            panic!("input is not single byte")
        }
    }
}
