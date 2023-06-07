pub fn subsets<T: Zero>(set: T) -> Subsets<T> {
    Subsets {
        set,
        cur: T::zero(),
        end: false,
    }
}
pub struct Subsets<T> {
    set: T,
    cur: T,
    end: bool,
}

pub fn bits<T>(n: T) -> Bits<T> {
    Bits(n)
}
pub struct Bits<T>(T);

pub fn bit_positions<T>(n: T) -> BitPositions<T> {
    BitPositions(n)
}
pub struct BitPositions<T>(T);

pub fn gray_code<T: Zero + One + std::ops::Shl<u32, Output = T>>(n: u32) -> GrayCode<T> {
    GrayCode(T::zero(), T::one() << n)
}

pub struct GrayCode<T>(T, T);

pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}

macro_rules! imp {
    ($ty: ty) => {
        impl Iterator for Subsets<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.end {
                    None
                } else {
                    let res = self.cur;
                    self.cur = (self.cur | !self.set).wrapping_add(1) & self.set;
                    self.end = self.cur == 0;
                    Some(res)
                }
            }
        }

        impl Iterator for Bits<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.0 == 0 {
                    None
                } else {
                    let lsb = self.0 & !self.0 + 1;
                    self.0 ^= lsb;
                    Some(lsb)
                }
            }
        }
        /*
        impl DoubleEndedIterator for Bits<$ty> {
            fn next_back(&mut self) -> Option<$ty> {
                if self.0 == 0 {
                    None
                } else {
                    let sb =
                }
            }
        }
         */
        impl ExactSizeIterator for Bits<$ty> {
            fn len(&self) -> usize {
                self.0.count_ones() as _
            }
        }

        impl Iterator for BitPositions<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.0 == 0 {
                    None
                } else {
                    let res = self.0.trailing_zeros() as $ty;
                    self.0 ^= 1 << res;
                    Some(res)
                }
            }
        }

        impl Iterator for GrayCode<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.0 < self.1 {
                    let x = self.0;
                    self.0 += 1;
                    Some(x ^ (x >> 1))
                } else {
                    None
                }
            }
        }

        impl Zero for $ty {
            fn zero() -> Self {
                0
            }
        }

        impl One for $ty {
            fn one() -> Self {
                1
            }
        }
    };
}

imp!(usize);
imp!(u8);
imp!(u16);
imp!(u32);
imp!(u64);
imp!(u128);
imp!(isize);
imp!(i8);
imp!(i16);
imp!(i32);
imp!(i64);
imp!(i128);
