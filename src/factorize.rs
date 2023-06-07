pub fn factorize_naive(n: u64) -> FactorizeNaive {
    FactorizeNaive { n, i: 0 }
}
pub struct FactorizeNaive {
    n: u64,
    i: usize,
}
impl Iterator for FactorizeNaive {
    type Item = (u64, u32);
    fn next(&mut self) -> Option<(u64, u32)> {
        loop {
            let d = if self.i <= 2 {
                [2u8, 3, 5][self.i] as u64
            } else {
                let i = self.i as usize - 2;
                i as u64 / 8 * 30 + [1u8, 7, 11, 13, 17, 19, 23, 29][i % 8] as u64
            };
            if d * d > self.n {
                break;
            }
            if self.n % d == 0 {
                let mut s = 0;
                while self.n % d == 0 {
                    self.n /= d;
                    s += 1;
                }
                return Some((d, s));
            }
            self.i += 1;
        }
        if self.n > 1 {
            let res = self.n;
            self.n = 1;
            Some((res, 1))
        } else {
            None
        }
    }
}
