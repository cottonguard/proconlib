use std::io::{self, prelude::*};

pub struct Output<W> {
    dst: W,
}

impl<W: Write> Output<W> {
    pub fn new(dst: W) -> Self {
        Self { dst }
    }
    pub fn flush(&mut self) {
        self.dst.flush().expect("io error");
    }
    pub fn line(&mut self) -> Line<W> {
        Line {
            out: self,
            ws: false,
            to_flush: cfg!(debug_assertions),
        }
    }
    /*
    pub fn put<T: Put>(&mut self, value: T) -> Line<W> {
        let mut line = self.line();
        line.put(value);
        line
    }
     */
    pub fn putln<T: Put>(&mut self, value: T) -> &mut Self {
        self.line().put(value);
        self
    }
    pub fn iter_lines<T: Put>(&mut self, iter: impl IntoIterator<Item = T>) -> &mut Self {
        for value in iter {
            self.line().put(value);
        }
        self
    }
    fn byte(&mut self, c: u8) -> &mut Self {
        self.dst
            .write_all(std::slice::from_ref(&c))
            .expect("io error");
        self
    }
}

pub trait Put {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()>;
}

impl<T: Put + ?Sized> Put for &T {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        (*self).put(dst)
    }
}

impl Put for [u8] {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        dst.write_all(self)
    }
}

impl Put for str {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        dst.write_all(self.as_bytes())
    }
}

macro_rules! deref {
    ($ty:ty) => {
        impl Put for $ty {
            fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
                (&**self).put(dst)
            }
        }
    };
}

deref!(Vec<u8>);
deref!(Box<[u8]>);
deref!(String);

impl Put for char {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        dst.write_all(self.encode_utf8(&mut [0; 4]).as_bytes())
    }
}

macro_rules! int {
    ($uty:ident, $ity:ident, $N:expr) => {
        impl Put for $uty {
            fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
                let mut buf = [0; $N];
                buf[0] = b'0';
                let mut i = 0;
                let mut x = *self;
                while x > 0 {
                    buf[i] = b'0' + (x % 10) as u8;
                    x /= 10;
                    i += 1;
                }
                buf[..i].reverse();
                dst.write_all(&buf[..i.max(1)])
            }
        }
        impl Put for $ity {
            fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
                let mut buf = [0; $N + 1];
                buf[0] = b'-';
                buf[1] = b'0';
                let mut i = 1;
                let neg = *self < 0;
                let mut x = self.abs() as $uty;
                while x > 0 {
                    buf[i] = b'0' + (x % 10) as u8;
                    x /= 10;
                    i += 1;
                }
                buf[1..i].reverse();
                let l = 1 - neg as usize;
                dst.write_all(&buf[l..i.max(2)])
            }
        }
    };
}

int!(u8, i8, 3);
int!(u16, i16, 5);
int!(u32, i32, 10);
int!(u64, i64, 20);
int!(u128, i128, 39);
#[cfg(target_pointer_width = "32")]
int!(usize, isize, 10);
#[cfg(target_pointer_width = "64")]
int!(usize, isize, 20);

macro_rules! fmt {
    ($ty:ident) => {
        impl Put for $ty {
            fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
                write!(dst, "{}", self)
            }
        }
    };
}

fmt!(f32);
fmt!(f64);

pub struct Byte(u8);

impl Put for Byte {
    fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        dst.write_all(std::slice::from_ref(&self.0))
    }
}

macro_rules! tuple {
    () => {};
    ($T:ident $(,$U:ident)*) => {
        tuple!($($U),*);
        impl<$T: Put, $($U: Put),*> Put for ($T, $($U),*) {
            fn put<W: Write>(&self, dst: &mut W) -> io::Result<()> {
                #![allow(non_snake_case)]
                let ($T, $($U),*) = self;
                $T.put(dst)?;
                $(
                    Byte(b' ').put(dst)?;
                    $U.put(dst)?;
                )*
                Ok(())
            }
        }
    };
}

tuple!(A, B, C, D, E, F, G, H);

pub struct Line<'a, W: Write> {
    out: &'a mut Output<W>,
    ws: bool,
    to_flush: bool,
}

impl<'a, W: Write> Line<'a, W> {
    pub fn endl(self) /*-> &'a mut Output<W>*/ {}
    pub fn put<T: Put>(&mut self, value: T) -> &mut Self {
        self.ws();
        value.put(&mut self.out.dst).expect("io error");
        self.ws = true;
        self
    }
    pub fn iter<T: Put>(&mut self, iter: impl IntoIterator<Item = T>) -> &mut Self {
        for value in iter {
            self.put(value);
        }
        self
    }
    fn byte(&mut self, c: u8) -> &mut Self {
        self.out.dst.write_all(std::slice::from_ref(&c)).unwrap();
        self
    }
    fn ws(&mut self) -> &mut Self {
        if self.ws {
            self.byte(b' ');
        }
        self
    }
}

impl<'a, W: Write> Drop for Line<'a, W> {
    fn drop(&mut self) {
        self.out.byte(b'\n');
        if self.to_flush {
            self.out.flush();
        }
    }
}
