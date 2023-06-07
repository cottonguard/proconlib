use crate::input::{Input, Parser};

pub struct Binary;

macro_rules! binary_uint {
    ($ty:ty) => {
        impl Parser<$ty> for Binary {
            #[inline]
            fn parse<I: Input + ?Sized>(&mut self, s: &mut I) -> $ty {
                s.bytes().iter().fold(0, |x, d| (x << 1) | (d & 1) as $ty)
            }
        }
    };
}

#[allow(unused)]
fn f(s: &[u8]) -> u64 {
    let mut x = u64::from_be_bytes(s[..8].try_into().unwrap());
    x = ((x & 0x0100010001000100) >> 7) | (x & 0x0001000100010001);
    x = ((x & 0x0003000000030000) >> 14) | (x & 0x0000000300000003);
    x = ((x & 0x0000000700000000) >> 28) | (x & 0x0000000000000007);
    x
}

macro_rules! impls {
    ($m:ident, $($ty:ty),*) => { $($m!($ty);)* };
}

impls!(binary_uint, usize, u8, u16, u32, u64, u128);
