use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn cast<U: From<T>>(self) -> Vec2<U> {
        Vec2::new(self.x.into(), self.y.into())
    }
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Vec2<U> {
        Vec2::new(f(self.x), f(self.y))
    }
    pub fn any<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
        f(self.x) || f(self.y)
    }
    pub fn all<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
        f(self.x) && f(self.y)
    }
}

impl<T: Scalar> Vec2<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }
    pub fn splat(value: T) -> Self {
        Self::new(value, value)
    }
    pub fn yx(self) -> Self {
        Self::new(self.y, self.x)
    }
    pub fn pend(self) -> Self {
        Self::new(-self.y, self.x)
    }
    pub fn norm(self) -> T {
        self.x * self.x + self.y * self.y
    }
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(self, other: Self) -> T {
        self.x * other.y - other.x * self.y
    }
    pub fn lerp(self, other: Self, t: T) -> Self {
        self + (other - self) * t
    }
    pub fn abs(self) -> Self {
        self.map(|e| e.abs())
    }
    pub fn project_onto(self, onto: Self) -> Self {
        onto * (self.dot(onto) / onto.norm())
    }
    pub fn project_onto_normalized(self, onto: Self) -> Self {
        onto * self.dot(onto)
    }
    pub fn abs_diff_eq(self, other: Self, eps: T) -> bool {
        let eqx = (self[0] - other[0]).abs() <= eps;
        let eqy = (self[1] - other[1]).abs() <= eps;
        eqx && eqy
    }
}

impl<T: Float> Vec2<T> {
    pub fn unit_angle(angle: T) -> Self {
        Self::new(angle.cos(), angle.sin())
    }
    pub fn length(self) -> T {
        self.x.hypot(self.y)
    }
    pub fn angle(self) -> T {
        self.y.atan2(self.x)
    }
    pub fn normalize(self) -> Self {
        let len = self.length();
        Self::new(self.x / len, self.y / len)
    }
    pub fn rotate_angle(self, angle: T) -> Self {
        self.rotate(Self::unit_angle(angle))
    }
    pub fn rotate(self, angle_vec: Self) -> Self {
        Self::new(
            angle_vec.x * self.x - angle_vec.y * self.y,
            angle_vec.y * self.x + angle_vec.x * self.y,
        )
    }
}

impl<T> Deref for Vec2<T> {
    type Target = [T; 2];
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const _) }
    }
}

impl<T> DerefMut for Vec2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }
}

impl<T: Scalar> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Scalar> Neg for &Vec2<T> {
    type Output = Vec2<T>;
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

macro_rules! binop {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<T: Scalar> $Op for Vec2<T> {
            type Output = Self;
            fn $op(self, other: Self) -> Self::Output {
                Self::new(self.x.$op(other.x), self.y.$op(other.y))
            }
        }
        impl<T: Scalar> $Op<&Vec2<T>> for Vec2<T> {
            type Output = Self;
            fn $op(self, other: &Vec2<T>) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<T: Scalar> $Op<Vec2<T>> for &Vec2<T> {
            type Output = Vec2<T>;
            fn $op(self, other: Vec2<T>) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<T: Scalar> $Op for &Vec2<T> {
            type Output = Vec2<T>;
            fn $op(self, other: Self) -> Self::Output {
                (*self).$op(*other)
            }
        }
        impl<T: Scalar> $OpAssign for Vec2<T> {
            fn $op_assign(&mut self, other: Self) {
                *self = <Self as $Op>::$op(*self, other);
            }
        }
        impl<T: Scalar> $OpAssign<&Vec2<T>> for Vec2<T> {
            fn $op_assign(&mut self, other: &Vec2<T>) {
                *self = <Self as $Op>::$op(*self, *other);
            }
        }
    };
}

binop!(Add, add, AddAssign, add_assign);
binop!(Sub, sub, SubAssign, sub_assign);
binop!(Mul, mul, MulAssign, mul_assign);
binop!(Div, div, DivAssign, div_assign);

macro_rules! scalar_rhs {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<T: Scalar> $Op<T> for Vec2<T> {
            type Output = Self;
            fn $op(self, other: T) -> Self::Output {
                Self::new(self.x.$op(other), self.y.$op(other))
            }
        }
        impl<T: Scalar> $Op<&T> for Vec2<T> {
            type Output = Self;
            fn $op(self, other: &T) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<T: Scalar> $Op<T> for &Vec2<T> {
            type Output = Vec2<T>;
            fn $op(self, other: T) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<T: Scalar> $Op<&T> for &Vec2<T> {
            type Output = Vec2<T>;
            fn $op(self, other: &T) -> Self::Output {
                (*self).$op(*other)
            }
        }
    };
}

scalar_rhs!(Mul, mul, MulAssign, mul_assign);
scalar_rhs!(Div, div, DivAssign, div_assign);

impl<T> AsRef<Vec2<T>> for [T; 2] {
    fn as_ref(&self) -> &Vec2<T> {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<T> AsMut<Vec2<T>> for [T; 2] {
    fn as_mut(&mut self) -> &mut Vec2<T> {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        let [x, y] = value;
        Self::new(x, y)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(value: Vec2<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(value: Vec2<T>) -> Self {
        (value.x, value.y)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub struct Mat2<T>([Vec2<T>; 2]);

pub fn mat2<T>(a0: Vec2<T>, a1: Vec2<T>) -> Mat2<T> {
    Mat2::from_rows(a0, a1)
}

impl<T> Mat2<T> {
    pub fn from_elems(a00: T, a01: T, a10: T, a11: T) -> Self {
        Self([vec2(a00, a01), vec2(a10, a11)])
    }
    pub fn from_rows(a0: Vec2<T>, a1: Vec2<T>) -> Self {
        Self([a0, a1])
    }
    pub fn as_flat_array(&self) -> &[T; 4] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Scalar> Mat2<T> {
    pub fn zero() -> Self {
        Self::from_elems(T::zero(), T::zero(), T::zero(), T::zero())
    }
    pub fn identity() -> Self {
        Self::from_elems(T::one(), T::zero(), T::zero(), T::one())
    }
    pub fn diag(d: Vec2<T>) -> Self {
        Self::from_elems(d[0], T::zero(), T::zero(), d[1])
    }
    pub fn transpose(mut self) -> Self {
        let t = self[0][1];
        self[0][1] = self[1][0];
        self[1][0] = t;
        self
    }
    pub fn det(self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    pub fn inv(self) -> Self {
        let det = self.det();
        Self([
            vec2(self[1][1] / det, -self[0][1] / det),
            vec2(-self[1][0] / det, self[0][0] / det),
        ])
    }
    pub fn pow(self, exp: u32) -> Self {
        pow(self, exp, Self::identity())
    }
    pub fn abs_diff_eq(self, other: Self, eps: T) -> bool {
        self[0].abs_diff_eq(other[0], eps) && self[1].abs_diff_eq(other[1], eps)
    }
}

impl<T: Float> Mat2<T> {
    pub fn rotation(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([vec2(c, -s), vec2(s, c)])
    }
    pub fn scale_rotation(scale: Vec2<T>, angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([
            vec2(scale.x * c, scale.x * -s),
            vec2(scale.y * s, scale.y * c),
        ])
    }
}

impl<T: Scalar> Add for Mat2<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            for j in 0..2 {
                self[i][j] = self[i][j] + other[i][j];
            }
        }
        self
    }
}

impl<T: Scalar> Sub for Mat2<T> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            for j in 0..2 {
                self[i][j] = self[i][j] - other[i][j];
            }
        }
        self
    }
}

impl<T: Scalar> Mul for Mat2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let other = other.transpose();
        Self([
            vec2(self[0].dot(other[0]), self[0].dot(other[1])),
            vec2(self[1].dot(other[0]), self[1].dot(other[1])),
        ])
    }
}

impl<T: Scalar> Mul<Vec2<T>> for Mat2<T> {
    type Output = Vec2<T>;
    fn mul(self, other: Vec2<T>) -> Self::Output {
        vec2(self[0].dot(other), self[1].dot(other))
    }
}

impl<T: Scalar> Mul<T> for Mat2<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self::Output {
        Self([self[0] * s, self[1] * s])
    }
}

impl<T: Scalar> Div<T> for Mat2<T> {
    type Output = Self;
    fn div(self, s: T) -> Self::Output {
        Self([self[0] / s, self[1] / s])
    }
}

impl<T> Deref for Mat2<T> {
    type Target = [Vec2<T>; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Mat2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Affine2<T> {
    pub mat: Mat2<T>,
    pub translation: Vec2<T>,
}

impl<T> Affine2<T> {
    pub fn new(mat: Mat2<T>, translation: Vec2<T>) -> Self {
        Self { mat, translation }
    }
}

impl<T: Scalar> Affine2<T> {
    pub fn identity() -> Self {
        Self::new(Mat2::identity(), Vec2::zero())
    }
    pub fn traslation(translation: Vec2<T>) -> Self {
        Self::new(Mat2::identity(), translation)
    }
    pub fn inv(self) -> Self {
        let mat = self.mat.inv();
        Self::new(mat, mat * -self.translation)
    }
    pub fn pow(self, exp: u32) -> Self {
        pow(self, exp, Self::identity())
    }
    pub fn abs_diff_eq(self, other: Self, eps: T) -> bool {
        self.mat.abs_diff_eq(other.mat, eps) && self.translation.abs_diff_eq(other.translation, eps)
    }
}

impl<T: Scalar> Mul<Vec2<T>> for Affine2<T> {
    type Output = Vec2<T>;
    fn mul(self, v: Vec2<T>) -> Self::Output {
        self.mat * v + self.translation
    }
}

impl<T: Scalar> Mul for Affine2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            mat: self.mat * other.mat,
            translation: self.translation + self.mat * other.translation,
        }
    }
}

impl<T: Scalar> From<Mat2<T>> for Affine2<T> {
    fn from(mat: Mat2<T>) -> Self {
        Self {
            mat,
            translation: Vec2::zero(),
        }
    }
}

fn pow<T: Copy + Mul<Output = T>>(x: T, mut k: u32, id: T) -> T {
    if k == 0 {
        return id;
    }
    let mut y = id;
    let mut x = x;
    while k > 1 {
        if k & 1 == 1 {
            y = y * x;
        }
        x = x * x;
        k >>= 1;
    }
    y * x
}

pub trait Scalar:
    Copy
    + Default
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn abs(self) -> Self;
}

pub trait Float: Scalar {
    fn abs_diff_eq(self, other: Self, eps: Self) -> bool;
    fn hypot(self, other: Self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

macro_rules! scalar {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn zero() -> Self {
                0 as Self
            }
            fn one() -> Self {
                1 as Self
            }
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

macro_rules! float {
    ($ty:ident) => {
        scalar!($ty);
        impl Float for $ty {
            fn abs_diff_eq(self, other: Self, eps: Self) -> bool {
                (self - other).abs() <= eps
            }
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
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
