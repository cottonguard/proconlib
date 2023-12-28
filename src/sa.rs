use crate::random::*;
use std::time::{Duration, Instant};

/// <https://img.atcoder.jp/intro-heuristics/editorial.pdf>
pub struct Sa {
    time_start: Instant,
    time_end: Instant,
    t0: f64,
    t1: f64,
    temp: f64,
    rng: Xoshiro,
    loops: u64,
}

impl Sa {
    #[inline]
    pub fn run<F: FnMut(&mut Self)>(mut self, mut f: F) {
        self.time_start = Instant::now();
        let amt = self.time_end - self.time_start;
        let mut elapsed = Duration::ZERO;
        let mut block = 32;
        while elapsed < amt {
            let ratio = amt.as_secs_f64() / elapsed.as_secs_f64();
            self.temp = self.t0.powf(1.0 - ratio) * self.t1.powf(ratio);
            for _ in 0..block {
                f(&mut self);
                self.loops += 1;
            }
            let elapsed_next = self.time_start.elapsed();
            if elapsed_next - elapsed < Duration::from_micros(100) {
                block *= 2;
            } else if block > 1 && elapsed_next - elapsed > Duration::from_micros(1000) {
                block /= 2;
            }
            elapsed = elapsed_next;
        }
    }

    #[inline]
    pub fn is_accepted(&mut self, delta: f64) -> bool {
        delta >= 0.0 || self.rng.gen_bool((delta / self.temp).exp())
    }
}
