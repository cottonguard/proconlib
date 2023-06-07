pub fn next_n_ones(x: u64) -> u64 {
    let tz = x.trailing_zeros();
    let a = x + (1 << tz);
    a | ((!a & x) >> tz + 1)
}

pub fn n_ones(n: u32, bit_len: u32) -> NOnes {
    if n <= bit_len {
        NOnes {
            cur: Some((1 << n) - 1),
            max: ((1 << n) - 1) << (bit_len - n),
        }
    } else {
        NOnes { cur: None, max: 0 }
    }
}

pub struct NOnes {
    cur: Option<u64>,
    max: u64,
}

impl Iterator for NOnes {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.cur {
            self.cur = if cur < self.max {
                Some(next_n_ones(cur))
            } else {
                None
            };
            Some(cur)
        } else {
            None
        }
    }
}

pub fn cartesian_tree<T: Ord>(a: &[T]) -> Vec<usize> {
    let mut par = vec![!0; a.len()];
    for i in 1..a.len() {
        let mut j = i - 1;
        let mut c = None;
        loop {
            match (par.get(j), a.get(j)) {
                (Some(&p), Some(x)) if &a[i] < x => {
                    c = Some(j);
                    j = p;
                }
                _ => break,
            }
        }
        if let Some(c) = c {
            par[c] = i;
        }
        par[i] = j;
    }
    par
}

pub fn a_to_u64_be(mut s: &[u8]) -> u64 {
    let mut res = 0;
    while s.len() >= 8 {
        let (p, rest) = s.split_at(8);
        s = rest;
        let x = u64::from_be_bytes(p.try_into().unwrap());
        let hi = x & 0x0f000f000f000f00;
        let lo = x & 0x000f000f000f000f;
        let x = 10 * (hi >> 8) + lo;
        let hi = x & 0xffff0000ffff0000;
        let lo = x & 0x0000ffff0000ffff;
        let x = 100 * (hi >> 16) + lo;
        let lo = x & 0x00000000ffffffff;
        let x = 10000 * (x >> 32) + lo;
        res = 100000000 * res + x;
    }
    if s.len() >= 4 {
        let (p, rest) = s.split_at(4);
        s = rest;
        let x = u32::from_be_bytes(p.try_into().unwrap());
        let hi = x & 0x0f000f00;
        let lo = x & 0x000f000f;
        let x = 10 * (hi >> 8) + lo;
        let lo = x & 0x0000ffff;
        let x = 100 * (x >> 16) + lo;
        res = 10000 * res + x as u64;
    }
    for &c in s {
        res = 10 * res + (c & 0xf) as u64;
    }
    res
}

pub fn a_to_u64_le(mut s: &[u8]) -> u64 {
    let mut res = 0;
    while s.len() >= 8 {
        let (p, rest) = s.split_at(8);
        s = rest;
        let x = u64::from_le_bytes(p.try_into().unwrap());
        let hi = x & 0x0f000f000f000f00;
        let lo = x & 0x000f000f000f000f;
        let x = 10 * lo + (hi >> 8);
        let hi = x & 0xffff0000ffff0000;
        let lo = x & 0x0000ffff0000ffff;
        let x = 100 * lo + (hi >> 16);
        let lo = x & 0x00000000ffffffff;
        let x = 10000 * lo + (x >> 32);
        res = 100000000 * res + x;
    }
    if s.len() >= 4 {
        let (p, rest) = s.split_at(4);
        s = rest;
        let x = u32::from_le_bytes(p.try_into().unwrap());
        let hi = x & 0x0f000f00;
        let lo = x & 0x000f000f;
        let x = 10 * lo + (hi >> 8);
        let lo = x & 0x0000ffff;
        let x = 100 * lo + (x >> 16);
        res = 10000 * res + x as u64;
    }
    for &c in s {
        res = 10 * res + (c & 0xf) as u64;
    }
    res
}

pub fn a_to_u64_naive(s: &[u8]) -> u64 {
    let mut res = 0;
    for &c in s {
        res = 10 * res + (c & 0xf) as u64;
    }
    res
}

pub fn gcd_u32(x: u32, y: u32) -> u32 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let tzx = x.trailing_zeros();
    let tzy = y.trailing_zeros();
    let tzg = tzx.min(tzy);
    let mut x = x >> tzx;
    let mut y = y >> tzy;
    while x != y {
        if x > y {
            x -= y;
            x >>= x.trailing_zeros();
        } else {
            y -= x;
            y >>= y.trailing_zeros();
        }
    }
    x << tzg
}

pub fn gcd_i32(x: i32, y: i32) -> i32 {
    gcd_u32(x.abs() as u32, y.abs() as u32) as i32
}

pub fn gcd_u32_euclid(x: u32, y: u32) -> u32 {
    if y == 0 {
        x
    } else {
        gcd_u32_euclid(y, x % y)
    }
}

#[cfg(feature = "never")]
mod random_range {
    use crate::random::*;
    pub struct Range<T> {
        buf: [u64; 2],
        buf_bits: u32,
        lo: T,
        range: T,
        range_bits: u32,
    }
    impl<T> Range<T> {
        pub fn gen<R: Rng>(&mut self, rng: &mut R) -> T {
            loop {
                if self.range_bits > self.buf_bits {
                    self.buf[1] = self.buf[0];
                    self.buf[0] = rng.gen();
                    self.buf_bits += 64;
                }
                let x = (self.buf[0] & ((1 << self.range_bits) - 1)) as T;
                self.buf[0] >>= self.range_bits;
                if self.buf_bits > 64 {
                    self.buf[0] |= self.buf[1] << (64 - self.range_bits);
                    self.buf[1] >>= self.range_bits;
                }
                self.buf_bits -= self.range_bits;
                if x <= self.range {
                    return self.lo + x;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BarrettReduction {
    m: u32,
    s: u64,
    e: u32,
}
impl BarrettReduction {
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
    pub fn store(&self) {
        MUL.store(self.s, Ordering::Relaxed);
        MODULO_SHIFT.store(((self.m as u64) << 32) | self.e as u64, Ordering::Relaxed);
    }
    #[inline]
    pub fn load() -> Self {
        let mul = MUL.load(Ordering::Relaxed);
        let modulo_shift = MODULO_SHIFT.load(Ordering::Relaxed);
        Self {
            s: mul,
            m: (modulo_shift >> 32) as u32,
            e: modulo_shift as u32,
        }
    }
    #[inline]
    pub fn store_thread(self) {
        THREAD.with(|br| br.set(self));
    }
    #[inline]
    pub fn load_thread() -> Self {
        THREAD.with(|br| br.get())
    }
}

use std::sync::atomic::{AtomicU64, Ordering};
static MUL: AtomicU64 = AtomicU64::new(0);
static MODULO_SHIFT: AtomicU64 = AtomicU64::new(0);

use std::cell::Cell;
thread_local! {
    static THREAD: Cell<BarrettReduction> = Cell::new(BarrettReduction { m:  0, s: 0, e: 0 });
}

// x/y
// 1/y ~= m/2^k
// m = floor(2^k/y)
// floor(x/y) ?= floor(mx/2^k)
//

// f in [0, 1)
// floor(xf)

// a <= xf < a + 1
// a/x <= f < (a + 1)/x
// a/x <= f < a/x + 1/x
// 2^ka/x <= f < 2^ka/x + 2^k/x

// r in [0, bm)
// bx <= r < b(x + 1)
// bm < 2^k <= b(m + 1)

// (m + 1)r / 2^k
// (m + 1)bx / 2^k
// (m + 1)(bx - 1) / 2^k

// (m + 1)bx / b(m + 1) <= (m + 1)bx / 2^k < (m + 1)bx / bm
// x <= (m + 1)bx / 2^k <= x * (m + 1) / m
// x <= mbx / 2^k < x + 1

// x >= 1
// (m + 1)b(x - 1) / b(m + 1) <= (m + 1)b(x - 1) / 2^k < (m + 1)b(x - 1) / bm
// x - 1 <= (m + 1)b(x - 1) / 2^k < (x - 1) * (m + 1) / m
// x - 1 <= (m + 1)b(x - 1) / 2^k < (mx - m + x - 1) / m = x - 1 + (x - 1) / m < x + 1
// ((x - 1) / m < 1)

// x - x * m / (m + 1)
// = x / (m + 1)
// < 1 (x < m)

// 1 / y ~= s / 2^k
// s = ceil(2^k / y)
// s / 2^k - 1 / y
// = (s - 2^k / y) / 2^k
// < 1 / 2^k
//
// floor(sx / 2^k)
// sx / 2^k - x / y
// x(s / 2^k - 1 / y)
// < x / 2^k
// x / y <= sx / 2^k < x / y + x / 2^k

// s = ceil(2^k / m) < 2^k / m + 1

// q = floor(sx / 2^k)
// q <= x / m <= sx / 2^k <= q + 1
// x / m - q <= 1 - 1 / m
// sx / 2^k
// < x / m + x / 2^k
// = q + (x - qm) / m + x / 2^k
// < q + 1 - 1 / m + x / 2^k
// x / 2^k <= 1 / m
// 2^64 / 2^k = 2^64-k <= 1 / m

// s < 2^64
// ceil(2^k / m) < 2^k / m + 1 < 2^64
// 2^t < m <= 2^t+1 <-> 2^k-t-1 <= 2^k / m < 2^k-t
// k-t-1 = 64
// k = 65 + t

// m <= 2^k-64 = 2^t+1
//

#[cfg(feature = "nightly")]
pub fn find_ws_simd(s: &[u8]) -> Option<usize> {
    #[inline]
    fn find_ws_naive(s: &[u8]) -> Option<usize> {
        for (i, c) in s.iter().enumerate() {
            if *c <= b' ' {
                return Some(i);
            }
        }
        None
    }

    const ALIGN: usize = std::mem::align_of::<u8x32>();

    use std::simd::*;
    let mut offset = s.as_ptr().align_offset(ALIGN);
    if let Some(pos) = find_ws_naive(&s[..offset.min(s.len())]) {
        return Some(pos);
    }
    if s.len() <= offset {
        return None;
    }
    while offset + ALIGN <= s.len() {
        let ss = &s[offset..offset + ALIGN];
        let ss = u8x32::from_slice(ss);
        let le = ss.simd_le(u8x32::splat(b' '));
        let m = le.to_bitmask();
        if m != 0 {
            return Some(offset + m.leading_zeros() as usize);
        }
        offset += ALIGN;
    }
    find_ws_naive(&s[offset..]).map(|pos| offset + pos)
}
