use std::{
    cell::{Cell, UnsafeCell},
    cmp, fmt,
    hash::Hash,
    iter,
    marker::PhantomData,
    ops,
};

#[inline]
pub fn mint<const M: u32>(value: impl Into<ModInt<ConstMod<M>>>) -> ModInt<ConstMod<M>> {
    value.into()
}

#[inline]
pub fn var_mint(value: impl Into<ModInt<VarMod>>) -> ModInt<VarMod> {
    value.into()
}

pub type Mint<const N: u32> = ModInt<ConstMod<N>>;
pub type VarMint = ModInt<VarMod>;

pub trait Modulo {
    fn modulo() -> u32;
    #[inline]
    fn rem32(x: u32) -> u32 {
        x % Self::modulo()
    }
    #[inline]
    fn rem64(x: u64) -> u32 {
        (x % Self::modulo() as u64) as u32
    }
}

pub struct ConstMod<const M: u32>;
impl<const M: u32> Modulo for ConstMod<M> {
    #[inline]
    fn modulo() -> u32 {
        M
    }
}

#[inline]
pub fn set_var_mod(m: u32) {
    BarrettReduction::new(m).store_thread();
}

pub struct VarMod;

impl Modulo for VarMod {
    #[inline]
    fn modulo() -> u32 {
        BarrettReduction::load_thread().m
    }
    #[inline]
    fn rem32(x: u32) -> u32 {
        Self::rem64(x as u64)
    }
    #[inline]
    fn rem64(x: u64) -> u32 {
        BarrettReduction::load_thread().rem(x)
    }
}

#[derive(Clone, Copy, Debug)]
struct BarrettReduction {
    m: u32,
    e: u32,
    s: u64,
}
impl BarrettReduction {
    #[inline]
    pub fn new(m: u32) -> Self {
        assert_ne!(m, 0);
        assert_ne!(m, 1);
        let e = 31 - (m - 1).leading_zeros();
        Self {
            s: ((1u128 << (64 + e)) / m as u128) as u64 + (!m.is_power_of_two()) as u64,
            m,
            e,
        }
    }
    #[inline]
    pub fn div(&self, x: u64) -> u64 {
        ((self.s as u128 * x as u128) >> 64) as u64 >> self.e
    }
    #[inline]
    pub fn rem(&self, x: u64) -> u32 {
        (x - self.m as u64 * self.div(x)) as u32
    }
    #[inline]
    pub fn store_thread(self) {
        BR.with(|br| br.set(self));
    }
    #[inline]
    pub fn load_thread() -> Self {
        BR.with(|br| br.get())
    }
}

thread_local! {
    static BR: Cell<BarrettReduction> = Cell::new(BarrettReduction { m:  0, s: 0, e: 0 });
}

#[repr(transparent)]
pub struct ModInt<M> {
    value: u32,
    marker: PhantomData<M>,
}

impl<M> ModInt<M> {
    pub const ZERO: Self = Self::unnormalized(0);
    #[inline]
    pub const fn unnormalized(value: u32) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
    #[inline]
    pub const fn get(self) -> u32 {
        self.value
    }
}

impl<M: Modulo> ModInt<M> {
    #[inline]
    pub fn new(value: u32) -> Self {
        Self::unnormalized(M::rem32(value))
    }
    #[inline]
    pub fn normalize(self) -> Self {
        Self::new(self.value)
    }
    #[inline]
    pub fn modulo() -> u32 {
        M::modulo()
    }
    #[inline]
    pub fn set<T: Into<ModInt<M>>>(&mut self, value: T) {
        *self = value.into();
    }
    #[inline]
    pub fn inv(self) -> Self {
        self.pow(M::modulo() - 2)
    }
}

impl<M: Modulo> ops::Neg for ModInt<M> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::unnormalized(if self.value == 0 {
            0
        } else {
            M::modulo() - self.value
        })
    }
}

impl<M: Modulo> ops::Neg for &ModInt<M> {
    type Output = ModInt<M>;
    #[inline]
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

impl<M: Modulo> ops::Add for ModInt<M> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        let sum = self.value + other.value;
        Self::unnormalized(if sum < M::modulo() {
            sum
        } else {
            sum - M::modulo()
        })
    }
}

impl<M: Modulo> ops::Sub for ModInt<M> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let (diff, of) = self.value.overflowing_sub(other.value);
        Self::unnormalized(if of {
            diff.wrapping_add(M::modulo())
        } else {
            diff
        })
    }
}

impl<M: Modulo> ops::Mul for ModInt<M> {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self::unnormalized(M::rem64(self.value as u64 * other.value as u64))
    }
}

impl<M: Modulo> ops::Div for ModInt<M> {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

macro_rules! binop {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<M: Modulo> ops::$Op<&ModInt<M>> for ModInt<M> {
            type Output = Self;
            #[inline]
            fn $op(self, other: &ModInt<M>) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<M: Modulo> ops::$Op<ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            #[inline]
            fn $op(self, other: ModInt<M>) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<M: Modulo> ops::$Op for &ModInt<M> {
            type Output = ModInt<M>;
            #[inline]
            fn $op(self, other: Self) -> Self::Output {
                (*self).$op(*other)
            }
        }
        impl<M: Modulo> ops::$OpAssign for ModInt<M> {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = <Self as ops::$Op>::$op(*self, rhs);
            }
        }
        impl<M: Modulo> ops::$OpAssign<&ModInt<M>> for ModInt<M> {
            #[inline]
            fn $op_assign(&mut self, rhs: &ModInt<M>) {
                *self = <Self as ops::$Op>::$op(*self, *rhs);
            }
        }
    };
}

binop!(Add, add, AddAssign, add_assign);
binop!(Sub, sub, SubAssign, sub_assign);
binop!(Mul, mul, MulAssign, mul_assign);
binop!(Div, div, DivAssign, div_assign);

impl<M: Modulo> iter::Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sum = iter.fold(0u64, |acc, x| acc + x.get() as u64);
        Self::from(sum)
    }
}

impl<M: Modulo> iter::Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |x, y| x * y)
    }
}

macro_rules! fold {
    ($Trait:ident, $f:ident) => {
        impl<'a, M: Modulo + 'a> iter::$Trait<&'a ModInt<M>> for ModInt<M> {
            fn $f<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
                <Self as iter::$Trait>::$f(iter.copied())
            }
        }
    };
}

fold!(Sum, sum);
fold!(Product, product);

pub trait Pow<Exp> {
    fn pow(self, exp: Exp) -> Self;
}

macro_rules! pow {
    ($Uint:ident, $Int:ident) => {
        impl<M: Modulo> Pow<$Uint> for ModInt<M> {
            #[inline]
            fn pow(self, mut exp: $Uint) -> Self {
                let mut res = Self::unnormalized(1);
                if exp == 0 {
                    return res;
                }
                let mut base = self;
                while exp > 1 {
                    if exp & 1 == 1 {
                        res *= base;
                    }
                    base *= base;
                    exp >>= 1;
                }
                res * base
            }
        }
        impl<M: Modulo> Pow<$Int> for ModInt<M> {
            #[inline]
            fn pow(self, exp: $Int) -> Self {
                let p = self.pow(exp.abs() as $Uint);
                if exp >= 0 {
                    p
                } else {
                    p.inv()
                }
            }
        }
    };
}

pow!(usize, isize);
pow!(u8, i8);
pow!(u16, i16);
pow!(u32, i32);
pow!(u64, i64);
pow!(u128, i128);

impl<M> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ModInt<M> {}

impl<M> Default for ModInt<M> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<M> PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<M> Eq for ModInt<M> {}

impl<M> PartialOrd for ModInt<M> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M> Ord for ModInt<M> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<M> Hash for ModInt<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<M> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<M> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl<M: Modulo> From<u32> for ModInt<M> {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl<M: Modulo> From<u64> for ModInt<M> {
    fn from(value: u64) -> Self {
        Self::unnormalized(M::rem64(value))
    }
}

impl<M: Modulo> From<u128> for ModInt<M> {
    fn from(value: u128) -> Self {
        Self::unnormalized((value % M::modulo() as u128) as u32)
    }
}

macro_rules! from_small_uint {
    ($ty:ident) => {
        impl<M: Modulo> From<$ty> for ModInt<M> {
            fn from(value: $ty) -> Self {
                Self::new(value as u32)
            }
        }
    };
}

from_small_uint!(u8);
from_small_uint!(u16);

impl<M: Modulo> From<usize> for ModInt<M> {
    fn from(value: usize) -> Self {
        if cfg!(target_pointer_width = "64") {
            ModInt::from(value as u64)
        } else {
            ModInt::from(value as u32)
        }
    }
}

macro_rules! from_signed {
    ($Uint:ident, $Int:ident) => {
        impl<M: Modulo> From<$Int> for ModInt<M> {
            fn from(value: $Int) -> Self {
                let abs = ModInt::from(value.abs() as $Uint);
                if value >= 0 {
                    abs
                } else {
                    -abs
                }
            }
        }
    };
}

from_signed!(usize, isize);
from_signed!(u8, i8);
from_signed!(u16, i16);
from_signed!(u32, i32);
from_signed!(u64, i64);
from_signed!(u128, i128);

pub struct Fact<M>(UnsafeCell<FactInner<M>>);

impl<M: Modulo> Fact<M> {
    #[inline]
    pub fn new() -> Self {
        Self(UnsafeCell::new(FactInner {
            fact: vec![],
            fact_inv: vec![],
        }))
    }
    #[inline]
    pub fn fact(&self, n: usize) -> ModInt<M> {
        unsafe { (*self.0.get()).fact(n) }
    }
    #[inline]
    pub fn fact_inv(&self, n: usize) -> ModInt<M> {
        unsafe { (*self.0.get()).fact_inv(n) }
    }
    #[inline]
    pub fn binom(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k) * self.fact_inv(k)
        } else {
            ModInt::unnormalized(0)
        }
    }
    #[inline]
    pub fn perm(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k)
        } else {
            ModInt::unnormalized(0)
        }
    }
    #[inline]
    pub fn catalan(&self, n: usize) -> ModInt<M> {
        self.fact(2 * n) * self.fact_inv(n + 1) * self.fact_inv(n)
    }
}

impl<M: Modulo> Default for Fact<M> {
    fn default() -> Self {
        Self::new()
    }
}

struct FactInner<M> {
    fact: Vec<ModInt<M>>,
    fact_inv: Vec<ModInt<M>>,
}

impl<M: Modulo> FactInner<M> {
    #[inline]
    fn fact(&mut self, n: usize) -> ModInt<M> {
        if let Some(&val) = self.fact.get(n) {
            val
        } else {
            self.grow_fact(n)
        }
    }
    fn grow_fact(&mut self, n: usize) -> ModInt<M> {
        self.fact.reserve(n + 1 - self.fact.len());
        if self.fact.is_empty() {
            self.fact.push(ModInt::new(1));
        }
        unsafe {
            let ptr = self.fact.as_mut_ptr();
            let mut val = *ptr.add(self.fact.len() - 1);
            for i in self.fact.len()..=n {
                val *= ModInt::new(i as u32);
                *ptr.add(i) = val;
            }
            self.fact.set_len(n + 1);
            val
        }
    }
    #[inline]
    fn fact_inv(&mut self, n: usize) -> ModInt<M> {
        if let Some(&val) = self.fact_inv.get(n) {
            val
        } else {
            self.grow_fact_inv(n)
        }
    }
    fn grow_fact_inv(&mut self, n: usize) -> ModInt<M> {
        self.fact(n);
        self.fact_inv.reserve(n + 1 - self.fact_inv.len());
        let orig_len = self.fact_inv.len();
        unsafe {
            let res = self.fact[n].inv();
            let mut val = res;
            let ptr = self.fact_inv.as_mut_ptr();
            *ptr.add(n) = val;
            for i in (orig_len..n).rev() {
                val *= ModInt::new(i as u32 + 1);
                *ptr.add(i) = val;
            }
            self.fact_inv.set_len(n + 1);
            res
        }
    }
}

/// experimental
/// <https://en.wikipedia.org/wiki/Thue%27s_lemma>
pub fn fraction<M: Modulo>(x: ModInt<M>) -> (i32, i32) {
    use std::mem::swap;
    if x.get() == 0 {
        return (0, 1);
    }
    let mut x = x.get() as i32;
    let mut y = M::modulo() as i32;
    let mut a = (1, 0);
    let mut b = (0, 1);
    while y != 0 {
        let q = x / y;
        x %= y;
        swap(&mut x, &mut y);
        a.0 -= q * b.0;
        a.1 -= q * b.1;
        swap(&mut a, &mut b);
        if a.0 != 0
            && (x as i64 * x as i64) <= M::modulo() as i64
            && (a.0 as i64 * a.0 as i64).abs() <= M::modulo() as i64
        {
            break;
        }
    }
    if a.0 >= 0 {
        (x, a.0)
    } else {
        (-x, -a.0)
    }
}
