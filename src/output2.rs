use std::{io::Write, mem::MaybeUninit, slice};

pub struct Output<W: ?Sized> {
    start: bool,
    dst: W,
}

impl<W: Write> Output<W> {
    pub fn new(dst: W) -> Self {
        Self { dst, start: true }
    }

    pub fn ln(&mut self) -> &mut Self {
        self.dst.write_all(b"\n").unwrap();
        self.start = true;
        self
    }

    pub fn put_bytes(&mut self, s: &[u8]) -> &mut Self {
        if !self.start {
            self.dst.write_all(b" ").unwrap();
        }
        self.start = false;
        self.dst.write_all(s).unwrap();
        self
    }

    pub fn put<T: Put>(&mut self, val: T) -> &mut Self {
        val.put(self);
        self
    }

    pub fn putln<T: Put>(&mut self, val: T) -> &mut Self {
        val.put(self);
        self.ln();
        self
    }
}

pub trait Put {
    fn put<W: Write>(&self, out: &mut Output<W>);
}

impl<T: Put> Put for &T {
    fn put<W: Write>(&self, out: &mut Output<W>) {
        (**self).put(out)
    }
}

impl Put for str {
    fn put<W: Write>(&self, out: &mut Output<W>) {
        out.put_bytes(self.as_bytes());
    }
}

impl Put for u32 {
    fn put<W: Write>(&self, out: &mut Output<W>) {
        const BUF_LEN: usize = 16;

        let mut n = *self;

        if n == 0 {
            out.put_bytes(b"0");
            return;
        }

        let mut buf: [MaybeUninit<u8>; BUF_LEN] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut i = BUF_LEN;
        while n != 0 {
            i -= 1;
            buf[BUF_LEN - i].write(b'0' + (n % 10) as u8);
            n /= 10;
        }
        let res = unsafe { slice::from_raw_parts(buf.as_ptr().add(i) as *const u8, BUF_LEN - i) };
        out.put_bytes(res);
    }
}

impl Put for f32 {
    fn put<W: Write>(&self, out: &mut Output<W>) {
        /*
        2^k n = 10^t m
        2^k 10^t n = 2^k+1 10^t-1 5n
        2^k 10^t n = 2^k-1 10^t 2n = 2^k-1 10^t+1 n/5
         */
        let x = *self;

        if x.is_infinite() {
            if x > 0.0 {
                out.put_bytes(b"inf");
            } else {
                out.put_bytes(b"-inf");
            }
            return;
        }

        if x.is_nan() {
            out.put_bytes(b"NaN");
            return;
        }

        let bits = x.to_bits();

        let exp_biassed = (bits << 1 >> 24) as i32;

        if exp_biassed == 0 {
            out.put_bytes(b"0");
            return;
        }

        let minus = (bits >> 31 & 1) == 1;
        let mut frac = bits & ((1 << 23) - 1) | (1 << 23);
        let tz = frac.trailing_zeros();
        frac >>= tz;
        let mut exp = exp_biassed - 127 - 23 + tz as i32;

        let mut exp10 = exp;
        while exp < 0 {
            if frac.overflowing_mul(5).1 {
                frac = (frac + 1) / 2;
                exp += 1;
            } else {
                frac *= 5;
                exp += 1;
                exp10 -= 1;
            }
        }

        if exp > 0 {
            frac <<= exp;
        }

        let mut buf = [0; 64];
        let mut i = 0;
        while frac != 0 || exp10 <= 0 {
            i += 1;
            buf[buf.len() - i] = b'0' + (frac % 10) as u8;
            frac /= 10;
            exp10 += 1;
            if exp10 == 0 {
                i += 1;
                buf[buf.len() - i] = b'.';
            }
        }

        if minus {
            i += 1;
            buf[buf.len() - i] = b'-';
        }

        out.put_bytes(&buf[buf.len() - i..]);
    }
}

/*

floor(5^a / 2^b)

*/
