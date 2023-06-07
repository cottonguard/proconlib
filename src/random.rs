mod pcg;
mod xoshiro;

pub use self::pcg::Pcg;
pub use self::xoshiro::*;

pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
}
pub trait Rng: RngCore {
    fn gen<T: Sample>(&mut self) -> T {
        T::sample(self)
    }
    fn range<T: Uniform>(&mut self, l: T, r: T) -> T {
        T::range(self, l, r)
    }
    fn range_inclusive<T: Uniform>(&mut self, l: T, r: T) -> T {
        T::range_inclusive(self, l, r)
    }
    fn gen_bool(&mut self, p: f64) -> bool {
        if p >= 1. {
            return true;
        }
        self.next_u64() < (2.0f64.powi(64) * p) as u64
    }
    fn open01<T: SampleFloat>(&mut self) -> T {
        T::open01(self)
    }
    fn standard_normal<T: SampleFloat>(&mut self) -> T {
        T::standard_normal(self)
    }
    fn normal<T: SampleFloat>(&mut self, mean: T, sd: T) -> T {
        T::normal(self, mean, sd)
    }
    fn exp<T: SampleFloat>(&mut self, lambda: T) -> T {
        T::exp(self, lambda)
    }
    fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            a.swap(self.range_inclusive(0, i), i);
        }
    }
    fn partial_shuffle<'a, T>(&mut self, a: &'a mut [T], n: usize) -> (&'a mut [T], &'a mut [T]) {
        let n = n.min(a.len());
        for i in 0..n {
            a.swap(i, self.range(i, a.len()));
        }
        a.split_at_mut(n)
    }
    fn choose<'a, T>(&mut self, a: &'a [T]) -> &'a T {
        assert!(!a.is_empty());
        &a[self.range(0, a.len())]
    }
    fn choose_mut<'a, T>(&mut self, a: &'a mut [T]) -> &'a mut T {
        assert!(!a.is_empty());
        &mut a[self.range(0, a.len())]
    }
}
impl<T: RngCore> Rng for T {}
pub trait Sample {
    fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self;
}
pub trait Uniform {
    fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
    fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self;
}
pub trait SampleFloat {
    fn open01<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn standard_normal<T: Rng + ?Sized>(rand: &mut T) -> Self;
    fn normal<T: Rng + ?Sized>(rand: &mut T, mean: Self, sd: Self) -> Self;
    fn exp<T: Rng + ?Sized>(rand: &mut T, lambda: Self) -> Self;
}
macro_rules! int_impl {
    ($($type:ident),*) => {$(
        impl Sample for $type {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                if 8 * std::mem::size_of::<Self>() <= 32 {
                    rand.next_u32() as $type
                } else {
                    rand.next_u64() as $type
                }
            }
        }
        impl Uniform for $type {
            fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l < r);
                Self::range_inclusive(rand, l, r - 1)
            }
            fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                if 8 * std::mem::size_of::<Self>() <= 32 {
                    int_impl!(range_inclusive $type, u32, rand, l, r);
                } else {
                    int_impl!(range_inclusive $type, u64, rand, l, r);
                }
            }
        }
    )*};
    (range_inclusive $type:ident, $via:ident, $rand:ident, $l:ident, $r:ident) => {
        let d = ($r - $l) as $via;
        let mask = if d == 0 { 0 } else { !0 >> d.leading_zeros() };
        loop {
            let x = $rand.gen::<$via>() & mask;
            if x <= d {
                return $l + x as $type;
            }
        }
    }
}
int_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
macro_rules! float_impl {
    ($($fty:ident, $uty:ident, $fract:expr, $exp_bias:expr);*) => {$(
        impl Sample for $fty {
            fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let x: $uty = rand.gen();
                let bits = 8 * std::mem::size_of::<$fty>();
                let prec = $fract + 1;
                let scale = 1. / ((1 as $uty) << prec) as $fty;
                scale * (x >> (bits - prec)) as $fty
            }
        }
        impl Uniform for $fty {
            fn range<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                l + Self::sample(rand) / (r - l)
            }
            fn range_inclusive<T: Rng + ?Sized>(rand: &mut T, l: Self, r: Self) -> Self {
                assert!(l <= r);
                Self::range(rand, l, r)
            }
        }
        impl SampleFloat for $fty {
            fn open01<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let x: $uty = rand.gen();
                let bits = 8 * std::mem::size_of::<$fty>();
                let exp = $exp_bias << $fract;
                $fty::from_bits(exp | (x >> (bits - $fract))) - (1. - std::$fty::EPSILON / 2.)
            }
            fn standard_normal<T: Rng + ?Sized>(rand: &mut T) -> Self {
                let r = (-2. * (1. - Self::sample(rand)).ln()).sqrt();
                let c = (2. * std::$fty::consts::PI * Self::sample(rand)).cos();
                r * c
            }
            fn normal<T: Rng + ?Sized>(rand: &mut T, mean: Self, sd: Self) -> Self {
                sd * Self::standard_normal(rand) + mean
            }
            fn exp<T: Rng + ?Sized>(rand: &mut T, lambda: Self) -> Self {
                -1. / lambda * Self::open01(rand).ln()
            }
        }
    )*}
}
float_impl!(f32, u32, 23, 127; f64, u64, 52, 1023);
impl Sample for bool {
    fn sample<T: Rng + ?Sized>(rand: &mut T) -> Self {
        (rand.next_u32() as i32) >= 0
    }
}
pub trait SeedableRng: Sized {
    fn seed_from_u64(seed: u64) -> Self;
    fn from_time() -> Self {
        use std::time::SystemTime;
        let dur = SystemTime::UNIX_EPOCH
            .elapsed()
            .map_or(0, |dur| dur.as_nanos());
        let seed = (dur ^ (dur >> 64)) as u64;
        Self::seed_from_u64(seed)
    }
}
pub struct WeightedIndex<T>(Vec<T>);
impl<I: Weight> WeightedIndex<I> {
    pub fn index<R: RngCore>(&self, rng: &mut R) -> usize {
        let x = rng.range(I::default(), *self.0.last().unwrap());
        let mut l = 0;
        let mut r = self.0.len();
        while r - l > 1 {
            let h = l + (r - l) / 2;
            if x < self.0[h - 1] {
                r = h;
            } else {
                l = h;
            }
        }
        l
    }
    pub fn choose<'a, T, R: RngCore>(&self, rng: &mut R, a: &'a [T]) -> &'a T {
        assert!(self.0.len() <= a.len());
        &a[self.index(rng)]
    }
    pub fn choose_mut<'a, T, R: RngCore>(&self, rng: &mut R, a: &'a mut [T]) -> &'a T {
        assert!(self.0.len() <= a.len());
        &mut a[self.index(rng)]
    }
}
impl<T: Weight> FromIterator<T> for WeightedIndex<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<T>>().into()
    }
}
impl<T: Weight> From<Vec<T>> for WeightedIndex<T> {
    fn from(mut a: Vec<T>) -> Self {
        assert!(!a.is_empty());
        for i in 1..a.len() {
            let (l, r) = a.split_at_mut(i);
            assert!(*r.first_mut().unwrap() >= T::default());
            *r.first_mut().unwrap() += *l.last().unwrap();
        }
        assert!(*a.last().unwrap() != T::default());
        Self(a)
    }
}
pub trait Weight: Copy + PartialEq + Default + PartialOrd + std::ops::AddAssign + Uniform {}
impl<T> Weight for T where T: Copy + PartialEq + Default + PartialOrd + std::ops::AddAssign + Uniform
{}
