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
    fn zero() -> Self;
    fn one() -> Self;

    fn div_floor(self, other: Self) -> Self;
    fn div_ceil(self, other: Self) -> Self;

    fn abs(self) -> Self;

    fn gcd(self, other: Self) -> Self;

    fn lcm(self, other: Self) -> Self {
        self / self.gcd(other) * other
    }
}

pub trait Int: UInt + Neg<Output = Self> {
    // type Unsigined: UInt + From<Self> + Into<Self>;

    fn ext_gcd(mut self, mut other: Self) -> (Self, Self, Self) {
        use std::mem::swap;
        let mut a = (Self::one(), Self::zero());
        let mut b = (Self::zero(), Self::one());
        while other != Self::zero() {
            let d = self / other;
            self = self % other;
            swap(&mut self, &mut other);
            a.0 = a.0 - d * b.0;
            a.1 = a.1 - d * b.1;
            swap(&mut a, &mut b);
        }
        if self >= Self::zero() {
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
    if (r2 - r1) % g != T::zero() {
        return None;
    }
    // m1q + r1 = r2 (mod u2)
    // q = v1(r2 - r1) / g (mod u2)
    let u2 = m2 / g;
    let q = (r2 - r1) / g * v1 % u2;
    let q = if q >= T::zero() { q } else { u2 + q };
    let x = m1 * q + r1;
    Some((x, m1 * u2))
}

macro_rules! common_fns {
    ($ty:ty) => {
        fn zero() -> Self {
            0
        }
        fn one() -> Self {
            1
        }
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
                if x > y {
                    x -= y;
                    x >>= x.trailing_zeros();
                } else {
                    y -= x;
                    y >>= y.trailing_zeros();
                }
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
        }
    };
}

macro_rules! int {
    ($ty:ty) => {
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
        }
        impl Int for $ty {
            // type Unsigined =
        }
    };
}

macro_rules! each {
    ($macro:ident, $($ty:ty),*) => {
        $($macro!($ty);)*
    };
}

each!(uint, usize, u8, u16, u32, u64, u128);
each!(int, isize, i8, i16, i32, i64, i128);

pub fn gcd<T: UInt>(x: T, y: T) -> T {
    x.gcd(y)
}
