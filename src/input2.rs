use std::{
    convert::TryInto,
    io::{self, Read},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr, slice,
};

pub struct Input<R> {
    src: R,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
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
    pub fn seq<T: Parse>(&mut self, n: usize) -> Seq<T, R> {
        Seq {
            src: self,
            n,
            marker: PhantomData,
        }
    }
    pub fn vec<T: Parse>(&mut self, n: usize) -> Vec<T> {
        self.seq(n).collect()
    }
    pub fn str(&mut self) -> &str {
        std::str::from_utf8(self.bytes()).expect("utf8 error")
    }
    pub fn bytes(&mut self) -> &[u8] {
        let range = self.bytes_inner();
        unsafe { self.buf.get_unchecked(range) }
    }
    pub fn bytes_vec(&mut self) -> Vec<u8> {
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
                if let Some(d) = find_ws_naive(unsafe {
                    self.buf.get_unchecked(self.pos + i..self.pos + self.len)
                }) {
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
            self.buf.resize(2 * self.buf.len(), 0);
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
}

/*
const USIZE_BYTES: usize = std::mem::size_of::<usize>();

#[inline]
fn find_ws(s: &[u8]) -> Option<usize> {
    if s.len() < 2 * USIZE_BYTES {
        return find_ws_naive(s);
    }
    find_ws_aligned(s)
}

fn find_ws_aligned(s: &[u8]) -> Option<usize> {
    let mut offset = s.as_ptr().align_offset(USIZE_BYTES);

    if offset > 0 {
        if let Some(i) = find_ws_naive(&s[..offset.min(s.len())]) {
            return Some(i);
        }
    }

    if offset >= s.len() {
        return None;
    }

    let mut s = &s[offset..];

    while USIZE_BYTES <= s.len() {
        if let Some(i) = find_ws_usize(usize::from_le_bytes(s[..USIZE_BYTES].try_into().unwrap())) {
            return Some(offset + i);
        }
        s = &s[USIZE_BYTES..];
        offset += USIZE_BYTES;
    }

    let i = find_ws_naive(&s)?;
    Some(offset + i)
}

#[inline]
fn find_ws_usize(s: usize) -> Option<usize> {
    const PAT: usize = 0x2121212121212121u64 as usize;
    const HI: usize = 0x8080808080808080u64 as usize;
    let x = s.wrapping_sub(PAT) & !s & HI;
    if x != 0 {
        Some(x.trailing_zeros() as usize / 8)
    } else {
        None
    }
}
 */

#[inline]
pub(crate) fn find_ws_naive(s: &[u8]) -> Option<usize> {
    for (i, c) in s.iter().enumerate() {
        if *c <= b' ' {
            return Some(i);
        }
    }
    None
}

pub(crate) fn find_ws_long(s: &[u8]) -> Option<usize> {
    let mut offset = s.as_ptr().align_offset(8);
    if let Some(pos) = find_ws_naive(&s[..offset.min(s.len())]) {
        return Some(pos);
    }
    if s.len() <= offset {
        return None;
    }
    while offset + 8 <= s.len() {
        let ss = &s[offset..offset + 8];
        let mut i = 0;
        let mut found = false;
        while i < 8 {
            found |= ss[i] <= b' ';
            i += 1;
        }
        if found {
            break;
        }
        offset += 8;
    }
    find_ws_naive(&s[offset..]).map(|pos| offset + pos)
}

pub(crate) fn find_ws(s: &[u8]) -> Option<usize> {
    let offset = s.as_ptr().align_offset(8);
    if let Some(pos) = find_ws_naive(&s[..(16 + offset).min(s.len())]) {
        return Some(pos);
    }
    if s.len() <= 16 + offset {
        return None;
    }
    find_ws_long(&s[16 + offset..]).map(|pos| pos + 16 + offset)
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
        (self.n, Some(self.n))
    }
}

pub trait Parse {
    fn parse<T: Read>(src: &mut Input<T>) -> Self;
}

impl Parse for Vec<u8> {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        src.bytes_vec()
    }
}

impl Parse for String {
    fn parse<T: Read>(src: &mut Input<T>) -> Self {
        String::from_utf8(src.bytes_vec()).unwrap()
    }
}

pub trait ParseBytes {
    fn parse_bytes(s: &[u8]) -> Self;
}

macro_rules! parse_int {
    ($ty:ident, $ity:ident) => {
        impl ParseBytes for $ty {
            fn parse_bytes(s: &[u8]) -> Self {
                $ty(s)
            }
        }

        impl ParseBytes for $ity {
            fn parse_bytes(s: &[u8]) -> Self {
                let (minus, s) = if let Some((b'-', s)) = s.split_first() {
                    (true, s)
                } else {
                    (false, s)
                };
                let x = $ty(s);
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
fn usize(s: &[u8]) -> usize {
    u32(s) as usize
}

#[cfg(target_pointer_width = "64")]
fn usize(s: &[u8]) -> usize {
    u64(s) as usize
}

#[inline]
fn u64(mut s: &[u8]) -> u64 {
    let mut res = 0;
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
fn u32(mut s: &[u8]) -> u32 {
    let mut res = 0;
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
fn u16(mut s: &[u8]) -> u16 {
    let mut res = 0;
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
fn u8(s: &[u8]) -> u8 {
    let mut res = 0;
    for &c in s {
        res = 10 * res + (c & 0xf);
    }
    res
}

/*
const TEN: [f32; 9] = [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8];

impl ParseBytes for f32 {
    fn parse_bytes(s: &[u8]) -> Self {
        const N: usize = 9;
        let (minus, s) = if let Some((b'-', s)) = s.split_first() {
            (true, s)
        } else {
            (false, s)
        };
        let (int, fract) = if let Some(p) = s.iter().position(|c| *c == b'.') {
            (&s[..p], &s[p + 1..])
        } else {
            (s, &[][..])
        };
        let (val, exp) = if int.len() <= N {
            (u32::parse_bytes(&int[..N]), (N - int.len()) as i32)
        } else {
            let n = fract.len().min(N - int.len());
        };
        todo!()
    }
}
 */
