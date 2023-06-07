use super::mod_int::*;
use std::cell::RefCell;

pub struct FixedFact<M> {
    f: Vec<ModInt<M>>,
    finv: Vec<ModInt<M>>,
}
impl<M: Modulo> FixedFact<M> {
    pub fn new(n: usize) -> Self {
        let mut res = Self {
            f: Vec::with_capacity(n + 1),
            finv: Vec::with_capacity(n + 1),
        };
        res.f.push(ModInt::new(1));
        res.finv.push(ModInt::new(1));
        res.grow(n);
        res
    }
    pub fn grow(&mut self, n: usize) -> &mut Self {
        let orig_n = self.f.len() - 1;
        if n <= orig_n {
            return self;
        }
        self.f.resize(n + 1, ModInt::new(0));
        for i in orig_n + 1..=n {
            self.f[i] = ModInt::new(i as u32) * self.f[i - 1];
        }
        self.finv.resize(n + 1, ModInt::new(0));
        self.finv[n] = self.f[n].inv();
        for i in (orig_n + 1..n).rev() {
            self.finv[i] = self.finv[i + 1] * ModInt::new(i as u32 + 1);
        }
        self
    }
    pub fn fact(&self, x: usize) -> ModInt<M> {
        self.f[x]
    }
    pub fn fact_inv(&self, x: usize) -> ModInt<M> {
        self.finv[x]
    }
    pub fn binom(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k) * self.fact_inv(k)
        } else {
            ModInt::new(0)
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.fact(n) * self.fact_inv(n - k)
        } else {
            ModInt::new(0)
        }
    }
}

pub struct Fact<M>(RefCell<FixedFact<M>>);
impl<M: Modulo> Fact<M> {
    pub fn new() -> Self {
        Self(RefCell::new(FixedFact::new(0)))
    }
    pub fn fact(&self, x: usize) -> ModInt<M> {
        self.0.borrow_mut().grow(x).fact(x)
    }
    pub fn fact_inv(&self, x: usize) -> ModInt<M> {
        self.0.borrow_mut().grow(x).fact_inv(x)
    }
    pub fn binom(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.0.borrow_mut().grow(n).binom(n, k)
        } else {
            ModInt::new(0)
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> ModInt<M> {
        if n >= k {
            self.0.borrow_mut().grow(n).perm(n, k)
        } else {
            ModInt::new(0)
        }
    }
}

impl<M: Modulo> Default for Fact<M> {
    fn default() -> Self {
        Self::new()
    }
}
