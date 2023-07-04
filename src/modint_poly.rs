use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    mem,
    ops::{
        Add, AddAssign, Deref, DerefMut, DivAssign, Mul, MulAssign, Neg, ShlAssign, Sub, SubAssign,
    },
};

use crate::modint2::*;

pub struct Poly<M>(Vec<ModInt<M>>);

impl<M> Poly<M> {
    pub fn zero() -> Self {
        Self(vec![])
    }
}

impl<M: Modulo> Poly<M> {
    pub fn normalize(&mut self) {
        if let Some(i) = self.0.iter().rposition(|a| a.get() != 0) {
            self.0.truncate(i + 1);
        }
    }

    pub fn evaluate(&self, x: ModInt<M>) -> ModInt<M> {
        let mut xpow = ModInt::new(1);
        let mut sum = ModInt::new(0);
        for &a in self.iter() {
            sum += a * xpow;
            xpow *= x;
        }
        sum
    }

    fn mul_naive(&mut self, other: &mut Self) {
        if self.0.len() < other.0.len() {
            mem::swap(self, other);
        }
        let orig_len = self.0.len();
        self.0
            .resize(self.0.len() + other.0.len() - 1, ModInt::new(0));
        for i in (0..orig_len).rev() {
            let x = self.0[i];
            self.0[i] = ModInt::new(0);
            for (j, y) in other.0.iter().enumerate() {
                self.0[i + j] += x * y;
            }
        }
    }
}

impl<M> Deref for Poly<M> {
    type Target = [ModInt<M>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M> DerefMut for Poly<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M> PartialEq for Poly<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<M> Eq for Poly<M> {}

impl<M> Default for Poly<M> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<M> Clone for Poly<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M> Display for Poly<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_zero = true;
        for (i, a) in self.iter().enumerate() {
            if a.get() == 0 {
                continue;
            }
            is_zero = false;
            if i == 0 {
                write!(f, "{a}")?;
            } else {
                write!(f, " + ")?;
                if a.get() != 1 {
                    write!(f, "{a}")?;
                }
                write!(f, "x")?;
                if i > 1 {
                    write!(f, "^{i}")?;
                }
            }
        }
        if is_zero {
            write!(f, "0")?;
        }
        Ok(())
    }
}

impl<M> Debug for Poly<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Poly").field(&self.0).finish()
    }
}

impl<M> Hash for Poly<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<M: Modulo> ShlAssign<usize> for Poly<M> {
    fn shl_assign(&mut self, rhs: usize) {
        let orig_len = self.len();
        self.0.resize(orig_len + rhs, ModInt::new(0));
        self.0.copy_within(..orig_len, rhs);
        for a in &mut self[..rhs] {
            a.set(0);
        }
    }
}

impl<M: Modulo> Neg for Poly<M> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for a in &mut self.0 {
            *a = -*a;
        }
        self
    }
}

impl<M: Modulo> AddAssign<&Self> for Poly<M> {
    fn add_assign(&mut self, rhs: &Self) {
        if rhs.0.len() > self.0.len() {
            self.0.resize(rhs.0.len(), ModInt::new(0));
        }
        for (x, y) in self.0.iter_mut().zip(rhs.0.iter()) {
            *x += y;
        }
        self.normalize();
    }
}

impl<M: Modulo> SubAssign<&Self> for Poly<M> {
    fn sub_assign(&mut self, rhs: &Self) {
        if rhs.0.len() > self.0.len() {
            self.0.resize(rhs.0.len(), ModInt::new(0));
        }
        for (x, y) in self.0.iter_mut().zip(rhs.0.iter()) {
            *x -= y;
        }
        self.normalize();
    }
}

macro_rules! ops {
    ($Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl<M: Modulo> $OpAssign for Poly<M> {
            fn $op_assign(&mut self, rhs: Self) {
                self.$op_assign(&rhs);
            }
        }
        impl<M: Modulo> $Op<&Self> for Poly<M> {
            type Output = Self;
            fn $op(mut self, rhs: &Self) -> Self::Output {
                self.$op_assign(rhs);
                self
            }
        }
        impl<M: Modulo> $Op for Poly<M> {
            type Output = Self;
            fn $op(self, rhs: Self) -> Self::Output {
                self.$op(&rhs)
            }
        }
        impl<M: Modulo> $Op for &Poly<M> {
            type Output = Poly<M>;
            fn $op(self, rhs: Self) -> Self::Output {
                self.clone().$op(rhs)
            }
        }
        impl<M: Modulo> $Op<Poly<M>> for &Poly<M> {
            type Output = Poly<M>;
            fn $op(self, rhs: Poly<M>) -> Self::Output {
                self.clone().$op(rhs)
            }
        }
    };
}

ops!(Add, add, AddAssign, add_assign);
ops!(Sub, sub, SubAssign, sub_assign);

impl<M: Modulo> Mul for Poly<M> {
    type Output = Self;
    fn mul(mut self, mut rhs: Self) -> Self::Output {
        if self.0.is_empty() || rhs.0.is_empty() {
            return Self::zero();
        }
        if self.0.len().min(rhs.0.len()) <= 16 {
            self.mul_naive(&mut rhs);
            return self;
        }
        let len = (self.0.len() + rhs.0.len() - 1).next_power_of_two();
        self.0.resize(len, ModInt::new(0));
        dft(&mut self.0);
        rhs.0.resize(len, ModInt::new(0));
        dft(&mut rhs.0);
        for (x, y) in self.0.iter_mut().zip(rhs.0.iter()) {
            *x *= y;
        }
        idft(&mut self.0);
        self.normalize();
        self
    }
}

macro_rules! mul {
    ($Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
        impl<M: Modulo> $OpAssign for Poly<M> {
            fn $op_assign(&mut self, rhs: Self) {
                *self = (mem::take(self)).$op(rhs);
            }
        }
        impl<M: Modulo> $OpAssign<&Self> for Poly<M> {
            fn $op_assign(&mut self, rhs: &Self) {
                *self = (mem::take(self)).$op(rhs);
            }
        }
        impl<M: Modulo> $Op for &Poly<M> {
            type Output = Poly<M>;
            fn $op(self, rhs: Self) -> Self::Output {
                self.clone().$op(rhs.clone())
            }
        }
        impl<M: Modulo> $Op<Poly<M>> for &Poly<M> {
            type Output = Poly<M>;
            fn $op(self, rhs: Poly<M>) -> Self::Output {
                self.clone().$op(rhs)
            }
        }
        impl<M: Modulo> $Op<&Self> for Poly<M> {
            type Output = Self;
            fn $op(self, rhs: &Self) -> Self::Output {
                self.$op(rhs.clone())
            }
        }
    };
}

mul!(Mul, mul, MulAssign, mul_assign);

pub fn dft<M: Modulo>(a: &mut [ModInt<M>]) {
    dft_impl(a, false);
}

pub fn idft<M: Modulo>(a: &mut [ModInt<M>]) {
    dft_impl(a, true);
    let ninv = ModInt::new(a.len() as u32).inv();
    for a in a {
        *a *= ninv;
    }
}

#[inline]
fn dft_impl<M: Modulo>(a: &mut [ModInt<M>], inv: bool) {
    assert!(a.len().is_power_of_two());
    if a.len() <= 2 {
        if a.len() == 2 {
            let x = a[0];
            let y = a[1];
            a[0] = x + y;
            a[1] = x - y;
        }
        return;
    }
    let sh = a.len().leading_zeros() + 1;
    for i in 0..a.len() {
        let j = i.reverse_bits() >> sh;
        if j < i {
            a.swap(i, j);
        }
    }
    // w[i] == 1^2^-i
    let e = a.len().trailing_zeros() as usize;
    let mut w = [ModInt::<M>::new(0); 32];
    let pr = ModInt::new(primitive_root::<M>());
    let pr = if inv { pr.inv() } else { pr };
    w[e] = pr.pow((M::modulo() - 1) >> e);
    for i in (0..e).rev() {
        w[i] = w[i + 1] * w[i + 1];
    }
    for r in (4..=a.len()).step_by(4) {
        let l = r - 4;
        let s0 = a[l] + a[l + 1];
        let s1 = a[l + 2] + a[l + 3];
        let d0 = a[l] - a[l + 1];
        let d1w = w[2] * (a[l + 2] - a[l + 3]);
        a[l] = s0 + s1;
        a[l + 1] = d0 + d1w;
        a[l + 2] = s0 - s1;
        a[l + 3] = d0 - d1w;
        for e in 3..=r.trailing_zeros() {
            let l = r - (1 << e);
            let m = r - (1 << (e - 1));

            let x = a[l];
            let y = a[m];
            a[l] = x + y;
            a[m] = x - y;

            let wb = w[e as usize];
            let mut wi = wb;
            for j in m + 1..r {
                let i = j - (1 << (e - 1));
                let x = a[i].get() as u64;
                let y = wi.get() as u64 * a[j].get() as u64;
                a[i] = ModInt::from(x + y);
                a[j] = ModInt::from(x as i64 - y as i64);
                wi *= wb;
            }
        }
    }
}

impl<M: Modulo> AddAssign<ModInt<M>> for Poly<M> {
    fn add_assign(&mut self, v: ModInt<M>) {
        if self.is_empty() {
            self.0 = vec![v];
        } else {
            self[0] += v;
            if self.len() == 1 {
                self.normalize();
            }
        }
    }
}

impl<M: Modulo> SubAssign<ModInt<M>> for Poly<M> {
    fn sub_assign(&mut self, v: ModInt<M>) {
        if self.is_empty() {
            self.0 = vec![-v];
        } else {
            self[0] -= v;
            if self.len() == 1 {
                self.normalize();
            }
        }
    }
}

impl<M: Modulo> MulAssign<ModInt<M>> for Poly<M> {
    fn mul_assign(&mut self, v: ModInt<M>) {
        if v.get() == 0 {
            *self = Self::zero();
        }
        for a in &mut self.0 {
            *a *= v;
        }
    }
}

impl<M: Modulo> DivAssign<ModInt<M>> for Poly<M> {
    fn div_assign(&mut self, v: ModInt<M>) {
        for a in &mut self.0 {
            *a /= v;
        }
    }
}

fn primitive_root<M: Modulo>() -> u32 {
    match M::modulo() {
        998244353 => 3,
        _ => todo!(),
    }
}

impl<M: Modulo> From<Vec<ModInt<M>>> for Poly<M> {
    fn from(value: Vec<ModInt<M>>) -> Self {
        let mut res = Self(value);
        res.normalize();
        res
    }
}

macro_rules! from_int32_vec {
    ($ty: ident) => {
        impl<M: Modulo> From<Vec<$ty>> for Poly<M> {
            fn from(mut a: Vec<$ty>) -> Self {
                for x in &mut a {
                    *x = ModInt::<M>::from(*x).get() as $ty;
                }
                Self::from(unsafe { mem::transmute::<_, Vec<ModInt<M>>>(a) })
            }
        }
    };
}

from_int32_vec!(u32);
from_int32_vec!(i32);
