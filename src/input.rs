use std::{
    io::{self, prelude::*},
    marker::PhantomData,
};

#[macro_export]
macro_rules! input {
    ($src: expr, $($var: ident $(($count: expr $(, $map: expr)?))? $(: $ty: ty)?),* $(,)?) => {
        $(input!(@ $src, $var $(($count $(, $map)?))? $(: $ty)?) ;)*
    };
    (@ $src: expr, $var: ident $(: $ty: ty)?) => {
        let $var $(: $ty)? = $src.parse();
    };
    (@ $src: expr, $var: ident ($count: expr): $($ty: ty)?) => {
        let $var $(: $ty)? = $src.seq().take($count).collect();
    };
    (@ $src: expr, $var: ident ($count: expr, $map: expr): $($ty: ty)?) => {
        let $var $(: $ty)? = $src.seq().take($count).map($map).collect();
    };
}

pub trait Input {
    fn bytes(&mut self) -> &[u8];

    fn bytes_vec(&mut self) -> Vec<u8> {
        self.bytes().to_vec()
    }

    fn str(&mut self) -> &str {
        std::str::from_utf8(self.bytes()).unwrap()
    }

    fn parse<T: Parse>(&mut self) -> T {
        self.parse_with(DefaultParser)
    }

    fn parse_with<T>(&mut self, mut parser: impl Parser<T>) -> T {
        parser.parse(self)
    }

    fn seq<T: Parse>(&mut self) -> Seq<T, Self, DefaultParser> {
        self.seq_with(DefaultParser)
    }

    fn seq_with<T, P: Parser<T>>(&mut self, parser: P) -> Seq<T, Self, P> {
        Seq {
            input: self,
            parser,
            marker: PhantomData,
        }
    }

    fn collect<T: Parse, C: std::iter::FromIterator<T>>(&mut self, n: usize) -> C {
        self.seq().take(n).collect()
    }
}

impl<T: Input> Input for &mut T {
    fn bytes(&mut self) -> &[u8] {
        (**self).bytes()
    }
}

pub trait Parser<T> {
    fn parse<I: Input + ?Sized>(&mut self, s: &mut I) -> T;
}

impl<T, P: Parser<T>> Parser<T> for &mut P {
    fn parse<I: Input + ?Sized>(&mut self, s: &mut I) -> T {
        (**self).parse(s)
    }
}

pub trait Parse {
    fn parse<I: Input + ?Sized>(s: &mut I) -> Self;
}

pub struct DefaultParser;

impl<T: Parse> Parser<T> for DefaultParser {
    fn parse<I: Input + ?Sized>(&mut self, s: &mut I) -> T {
        T::parse(s)
    }
}

pub struct Seq<'a, T, I: ?Sized, P> {
    input: &'a mut I,
    parser: P,
    marker: PhantomData<*const T>,
}

impl<'a, T, I: Input + ?Sized, P: Parser<T>> Iterator for Seq<'a, T, I, P> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.input.parse_with(&mut self.parser))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (!0, None)
    }
}

impl Parse for char {
    #[inline]
    fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
        let s = s.bytes();
        debug_assert_eq!(s.len(), 1);
        *s.first().expect("zero length") as char
    }
}

macro_rules! tuple {
    ($($T:ident),*) => {
        impl<$($T: Parse),*> Parse for ($($T,)*) {
            #[inline]
            #[allow(unused_variables)]
            #[allow(clippy::unused_unit)]
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                ($($T::parse(s),)*)
            }
        }
    };
}

tuple!();
tuple!(A);
tuple!(A, B);
tuple!(A, B, C);
tuple!(A, B, C, D);
tuple!(A, B, C, D, E);
tuple!(A, B, C, D, E, F);
tuple!(A, B, C, D, E, F, G);

#[cfg(feature = "newer")]
impl<T: Parse, const N: usize> Parse for [T; N] {
    fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
        use std::{
            mem::{self, MaybeUninit},
            ptr,
        };
        struct Guard<T, const N: usize> {
            arr: [MaybeUninit<T>; N],
            i: usize,
        }
        impl<T, const N: usize> Drop for Guard<T, N> {
            fn drop(&mut self) {
                unsafe {
                    ptr::drop_in_place(&mut self.arr[..self.i] as *mut _ as *mut [T]);
                }
            }
        }
        let mut g = Guard::<T, N> {
            arr: unsafe { MaybeUninit::uninit().assume_init() },
            i: 0,
        };
        while g.i < N {
            g.arr[g.i] = MaybeUninit::new(s.parse());
            g.i += 1;
        }
        unsafe { mem::transmute_copy(&g.arr) }
    }
}

macro_rules! uint {
    ($ty:ty) => {
        impl Parse for $ty {
            #[inline]
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                let s = s.bytes();
                s.iter().fold(0, |x, d| 10 * x + (0xf & d) as $ty)
            }
        }
    };
}

macro_rules! int {
    ($ty:ty) => {
        impl Parse for $ty {
            #[inline]
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                let f = |s: &[u8]| {
                    s.iter()
                        .fold(0 as $ty, |x, d| (10 * x).wrapping_add((0xf & d) as $ty))
                };
                let s = s.bytes();
                if let Some((b'-', s)) = s.split_first() {
                    f(s).wrapping_neg()
                } else {
                    f(s)
                }
            }
        }
    };
}

macro_rules! float {
    ($ty:ty) => {
        impl Parse for $ty {
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                const POW: [$ty; 18] = [
                    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14,
                    1e15, 1e16, 1e17,
                ];
                let s = s.bytes();
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
                let x = int
                    .iter()
                    .chain(fract)
                    .fold(0u64, |x, d| 10 * x + (0xf & *d) as u64);
                let x = x as $ty;
                let x = if minus { -x } else { x };
                let exp = fract.len();
                if exp == 0 {
                    x
                } else if let Some(pow) = POW.get(exp) {
                    x / pow
                } else {
                    x / (10.0 as $ty).powi(exp as i32)
                }
            }
        }
    };
}

macro_rules! from_bytes {
    ($ty:ty) => {
        impl Parse for $ty {
            #[inline]
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                s.bytes().into()
            }
        }
    };
}

macro_rules! from_str {
    ($ty:ty) => {
        impl Parse for $ty {
            #[inline]
            fn parse<I: Input + ?Sized>(s: &mut I) -> Self {
                s.str().into()
            }
        }
    };
}

macro_rules! impls {
    ($m:ident, $($ty:ty),*) => { $($m!($ty);)* };
}

impls!(uint, usize, u8, u16, u32, u64, u128);
impls!(int, isize, i8, i16, i32, i64, i128);
impls!(float, f32, f64);
impls!(from_bytes, Vec<u8>, Box<[u8]>);
impls!(from_str, String);

#[derive(Clone)]
pub struct SplitWs<T> {
    src: T,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

const BUF_SIZE: usize = 1 << 26;

impl<T: Read> SplitWs<T> {
    pub fn new(src: T) -> Self {
        Self {
            src,
            buf: vec![0; BUF_SIZE],
            pos: 0,
            len: 0,
        }
    }

    #[inline(always)]
    fn peek(&self) -> &[u8] {
        unsafe { self.buf.get_unchecked(self.pos..self.len) }
    }

    #[inline(always)]
    fn consume(&mut self, n: usize) -> &[u8] {
        let pos = self.pos;
        self.pos += n;
        unsafe { self.buf.get_unchecked(pos..self.pos) }
    }

    fn read(&mut self) -> usize {
        self.buf.copy_within(self.pos..self.len, 0);
        self.len -= self.pos;
        self.pos = 0;
        if self.len == self.buf.len() {
            self.buf.resize(2 * self.buf.len(), 0);
        }
        loop {
            match self.src.read(&mut self.buf[self.len..]) {
                Ok(n) => {
                    self.len += n;
                    return n;
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => panic!("io error: {:?}", e),
            }
        }
    }
}

impl<T: Read> Input for SplitWs<T> {
    #[inline]
    fn bytes(&mut self) -> &[u8] {
        loop {
            if let Some(del) = self.peek().iter().position(|c| c.is_ascii_whitespace()) {
                if del > 0 {
                    let s = self.consume(del + 1);
                    return s.split_last().unwrap().1;
                } else {
                    self.consume(1);
                }
            } else if self.read() == 0 {
                return self.consume(self.len - self.pos);
            }
        }
    }

    /*
    fn bytes_vec(&mut self) -> Vec<u8> {
        let bytes = self.bytes();
        let bytes_len = bytes.len();
        if self.pos == 0 && 2 * bytes_len >= self.buf.len() {
            let mut new_buf = vec![0; self.buf.len()];
            new_buf[..self.len - self.pos].copy_from_slice(&self.buf[self.pos..self.len]);
            self.len -= self.pos;
            self.pos = 0;
            let mut res = mem::replace(&mut self.buf, new_buf);
            res.truncate(bytes_len);
            res
        } else {
            bytes.to_vec()
        }
    }
    */
}
