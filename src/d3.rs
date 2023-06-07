use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn cast<U: From<T>>(self) -> Vec3<U> {
        Vec3::new(self.x.into(), self.y.into(), self.z.into())
    }
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Vec3<U> {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
    pub fn any<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
        f(self.x) || f(self.y) || f(self.z)
    }
    pub fn all<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
        f(self.x) && f(self.y) && f(self.z)
    }
}

impl<T: Scalar> Vec3<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
    pub fn splat(value: T) -> Self {
        Self::new(value, value, value)
    }
    /*
    pub fn yx(self) -> Self {
        Self::new(self.y, self.x)
    }
     */
    pub fn norm(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Self) -> Self {
        Vec3::new(
            self.y * other.z - other.y * self.z,
            self.z * other.x - other.z * self.x,
            self.x * other.y - other.x * self.y,
        )
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
        let eqz = (self[2] - other[2]).abs() <= eps;
        eqx && eqy && eqz
    }
}

impl<T: Float> Vec3<T> {
    /*
    pub fn unit_angle(angle: T) -> Self {
        Self::new(angle.cos(), angle.sin())
    }
     */
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }
    /*
    pub fn angle(self) -> T {
        self.y.atan2(self.x)
    }
     */
    pub fn normalize(self) -> Self {
        let len = self.length();
        Self::new(self.x / len, self.y / len, self.z / len)
    }
    /*
    pub fn rotate_angle(self, angle: T) -> Self {
        self.rotate(Self::unit_angle(angle))
    }
    pub fn rotate(self, angle_vec: Self) -> Self {
        Self::new(
            angle_vec.x * self.x - angle_vec.y * self.y,
            angle_vec.y * self.x + angle_vec.x * self.y,
        )
    }
     */
}

impl<T> Deref for Vec3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const _) }
    }
}

impl<T> DerefMut for Vec3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }
}

impl<T: Scalar> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Scalar> Neg for &Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

macro_rules! binop {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<T: Scalar> $Op for Vec3<T> {
            type Output = Self;
            fn $op(self, other: Self) -> Self::Output {
                Self::new(
                    self.x.$op(other.x),
                    self.y.$op(other.y),
                    self.z.$op(other.z),
                )
            }
        }
        impl<T: Scalar> $Op<&Vec3<T>> for Vec3<T> {
            type Output = Self;
            fn $op(self, other: &Vec3<T>) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<T: Scalar> $Op<Vec3<T>> for &Vec3<T> {
            type Output = Vec3<T>;
            fn $op(self, other: Vec3<T>) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<T: Scalar> $Op for &Vec3<T> {
            type Output = Vec3<T>;
            fn $op(self, other: Self) -> Self::Output {
                (*self).$op(*other)
            }
        }
        impl<T: Scalar> $OpAssign for Vec3<T> {
            fn $op_assign(&mut self, other: Self) {
                *self = <Self as $Op>::$op(*self, other);
            }
        }
        impl<T: Scalar> $OpAssign<&Vec3<T>> for Vec3<T> {
            fn $op_assign(&mut self, other: &Vec3<T>) {
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
        impl<T: Scalar> $Op<T> for Vec3<T> {
            type Output = Self;
            fn $op(self, other: T) -> Self::Output {
                Self::new(self.x.$op(other), self.y.$op(other), self.z.$op(other))
            }
        }
        impl<T: Scalar> $Op<&T> for Vec3<T> {
            type Output = Self;
            fn $op(self, other: &T) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<T: Scalar> $Op<T> for &Vec3<T> {
            type Output = Vec3<T>;
            fn $op(self, other: T) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<T: Scalar> $Op<&T> for &Vec3<T> {
            type Output = Vec3<T>;
            fn $op(self, other: &T) -> Self::Output {
                (*self).$op(*other)
            }
        }
    };
}

scalar_rhs!(Mul, mul, MulAssign, mul_assign);
scalar_rhs!(Div, div, DivAssign, div_assign);

impl<T> AsRef<Vec3<T>> for [T; 2] {
    fn as_ref(&self) -> &Vec3<T> {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<T> AsMut<Vec3<T>> for [T; 2] {
    fn as_mut(&mut self) -> &mut Vec3<T> {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        let [x, y, z] = value;
        Self::new(x, y, z)
    }
}

impl<T> From<Vec3<T>> for [T; 2] {
    fn from(value: Vec3<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T> From<Vec3<T>> for (T, T) {
    fn from(value: Vec3<T>) -> Self {
        (value.x, value.y)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub struct Mat3<T>([Vec3<T>; 3]);

pub fn mat3<T>(a0: Vec3<T>, a1: Vec3<T>, a2: Vec3<T>) -> Mat3<T> {
    Mat3::from_rows(a0, a1, a2)
}

impl<T> Mat3<T> {
    /*
    pub fn from_elems(a00: T, a01: T, a10: T, a11: T) -> Self {
        Self([vec3(a00, a01), vec3(a10, a11),])
    }
     */
    pub fn from_rows(a0: Vec3<T>, a1: Vec3<T>, a2: Vec3<T>) -> Self {
        Self([a0, a1, a2])
    }
    pub fn as_flat_array(&self) -> &[T; 9] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 9] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Scalar> Mat3<T> {
    pub fn zero() -> Self {
        Self::from_rows(Vec3::zero(), Vec3::zero(), Vec3::zero())
    }
    pub fn identity() -> Self {
        Self::from_rows(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z())
    }
    pub fn diag(d: Vec3<T>) -> Self {
        let mut mat = Self::zero();
        for i in 0..3 {
            mat[i][i] = d[i];
        }
        mat
    }
    pub fn transpose(mut self) -> Self {
        for i in 0..2 {
            for j in i + 1..3 {
                let t = self[i][j];
                self[i][j] = self[j][i];
                self[j][i] = t;
            }
        }
        self
    }
    pub fn det(self) -> T {
        self[0].cross(self[1]).dot(self[2])
    }
    pub fn inv(self) -> Self {
        let t0 = self[1].cross(self[2]);
        let t1 = self[2].cross(self[0]);
        let t2 = self[0].cross(self[1]);
        let det = self[0].dot(t0);
        Self::from_rows(t0, t1, t2).transpose() / det
    }
    pub fn pow(self, exp: u32) -> Self {
        pow(self, exp, Self::identity())
    }
    pub fn abs_diff_eq(self, other: Self, eps: T) -> bool {
        self[0].abs_diff_eq(other[0], eps)
            && self[1].abs_diff_eq(other[1], eps)
            && self[2].abs_diff_eq(other[2], eps)
    }
}

impl<T: Float> Mat3<T> {
    /*
    pub fn rotation(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([vec3(c, -s), vec3(s, c)])
    }
    pub fn scale_rotation(scale: Vec3<T>, angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([
            vec3(scale.x * c, scale.x * -s),
            vec3(scale.y * s, scale.y * c),
        ])
    }
     */
}

impl<T: Scalar> Add for Mat3<T> {
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

impl<T: Scalar> Sub for Mat3<T> {
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

impl<T: Scalar> Mul for Mat3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let other = other.transpose();
        Self([
            vec3(
                self[0].dot(other[0]),
                self[0].dot(other[1]),
                self[0].dot(other[2]),
            ),
            vec3(
                self[1].dot(other[0]),
                self[1].dot(other[1]),
                self[1].dot(other[2]),
            ),
            vec3(
                self[2].dot(other[0]),
                self[2].dot(other[1]),
                self[2].dot(other[2]),
            ),
        ])
    }
}

impl<T: Scalar> Mul<Vec3<T>> for Mat3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        vec3(self[0].dot(other), self[1].dot(other), self[2].dot(other))
    }
}

impl<T: Scalar> Mul<T> for Mat3<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self::Output {
        Self([self[0] * s, self[1] * s, self[2] * s])
    }
}

impl<T: Scalar> Div<T> for Mat3<T> {
    type Output = Self;
    fn div(self, s: T) -> Self::Output {
        Self([self[0] / s, self[1] / s, self[2] / s])
    }
}

impl<T> Deref for Mat3<T> {
    type Target = [Vec3<T>; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Mat3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Affine3<T> {
    pub mat: Mat3<T>,
    pub translation: Vec3<T>,
}

impl<T> Affine3<T> {
    pub fn new(mat: Mat3<T>, translation: Vec3<T>) -> Self {
        Self { mat, translation }
    }
}

impl<T: Scalar> Affine3<T> {
    pub fn identity() -> Self {
        Self::new(Mat3::identity(), Vec3::zero())
    }
    pub fn traslation(translation: Vec3<T>) -> Self {
        Self::new(Mat3::identity(), translation)
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

impl<T: Scalar> Mul<Vec3<T>> for Affine3<T> {
    type Output = Vec3<T>;
    fn mul(self, v: Vec3<T>) -> Self::Output {
        self.mat * v + self.translation
    }
}

impl<T: Scalar> Mul for Affine3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            mat: self.mat * other.mat,
            translation: self.translation + self.mat * other.translation,
        }
    }
}

impl<T: Scalar> From<Mat3<T>> for Affine3<T> {
    fn from(mat: Mat3<T>) -> Self {
        Self {
            mat,
            translation: Vec3::zero(),
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
    fn sqrt(self) -> Self;
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
            fn sqrt(self) -> Self {
                self.sqrt()
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
