use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rational {
    num: i64,
    den: u64,
}

impl Rational {
    pub fn new(num: i64, den: u64) -> Self {
        Self::unnormalized(num, den).normalize()
    }
    pub fn int(num: i64) -> Self {
        Self::unnormalized(num, 1)
    }
    pub fn unnormalized(num: i64, den: u64) -> Self {
        assert_ne!(den, 0);
        Self { num, den }
    }
    pub unsafe fn new_unchecked(num: i64, den: u64) -> Self {
        Self { num, den }
    }
    pub fn normalize(mut self) -> Self {
        if self.num == 0 {
            return Self::unnormalized(0, 1);
        }
        let g = unsafe { gcd_nonzero(self.num.abs() as u64, self.den) };
        self.num /= g as i64;
        self.den /= g;
        self
    }
    pub fn num(self) -> i64 {
        self.num
    }
    pub fn den(self) -> u64 {
        self.den
    }
    pub fn abs(mut self) -> Self {
        self.num = self.num.abs();
        self
    }
    pub fn inv(self) -> Self {
        assert_ne!(self.num, 0);
        if self.num > 0 {
            Self::unnormalized(self.den as i64, self.num as u64)
        } else {
            Self::unnormalized(-(self.den as i64), (-self.num) as u64)
        }
    }
    pub fn trunc(self) -> i64 {
        self.invariant();
        self.num / self.den as i64
    }
    pub fn fract(mut self) -> Self {
        self.invariant();
        self.num %= self.den as i64;
        self
    }
    pub fn int_fract(self) -> (i64, Self) {
        (self.trunc(), self.fract())
    }
    pub fn is_int(self) -> bool {
        self.den == 1
    }
    pub fn approx_float(self) -> f64 {
        self.num as f64 / self.den as f64
    }
    #[inline]
    fn invariant(&self) {
        unsafe {
            assume_nonzero(self.den);
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num as i128 * other.den as i128).partial_cmp(&(other.num as i128 * self.den as i128))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num as i128 * other.den as i128).cmp(&(other.num as i128 * self.den as i128))
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::unnormalized(0, 1)
    }
}

impl ops::Neg for Rational {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.num = -self.num;
        self
    }
}

impl ops::Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        // (a1b2 + a2b1) / b1b2
        let gd = unsafe { gcd_nonzero(self.den, other.den) };
        let (num, of) = (self.num * (other.den / gd) as i64)
            .overflowing_add(other.num * (self.den / gd) as i64);
        if num == 0 {
            return Self::unnormalized(0, 1);
        }
        let neg = of ^ (num < 0);
        let num_abs = (if neg { num.wrapping_neg() } else { num }) as u64;
        let den = self.den / gd * other.den;
        let g = unsafe { gcd_nonzero(num_abs, den) };
        let num_abs = (num_abs / g) as i64;
        Self::unnormalized(if neg { -num_abs } else { num_abs }, den / g)
    }
}

impl ops::Add<i64> for Rational {
    type Output = Self;
    fn add(mut self, int: i64) -> Self::Output {
        self.num += self.den as i64 * int;
        self
    }
}

impl ops::Add<Rational> for i64 {
    type Output = Rational;
    fn add(self, mut ratio: Rational) -> Self::Output {
        ratio.num = ratio.den as i64 * self + ratio.num;
        ratio
    }
}

impl ops::Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl ops::Sub<i64> for Rational {
    type Output = Self;
    fn sub(mut self, int: i64) -> Self::Output {
        self.num -= self.den as i64 * int;
        self
    }
}

impl ops::Sub<Rational> for i64 {
    type Output = Rational;
    fn sub(self, mut ratio: Rational) -> Self::Output {
        ratio.num = ratio.den as i64 * self - ratio.num;
        ratio
    }
}

impl ops::Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if self.num == 0 || other.num == 0 {
            return Self::unnormalized(0, 1);
        }
        let g1 = unsafe { gcd_nonzero(self.num as u64, other.den) };
        let g2 = unsafe { gcd_nonzero(other.num as u64, self.den) };
        let neg = (self.num < 0) != (other.num < 0);
        let num_abs = (self.num as u64 / g1 * other.num as u64 / g2) as i64;
        Self {
            num: if neg { -num_abs } else { num_abs },
            den: (self.den / g2) * (other.den / g1),
        }
    }
}

impl ops::Mul<i64> for Rational {
    type Output = Self;
    fn mul(mut self, int: i64) -> Self::Output {
        if int == 0 {
            return Self::unnormalized(0, 1);
        }
        let g = unsafe { gcd_nonzero(self.den, int.abs() as u64) };
        self.num *= int / g as i64;
        self.den /= g;
        self
    }
}

impl ops::Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

impl ops::Div<i64> for Rational {
    type Output = Self;
    fn div(mut self, int: i64) -> Self::Output {
        assert_ne!(int, 0);
        if int < 0 {
            self.num = -self.num;
        }
        let int_abs = int.abs() as u64;
        let g = unsafe { gcd_nonzero(self.num.abs() as u64, int_abs) };
        self.num /= g as i64;
        self.den *= int_abs / g;
        self
    }
}

unsafe fn gcd_nonzero(x: u64, y: u64) -> u64 {
    assume_nonzero(x);
    assume_nonzero(y);
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
    assume_nonzero(x << tzg)
}

#[inline]
unsafe fn assume_nonzero(x: u64) -> u64 {
    if x == 0 {
        std::hint::unreachable_unchecked()
    }
    x
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Self::unnormalized(value, 1)
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
