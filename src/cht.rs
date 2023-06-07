use std::ops;

pub struct Cht<T> {
    lines: Vec<(T, T)>,
}

impl<T: Num> Cht<T> {
    pub fn new(lines: impl IntoIterator<Item = (T, T)>) -> Self {
        let mut lines_unfiltered: Vec<_> = lines.into_iter().collect();
        lines_unfiltered.sort_by(|(a1, _), (a2, _)| a2.partial_cmp(a1).expect("comparison failed"));
        let mut lines = vec![];

        for (a, b) in lines_unfiltered {
            if let &[.., (a1, b1)] = &lines[..] {
                if a == a1 {
                    if b >= b1 {
                        continue;
                    } else {
                        lines.pop();
                    }
                }
            }

            while let &[.., (a2, b2), (a1, b1)] = &lines[..] {
                // a0x0 + b0 = a1x0 + b1
                // (a0 - a1)x0 = b1 - b0
                // x0 = (b1 - b0) / (a0 - a1)
                // x2 = (b2 - b1) / (a1 - a2)
                // x0 <= x2
                // (b1 - b0) / (a0 - a1) <= (b2 - b1) / (a1 - a2)
                // (b1 - b0)(a1 - a2) <= (b2 - b1)(a0 - a1)
                if T::Wide::from(b1 - b) * T::Wide::from(a1 - a2)
                    <= T::Wide::from(b2 - b1) * T::Wide::from(a - a1)
                {
                    lines.pop();
                } else {
                    break;
                }
            }

            lines.push((a, b));
        }

        Self { lines }
    }

    pub fn y(&self, x: T) -> T {
        let mut li = 0;
        let mut ri = self.lines.len();
        while ri - li > 1 {
            let i = li + (ri - li) / 2;
            // a[i]x' + b[i] = a[i + 1]x' + b[i + 1]
            // x' = (b[i + 1] - b[i]) / (a[i] - a[i + 1])
            // x <= x'
            // x <= (b[i + 1] - b[i]) / (a[i] - a[i + 1])
            // (a[i] - a[i + 1])x <= b[i + 1] - b[i]
            let (a0, b0) = self.lines[i - 1];
            let (a1, b1) = self.lines[i];
            if T::Wide::from(a0 - a1) * T::Wide::from(x) <= T::Wide::from(b1 - b0) {
                ri = i;
            } else {
                li = i;
            }
        }
        let (a, b) = self.lines[li];
        a * x + b
    }

    pub fn lines(&self) -> &[(T, T)] {
        &self.lines
    }
}

impl<T: Num> FromIterator<(T, T)> for Cht<T> {
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        Self::new(iter)
    }
}

pub trait Num: NumBase {
    type Wide: NumBase + From<Self>;
}

pub trait NumBase:
    Copy
    + PartialOrd
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + std::fmt::Debug
{
}

macro_rules! num {
    ($Ty: ty, $Wide: ty) => {
        impl Num for $Ty {
            type Wide = $Wide;
        }
        impl NumBase for $Ty {}
    };
}

macro_rules! num_base {
    ($Ty: ty) => {
        impl NumBase for $Ty {}
    };
}

num!(i32, i64);
num!(i64, i128);
num!(f32, f32);
num!(f64, f64);

num_base!(i128);
