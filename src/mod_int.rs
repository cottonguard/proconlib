use std::{
    cmp,
    fmt::{self, Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    marker::PhantomData,
    mem,
    ops::*,
    sync::atomic::{AtomicU32, Ordering},
};

pub struct ModInt<M> {
    x: u32,
    marker: PhantomData<*const M>,
}

pub trait Modulo {
    fn modulo() -> u32;

    #[inline]
    fn rem(x: u64) -> u32 {
        (x % Self::modulo() as u64) as u32
    }
}

impl<M> ModInt<M> {
    pub fn new(x: u32) -> Self {
        Self {
            x,
            marker: PhantomData,
        }
    }

    pub fn get(self) -> u32 {
        self.x
    }
}

impl<M: Modulo> ModInt<M> {
    pub fn modulo() -> u32 {
        M::modulo()
    }

    pub fn normalize(self) -> Self {
        Self::new(M::rem(self.x as u64))
    }

    pub fn inv(self) -> Self {
        assert_ne!(self.get(), 0);
        self.pow(M::modulo() - 2)
    }

    pub fn twice(self) -> Self {
        self + self
    }

    pub fn half(self) -> Self {
        if self.x & 1 == 0 {
            Self::new(self.x >> 1)
        } else {
            Self::new((self.x >> 1) + ((Self::modulo() + 1) >> 1))
        }
    }
}

impl<M> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ModInt<M> {}

impl<M: Modulo> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(if self.x != 0 { M::modulo() - self.x } else { 0 })
    }
}

impl<M: Modulo> Neg for &ModInt<M> {
    type Output = ModInt<M>;
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

impl<M: Modulo> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let x = self.x + rhs.x;
        Self::new(if x < M::modulo() { x } else { x - M::modulo() })
    }
}

impl<M: Modulo> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = if self.x >= rhs.x {
            self.x - rhs.x
        } else {
            M::modulo() + self.x - rhs.x
        };
        Self::new(x)
    }
}

impl<M: Modulo> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(M::rem(self.x as u64 * rhs.x as u64))
    }
}

impl<M: Modulo> Div for ModInt<M> {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

macro_rules! biops {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<M: Modulo> $Op<&Self> for ModInt<M> {
            type Output = Self;
            fn $op(self, rhs: &Self) -> Self {
                self.$op(*rhs)
            }
        }

        impl<M: Modulo> $Op<ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: ModInt<M>) -> ModInt<M> {
                (*self).$op(rhs)
            }
        }

        impl<M: Modulo> $Op for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: Self) -> ModInt<M> {
                (*self).$op(*rhs)
            }
        }

        impl<M: Modulo> $OpAssign for ModInt<M> {
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<M: Modulo> $OpAssign<&Self> for ModInt<M> {
            fn $op_assign(&mut self, rhs: &Self) {
                *self = self.$op(rhs);
            }
        }
    };
}

biops!(Add, add, AddAssign, add_assign);
biops!(Sub, sub, SubAssign, sub_assign);
biops!(Mul, mul, MulAssign, mul_assign);
biops!(Div, div, DivAssign, div_assign);

impl<M: Modulo> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sum = iter.fold(0u64, |acc, x| acc + x.get() as u64);
        Self::from(sum)
    }
}

impl<M: Modulo> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt::new(1), |x, y| x * y)
    }
}

macro_rules! fold {
    ($Trait:ident, $f:ident) => {
        impl<'a, M: Modulo + 'a> $Trait<&'a ModInt<M>> for ModInt<M> {
            fn $f<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
                iter.copied().$f()
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
    ($uty:ident, $ity:ident) => {
        impl<M: Modulo> Pow<$uty> for ModInt<M> {
            fn pow(self, mut exp: $uty) -> Self {
                if exp == 0 {
                    return ModInt::new(1);
                }
                let mut res = ModInt::new(1);
                let mut base = self;
                while exp > 1 {
                    if exp & 1 != 0 {
                        res *= base;
                    }
                    exp >>= 1;
                    base *= base;
                }
                base * res
            }
        }

        impl<M: Modulo> Pow<$ity> for ModInt<M> {
            fn pow(self, exp: $ity) -> Self {
                if exp >= 0 {
                    self.pow(exp as $uty)
                } else {
                    self.inv().pow(-exp as $uty)
                }
            }
        }
    };
}

macro_rules! impls {
    ($m:ident, $($uty:ident, $ity:ident),*) => { $($m!($uty, $ity);)* };
}

impls!(pow, usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

impl<M> Default for ModInt<M> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<M> PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl<M> Eq for ModInt<M> {}

impl<M> PartialOrd for ModInt<M> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<M> Ord for ModInt<M> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

impl<M> Hash for ModInt<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state)
    }
}

macro_rules! from_uint {
    ($ty:ident) => {
        impl<M: Modulo> From<$ty> for ModInt<M> {
            fn from(x: $ty) -> Self {
                if mem::size_of::<$ty>() <= 4 {
                    if ($ty::max_value() as u32) < M::modulo() {
                        Self::new(x as u32)
                    } else {
                        Self::new(x as u32).normalize()
                    }
                } else {
                    Self::new((x % M::modulo() as $ty) as u32)
                }
            }
        }
    };
}

macro_rules! impls {
    ($m:ident, $($ty:ident),*) => { $($m!($ty);)* };
}

impls!(from_uint, usize, u8, u16, u32, u64, u128);

macro_rules! from_small_int {
    ($ty:ident) => {
        impl<M: Modulo> From<$ty> for ModInt<M> {
            fn from(x: $ty) -> Self {
                let mut x = x as i32;
                if x >= 0 {
                    Self::from(x as u32)
                } else {
                    while x < 0 {
                        x += M::modulo() as i32;
                    }
                    Self::new(x as u32)
                }
            }
        }
    };
}

impls!(from_small_int, i8, i16, i32);

impl<M> Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.x, f)
    }
}

impl<M> Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.x, f)
    }
}

pub struct VarMod;

static VAR_MOD: AtomicU32 = AtomicU32::new(0);

pub fn set_var_mod(m: u32) {
    VAR_MOD.store(m, Ordering::Relaxed);
}

pub fn var_mint(x: u32) -> ModInt<VarMod> {
    ModInt::new(x)
}

impl Modulo for VarMod {
    fn modulo() -> u32 {
        VAR_MOD.load(Ordering::Relaxed)
    }
}

#[macro_export]
macro_rules! def_mint {
    ($modulo:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
        pub struct MintModulo;
        impl $crate::mod_int::Modulo for MintModulo {
            fn modulo() -> u32 {
                $modulo
            }
        }
        pub type Mint = crate::mod_int::ModInt<MintModulo>;
        pub fn mint(x: u32) -> Mint {
            x.into()
        }
    };
}
