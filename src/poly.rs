// f(x) = x^k f1(x) + f2(x)
// g(x) = x^k g1(x) + g2(x)
// f(x)g(x) = x^2k f1(x)g1(x) + x^k(f1(x)g2(x) + f2(x)g1(x)) + f2(x)g2(x)
// h(x) = (f1(x) + f2(x))(g1(x) + g2(x))
//      = f1(x)g1(x) + f2(x)g1(x) + f1(x)g2(x) + f2(x)g2(x)
// f(x)g(x) = x^2k f1(x)g1(x) + x^k(h(x) - f1(x)g1(x) - f2(x)g2(x)) + f2(x)g2(x)

use std::ops;

pub trait Num:
    Copy
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
{
    fn zero() -> Self;
}

macro_rules! num {
    ($ty: ty) => {
        impl Num for $ty {
            #[inline]
            fn zero() -> Self {
                0i32 as Self
            }
        }
    };
}

num!(isize);
num!(i8);
num!(i16);
num!(i32);
num!(i64);
num!(i128);
num!(f32);
num!(f64);

const LONG_MUL_THRESHOULD: usize = 16;

fn long_mul<T: Num>(a: &[T], b: &[T], dst: &mut [T]) {
    for (i, &a) in a.iter().enumerate() {
        for (j, &b) in b.iter().enumerate() {
            dst[i + j] += a * b;
        }
    }
}

fn karatsuba<T: Num>(a: &[T], b: &[T], dst: &mut [T], buf: &mut [T]) {
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };

    if b.is_empty() {
        return;
    }

    if b.len() <= LONG_MUL_THRESHOULD {
        long_mul(a, b, dst);
        return;
    }

    let mid = (a.len() + 1) / 2;

    if b.len() <= mid {
        karatsuba(&a[..mid], b, &mut dst[..], buf);
        karatsuba(&a[mid..], b, &mut dst[mid..], buf);
        return;
    }

    let (buf, buf_rest) = buf.split_at_mut(2 * mid);
    for buf in buf.iter_mut() {
        *buf = T::zero();
    }
    karatsuba(&a[mid..], &b[mid..], buf, buf_rest);
    let buf_len = (a.len() - mid) + (b.len() - mid) - 1;
    add(&buf[..buf_len], &mut dst[2 * mid..]);
    sub(&buf[..buf_len], &mut dst[mid..]);

    for buf in buf.iter_mut() {
        *buf = T::zero();
    }
    karatsuba(&a[..mid], &b[..mid], buf, buf_rest);
    add(&buf[..2 * mid - 1], &mut dst[..]);
    sub(&buf[..2 * mid - 1], &mut dst[mid..]);

    for buf in buf.iter_mut() {
        *buf = T::zero();
    }

    let (sum_a, sum_b) = buf.split_at_mut(mid);
    for i in 0..mid {
        sum_a[i] = if let Some(v) = a.get(mid + i) {
            a[i] + *v
        } else {
            a[i]
        };
    }
    for i in 0..mid {
        sum_b[i] = if let Some(v) = b.get(mid + i) {
            b[i] + *v
        } else {
            b[i]
        };
    }
    karatsuba(sum_a, sum_b, &mut dst[mid..], buf_rest);
}

#[inline]
fn add<T: Num>(a: &[T], dst: &mut [T]) {
    for (a, dst) in a.iter().zip(dst) {
        *dst += *a;
    }
}

#[inline]
fn sub<T: Num>(a: &[T], dst: &mut [T]) {
    for (a, dst) in a.iter().zip(dst) {
        *dst -= *a;
    }
}

pub fn mul<T: Num>(a: &[T], b: &[T]) -> Vec<T> {
    let mut dst = vec![T::zero(); a.len() + b.len() - 1];
    let max_len = a.len().max(b.len());
    let buf_len = if max_len > LONG_MUL_THRESHOULD {
        2 * max_len
    } else {
        0
    };
    karatsuba(a, b, &mut dst, &mut vec![T::zero(); buf_len]);
    dst
}

// 2ceil(a.len/2) + 2ceil(ceil(a.len/2)/2) + ...
