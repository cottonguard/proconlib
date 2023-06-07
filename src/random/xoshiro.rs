use super::{RngCore, SeedableRng};

/// <https://xoshiro.di.unimi.it/xoshiro256plusplus.c>
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Xoshiro([u64; 4]);

impl Xoshiro {
    pub fn from_seed(seed: [u64; 4]) -> Self {
        Self(seed)
    }
}
impl SeedableRng for Xoshiro {
    fn seed_from_u64(seed: u64) -> Self {
        let mut sm = SplitMix::seed_from_u64(seed);
        let mut state = [0; 4];
        for s in &mut state {
            *s = sm.next_u64();
        }
        Self::from_seed(state)
    }
}
impl RngCore for Xoshiro {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    fn next_u64(&mut self) -> u64 {
        let res = (self.0[0].wrapping_add(self.0[3]))
            .rotate_left(23)
            .wrapping_add(self.0[0]);
        let t = self.0[1] << 17;
        self.0[2] ^= self.0[0];
        self.0[3] ^= self.0[1];
        self.0[1] ^= self.0[2];
        self.0[0] ^= self.0[3];
        self.0[2] ^= t;
        self.0[3] = self.0[3].rotate_left(45);
        res
    }
}

/// <https://xoshiro.di.unimi.it/splitmix64.c>
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SplitMix(u64);

impl SeedableRng for SplitMix {
    fn seed_from_u64(seed: u64) -> Self {
        Self(seed)
    }
}
impl RngCore for SplitMix {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
