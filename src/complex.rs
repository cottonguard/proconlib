use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type Ci32 = Complex<i32>;
pub type Ci64 = Complex<i64>;

pub type C32 = Complex<f32>;
pub type C64 = Complex<f64>;

pub fn c<T>(re: T, im: T) -> Complex<T> {
    Complex(re, im)
}

pub fn re<T: Scalar>(re: T) -> Complex<T> {
    Complex(re, T::zero())
}

pub fn im<T: Scalar>(im: T) -> Complex<T> {
    Complex(T::zero(), im)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Complex<T>(pub T, pub T);

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self(re, im)
    }
}

impl<T: Scalar> Complex<T> {
    pub fn norm(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn dot(self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }
    pub fn cross(self, other: Self) -> T {
        self.0 * other.1 - other.0 * self.1
    }
    pub fn conj(self) -> Self {
        Self(self.0, -self.1)
    }
    pub fn powi(self, exp: i32) -> Self {
        let (mut p, mut exp) = if exp >= 0 {
            (self, exp)
        } else {
            (self.inv(), -exp)
        };
        let mut z = Self(T::one(), T::zero());
        while exp > 1 {
            if exp & 1 == 1 {
                z *= p;
            }
            p *= p;
            exp /= 2;
        }
        z * p
    }
    pub fn inv(self) -> Self {
        re(T::one()) / self
    }
}

impl<T: Float> Complex<T> {
    pub fn abs(self) -> T {
        self.0.hypot(self.1)
    }
    pub fn polar(r: T, arg: T) -> Self {
        Self(r * arg.cos(), r * arg.sin())
    }
    pub fn arg(self) -> T {
        self.1.atan2(self.0)
    }
    pub fn normalize(self) -> Self {
        self / self.abs()
    }
    pub fn exp(self) -> Self {
        Self::polar(self.0.exp(), self.1)
    }
    pub fn powf(self, exp: T) -> Self {
        Self::polar(self.abs().powf(exp), self.arg() * exp)
    }
}

impl<T: Scalar> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<T: Scalar> Add for Complex<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Scalar> Add<T> for Complex<T> {
    type Output = Self;
    fn add(self, re: T) -> Self::Output {
        Self(self.0 + re, self.1)
    }
}

impl<T: Scalar> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Scalar> Sub<T> for Complex<T> {
    type Output = Self;
    fn sub(self, re: T) -> Self::Output {
        Self(self.0 - re, self.1)
    }
}

impl<T: Scalar> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}

impl<T: Scalar> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self(self.0 * other, self.1 * other)
    }
}

impl<T: Scalar> Div for Complex<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let norm = other.norm();
        let re = self.0 * other.0 + self.1 * other.1;
        let im = self.1 * other.0 - self.0 * other.1;
        Self(re / norm, im / norm)
    }
}

impl<T: Scalar> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex(value.0, value.1)
    }
}

impl<T> From<Complex<T>> for (T, T) {
    fn from(value: Complex<T>) -> Self {
        (value.0, value.1)
    }
}

impl<T> From<[T; 2]> for Complex<T> {
    fn from(value: [T; 2]) -> Self {
        let [re, im] = value;
        Complex(re, im)
    }
}

impl<T> From<Complex<T>> for [T; 2] {
    fn from(value: Complex<T>) -> Self {
        [value.0, value.1]
    }
}

impl<T: Scalar> From<T> for Complex<T> {
    fn from(value: T) -> Self {
        Complex(value, T::default())
    }
}

pub trait Scalar:
    Copy
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait Float: Scalar {
    fn exp(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn powf(self, exp: Self) -> Self;
}

macro_rules! scalar {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn zero() -> Self {
                0 as $ty
            }
            fn one() -> Self {
                1 as $ty
            }
        }
        impl Add<Complex<$ty>> for $ty {
            type Output = Complex<$ty>;
            fn add(self, c: Complex<$ty>) -> Self::Output {
                Complex(self + c.0, c.1)
            }
        }
        impl Sub<Complex<$ty>> for $ty {
            type Output = Complex<$ty>;
            fn sub(self, c: Complex<$ty>) -> Self::Output {
                Complex(self - c.0, -c.1)
            }
        }
        impl Mul<Complex<$ty>> for $ty {
            type Output = Complex<$ty>;
            fn mul(self, other: Complex<$ty>) -> Self::Output {
                Complex(self * other.0, self * other.1)
            }
        }
        impl Div<Complex<$ty>> for $ty {
            type Output = Complex<$ty>;
            fn div(self, other: Complex<$ty>) -> Self::Output {
                let norm = other.norm();
                Complex(self * other.0 / norm, -self * other.1 / norm)
            }
        }
    };
}

macro_rules! float {
    ($ty:ident) => {
        scalar!($ty);
        impl Float for $ty {
            fn exp(self) -> Self {
                self.exp()
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }
            fn powf(self, exp: Self) -> Self {
                self.powf(exp)
            }
        }
    };
}

scalar!(isize);
scalar!(i8);
scalar!(i16);
scalar!(i32);
scalar!(i64);
scalar!(i128);

float!(f32);
float!(f64);

macro_rules! binop {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<T: Scalar> $Op<&Complex<T>> for Complex<T> {
            type Output = Self;
            fn $op(self, rhs: &Complex<T>) -> Self {
                <Self as $Op>::$op(self, *rhs)
            }
        }
        impl<T: Scalar> $Op<Complex<T>> for &Complex<T> {
            type Output = Complex<T>;
            fn $op(self, rhs: Complex<T>) -> Self::Output {
                <Complex<T> as $Op>::$op(*self, rhs)
            }
        }
        impl<T: Scalar> $Op for &Complex<T> {
            type Output = Complex<T>;
            fn $op(self, rhs: Self) -> Self::Output {
                <Complex<T> as $Op>::$op(*self, *rhs)
            }
        }
        impl<T: Scalar> $OpAssign for Complex<T> {
            fn $op_assign(&mut self, rhs: Self) {
                *self = <Self as $Op>::$op(*self, rhs);
            }
        }
        impl<T: Scalar> $OpAssign<&Complex<T>> for Complex<T> {
            fn $op_assign(&mut self, rhs: &Complex<T>) {
                *self = <Self as $Op>::$op(*self, *rhs);
            }
        }
        impl<T: Scalar> $OpAssign<T> for Complex<T> {
            fn $op_assign(&mut self, rhs: T) {
                *self = <Self as $Op<T>>::$op(*self, rhs);
            }
        }
        impl<T: Scalar> $OpAssign<&T> for Complex<T> {
            fn $op_assign(&mut self, rhs: &T) {
                *self = <Self as $Op<T>>::$op(*self, *rhs);
            }
        }
    };
}

binop!(Add, add, AddAssign, add_assign);
binop!(Sub, sub, SubAssign, sub_assign);
binop!(Mul, mul, MulAssign, mul_assign);
binop!(Div, div, DivAssign, div_assign);

impl<T: Scalar> Sum for Complex<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(T::zero().into(), |acc, z| acc + z)
    }
}

impl<T: Scalar> Product for Complex<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(T::one().into(), |acc, z| acc * z)
    }
}

macro_rules! acc {
    ($Acc:ident, $acc:ident) => {
        impl<'a, T: Scalar + 'a> $Acc<&'a Complex<T>> for Complex<T> {
            fn $acc<I: Iterator<Item = &'a Complex<T>>>(iter: I) -> Complex<T> {
                iter.copied().$acc()
            }
        }
    };
}

acc!(Sum, sum);
acc!(Product, product);
