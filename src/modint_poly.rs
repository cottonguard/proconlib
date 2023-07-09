use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    mem,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign,
        ShlAssign, Sub, SubAssign,
    },
};

use crate::modint2::*;

pub struct Poly<M>(Vec<ModInt<M>>);

impl<M> Poly<M> {
    pub fn zero() -> Self {
        Self(vec![])
    }

    pub fn repeat(a: ModInt<M>, deg: usize) -> Self {
        if a.get() == 0 {
            Self::zero()
        } else {
            Self(vec![a; deg + 1])
        }
    }

    pub fn deg(&self) -> usize {
        self.0.iter().rposition(|a| a.get() != 0).unwrap_or(!0)
    }

    pub fn normalize(&mut self) {
        let len = self
            .0
            .iter()
            .rposition(|a| a.get() != 0)
            .map_or(0, |i| i + 1);
        self.0.truncate(len);
    }

    pub fn as_normalized_slice(&self) -> &[ModInt<M>] {
        let len = self
            .0
            .iter()
            .rposition(|a| a.get() != 0)
            .map_or(0, |i| i + 1);
        &self.0[..len]
    }

    pub fn coef(&self, i: usize) -> ModInt<M> {
        self.0.get(i).copied().unwrap_or(ModInt::ZERO)
    }

    pub fn coef_mut(&mut self, i: usize) -> &mut ModInt<M> {
        if i >= self.0.len() {
            self.0.resize(i + 1, ModInt::ZERO);
        }
        self.0.get_mut(i).unwrap()
    }
}

impl<M: Modulo> Poly<M> {
    pub fn evaluate(&self, x: ModInt<M>) -> ModInt<M> {
        let mut xpow = ModInt::new(1);
        let mut sum = ModInt::new(0);
        for &a in self.iter() {
            sum += a * xpow;
            xpow *= x;
        }
        sum
    }

    fn mul_naive(&mut self, other: &Self) {
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

    pub fn inv(&self, n: usize) -> Self {
        assert!(!self.is_empty());
        assert!(self[0] != ModInt::new(0));
        let mut f = Self(vec![ModInt::new(0); n.next_power_of_two()]);
        let mut res = Self(vec![ModInt::new(0); n]);
        res[0] = self[0].inv();
        let mut tmp = Self(vec![ModInt::new(0); n.next_power_of_two()]);
        let mut m = 1;
        while m < n {
            tmp[..m].copy_from_slice(&res[..m]);
            dft(&mut tmp[..2 * m]);
            f[..(2 * m).min(self.len())].copy_from_slice(&self[..(2 * m).min(self.len())]);
            if self.len() < 2 * m {
                for f in f[self.len()..2 * m].iter_mut() {
                    *f = ModInt::new(0);
                }
            }
            dft(&mut f[..2 * m]);
            for (f, t) in f[..2 * m].iter_mut().zip(tmp[..2 * m].iter()) {
                *f *= t;
            }
            idft(&mut f[..2 * m]);
            for f in f[..m].iter_mut() {
                *f = ModInt::new(0);
            }
            dft(&mut f[..2 * m]);
            for (f, t) in f[..2 * m].iter_mut().zip(tmp[..2 * m].iter()) {
                *f *= t;
            }
            idft(&mut f[..2 * m]);
            for (r, f) in res[m..(2 * m).min(n)]
                .iter_mut()
                .zip(f[m..(2 * m).min(n)].iter())
            {
                *r = -f;
            }
            m *= 2;
        }
        res.0.truncate(n);
        res
    }

    pub fn div_rem(self, other: Self) -> (Self, Self) {
        let q = &self / &other;
        let r = self - other * &q;
        (q, r)
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
        self.as_normalized_slice() == other.as_normalized_slice()
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

impl<M: Modulo> Display for Poly<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_zero = true;
        for (i, a) in self.iter().enumerate() {
            if a.get() == 0 {
                continue;
            }
            let (aa, minus) = if a.get() <= M::modulo() / 20 * 19 {
                (a.get(), false)
            } else {
                (M::modulo() - a.get(), true)
            };
            is_zero = false;
            if i == 0 {
                if minus {
                    write!(f, "-")?;
                }
                write!(f, "{aa}")?;
            } else {
                if minus {
                    write!(f, " - ")?;
                } else {
                    write!(f, " + ")?;
                }
                if aa != 1 {
                    write!(f, "{aa}")?;
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

impl<M: Modulo> MulAssign for Poly<M> {
    fn mul_assign(&mut self, mut rhs: Self) {
        if self.0.is_empty() || rhs.0.is_empty() {
            self.0.clear();
            return;
        }
        if self.0.len().min(rhs.0.len()) <= 16 {
            if self.0.len() < rhs.0.len() {
                mem::swap(self, &mut rhs);
            }
            self.mul_naive(&mut rhs);
            return;
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
    }
}

impl<M: Modulo> DivAssign for Poly<M> {
    fn div_assign(&mut self, mut rhs: Self) {
        self.normalize();
        rhs.normalize();
        if self.len() < rhs.len() {
            self.0.clear();
            return;
        }
        self.reverse();
        rhs.reverse();
        let len = self.len() - rhs.len() + 1;
        let inv = rhs.inv(len);
        *self *= inv;
        self.0.resize(len, ModInt::ZERO);
        self.reverse();
        self.normalize();
    }
}

impl<M: Modulo> RemAssign for Poly<M> {
    fn rem_assign(&mut self, rhs: Self) {
        let q = &*self / &rhs;
        *self -= rhs * q;
    }
}

macro_rules! ops_assign_val_from_ref {
    ($OpAssign: ident, $op_assign: ident) => {
        impl<M: Modulo> $OpAssign for Poly<M> {
            fn $op_assign(&mut self, rhs: Self) {
                self.$op_assign(&rhs);
            }
        }
    };
}

macro_rules! ops_assign_ref_from_val {
    ($OpAssign: ident, $op_assign: ident) => {
        impl<M: Modulo> $OpAssign<&Self> for Poly<M> {
            fn $op_assign(&mut self, rhs: &Self) {
                self.$op_assign(rhs.clone());
            }
        }
    };
}

ops_assign_val_from_ref!(AddAssign, add_assign);
ops_assign_val_from_ref!(SubAssign, sub_assign);
ops_assign_ref_from_val!(MulAssign, mul_assign);
ops_assign_ref_from_val!(DivAssign, div_assign);
ops_assign_ref_from_val!(RemAssign, rem_assign);

macro_rules! ops {
    ($Op: ident, $op: ident, $OpAssign: ident, $op_assign: ident) => {
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
ops!(Mul, mul, MulAssign, mul_assign);
ops!(Div, div, DivAssign, div_assign);
ops!(Rem, rem, RemAssign, rem_assign);

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

impl<M> From<Vec<ModInt<M>>> for Poly<M> {
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

impl<M> FromIterator<ModInt<M>> for Poly<M> {
    fn from_iter<T: IntoIterator<Item = ModInt<M>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
