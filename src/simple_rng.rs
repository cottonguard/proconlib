use std::{
    cell::RefCell,
    hash::{BuildHasher, Hasher, RandomState},
};

pub struct Rng(pub [u64; 2]);

/// <https://prng.di.unimi.it/xoroshiro128starstar.c>
impl Rng {
    pub fn new() -> Self {
        THREAD_RNG.with(|t| Self::seed_from_u64(t.borrow_mut().u64()))
    }
    pub fn seed_from_u64(mut seed: u64) -> Self {
        /// <https://prng.di.unimi.it/splitmix64.c>
        fn splitmix(x: &mut u64) -> u64 {
            *x = x.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *x;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            return z ^ (z >> 31);
        }
        Self([splitmix(&mut seed), splitmix(&mut seed)])
    }
    pub fn u64(&mut self) -> u64 {
        let [s0, mut s1] = self.0;
        s1 ^= s0;
        self.0 = [s0.rotate_left(24) ^ s1 ^ (s1 << 16), s1.rotate_left(37)];
        s0.wrapping_mul(5).rotate_left(7).wrapping_mul(5)
    }
    pub fn u32(&mut self) -> u32 {
        (self.u64() >> 32) as u32
    }
    pub fn f64(&mut self) -> f64 {
        let n = self.u64();
        f64::from_bits(0x3ff << 52 | n >> 12) - 1.0
    }
}

thread_local! {
    static THREAD_RNG: RefCell<Rng> = RefCell::new(Rng::seed_from_u64(RandomState::new().build_hasher().finish()));
}
