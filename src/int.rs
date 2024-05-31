#![allow(unstable_name_collisions)]

use std::ops::*;

pub trait UInt:
    Sized
    + Copy
    + Eq
    + Ord
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    fn div_floor(self, other: Self) -> Self;
    fn div_ceil(self, other: Self) -> Self;

    fn abs(self) -> Self;

    fn gcd(self, other: Self) -> Self;

    fn lcm(self, other: Self) -> Self {
        self / self.gcd(other) * other
    }

    fn isqrt(self) -> Self;

    fn quotients(self) -> Qutients<Self> {
        Qutients::new(self)
    }
}

pub trait Int: UInt + Neg<Output = Self> {
    // type Unsigined: UInt + From<Self> + Into<Self>;

    fn ext_gcd(mut self, mut other: Self) -> (Self, Self, Self) {
        use std::mem::swap;
        let mut a = (Self::ONE, Self::ZERO);
        let mut b = (Self::ZERO, Self::ONE);
        while other != Self::ZERO {
            let d = self / other;
            self = self % other;
            swap(&mut self, &mut other);
            a.0 = a.0 - d * b.0;
            a.1 = a.1 - d * b.1;
            swap(&mut a, &mut b);
        }
        if self >= Self::ZERO {
            (self, a.0, a.1)
        } else {
            (-self, -a.0, -a.1)
        }
    }
}

pub fn crt<T: Int>(r1: T, m1: T, r2: T, m2: T) -> Option<(T, T)> {
    if m1 > m2 {
        return crt(r2, m2, r1, m1);
    }
    let (g, v1, _) = m1.ext_gcd(m2);
    if m1 == g {
        return if r1 == r2 % m1 { Some((r1, m1)) } else { None };
    }
    if (r2 - r1) % g != T::ZERO {
        return None;
    }
    // m1q + r1 = r2 (mod u2)
    // q = v1(r2 - r1) / g (mod u2)
    let u2 = m2 / g;
    let q = (r2 - r1) / g * v1 % u2;
    let q = if q >= T::ZERO { q } else { u2 + q };
    let x = m1 * q + r1;
    Some((x, m1 * u2))
}

macro_rules! common_fns {
    ($ty:ty) => {
        const ZERO: Self = 0;
        const ONE: Self = 0;
        fn gcd(self, other: Self) -> Self {
            let x = self.abs();
            let y = other.abs();
            if x == 0 {
                return y;
            }
            if y == 0 {
                return x;
            }
            let tzx = x.trailing_zeros();
            let tzy = y.trailing_zeros();
            let tzg = tzx.min(tzy);
            let mut x = x >> tzx;
            let mut y = y >> tzy;
            while x != y {
                if x < y {
                    std::mem::swap(&mut x, &mut y);
                }
                x -= y;
                x >>= x.trailing_zeros();
            }
            x << tzg
        }
    };
}

macro_rules! uint {
    ($ty:ty) => {
        impl UInt for $ty {
            common_fns!($ty);
            fn abs(self) -> Self {
                self
            }
            fn div_floor(self, other: Self) -> Self {
                self / other
            }
            fn div_ceil(self, other: Self) -> Self {
                let q = self / other;
                let r = self % other;
                q + (r != 0) as Self
            }
            fn isqrt(self) -> Self {
                let mut res = 0;
                let mut diff = self;
                for exp in (0..=(Self::BITS - self.leading_zeros()) / 2).rev() {
                    if let Some(new_diff) = diff.checked_sub((res << (exp + 1)) + (1 << (2 * exp)))
                    {
                        diff = new_diff;
                        res += 1 << exp;
                    }
                }
                res
            }
        }
    };
}

macro_rules! int {
    ($ty:ty, $uty:ty) => {
        impl UInt for $ty {
            common_fns!($ty);
            fn abs(self) -> Self {
                self.abs()
            }
            fn div_floor(self, other: Self) -> Self {
                let q = self / other;
                let r = self % other;
                q - ((other ^ r < 0) as Self & (r != 0) as Self)
            }
            fn div_ceil(self, other: Self) -> Self {
                let q = self / other;
                let r = self % other;
                q + ((other ^ r > 0) as Self & (r != 0) as Self)
            }
            fn isqrt(self) -> Self {
                assert!(self >= 0, "the value is negative ({self})");
                (self as $uty).isqrt() as Self
            }
        }
        impl Int for $ty {
            // type Unsigined =
        }
    };
}

uint!(usize);
uint!(u8);
uint!(u16);
uint!(u32);
uint!(u64);
uint!(u128);
int!(isize, usize);
int!(i8, u8);
int!(i16, u16);
int!(i32, u32);
int!(i64, u64);
int!(i128, u128);

pub fn gcd<T: UInt>(x: T, y: T) -> T {
    x.gcd(y)
}

pub fn ext_gcd<T: Int>(x: T, y: T) -> (T, T, T) {
    x.ext_gcd(y)
}

pub fn lcm<T: UInt>(x: T, y: T) -> T {
    x.lcm(y)
}

#[derive(Debug, Clone)]
pub struct Qutients<T> {
    n: T,
    ndivh: T,
    h: T,
}

impl<T: UInt> Qutients<T> {
    #[inline]
    fn new(n: T) -> Self {
        if n < T::ZERO {
            unimplemented!()
        }
        let mut h = UInt::isqrt(n);
        if n >= h * (h + T::ONE) {
            h = h + T::ONE;
        }
        Qutients { n, h, ndivh: n / h }
    }
    #[inline]
    pub fn get(&self, i: T) -> Option<T> {
        if i < self.ndivh {
            Some(i)
        } else if self.h > i - self.ndivh {
            let j = i - self.ndivh;
            Some(self.n / (self.h - j))
        } else {
            None
        }
    }

    #[inline]
    pub fn count(&self) -> T {
        self.h + self.ndivh
    }

    pub fn iter(&self) -> QutientsIter<T> {
        QutientsIter {
            i: T::ZERO,
            q: self.clone(),
        }
    }
}

pub struct QutientsIter<T> {
    i: T,
    q: Qutients<T>,
}

impl<T: UInt + TryInto<usize>> Iterator for QutientsIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let q = self.q.get(self.i)?;
        self.i = self.i + T::ONE;
        Some(q)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.q.count() - self.i).try_into().unwrap_or(usize::MAX);
        (len, Some(len))
    }
}
