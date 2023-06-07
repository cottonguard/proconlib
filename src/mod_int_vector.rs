use crate::mod_int::*;
use std::simd::{i32x8, u32x4, u64x4, SimdUint};

/*
#[inline]
pub fn sum<M: Modulo>(a: &[ModInt<M>]) -> ModInt<M> {
    let mut offset = (a.as_ptr()).align_offset(std::mem::size_of::<i32x8>());
    if a.len() <= offset {
        return a.iter().sum();
    }
    let head_sum: ModInt<M> = a[..offset].iter().sum();
    let mut acc = i32x8::splat(0);
    while offset + 8 <= a.len() {
        let chunk: &[i32] = unsafe { std::mem::transmute(&a[offset..offset + 8]) };
        let v = i32x8::from_slice(chunk);
        acc = add_simd(acc, v, M::modulo() as i32);
        offset += 8;
    }
    let body_sum: ModInt<M> = acc.as_array().iter().map(|x| ModInt::new(*x as u32)).sum();
    let tail_sum: ModInt<M> = a[offset..].iter().sum();
    head_sum + body_sum + tail_sum
}
 */

#[inline]
pub fn sum<M: Modulo>(a: &[ModInt<M>]) -> ModInt<M> {
    let mut offset = (a.as_ptr()).align_offset(std::mem::align_of::<u32x4>());
    if a.len() <= offset {
        return a.iter().sum();
    }
    let head_sum: ModInt<M> = a[..offset].iter().sum();
    let mut acc = u64x4::splat(0);
    while offset + 4 <= a.len() {
        let chunk: &[u32] = unsafe { std::mem::transmute(&a[offset..offset + 4]) };
        let v = u32x4::from_slice(chunk);
        acc += v.cast();
        offset += 4;
    }
    let body_sum: ModInt<M> =
        ModInt::new((acc.as_array().iter().sum::<u64>() % M::modulo() as u64) as u32);
    let tail_sum: ModInt<M> = a[offset..].iter().sum();
    head_sum + body_sum + tail_sum
}

#[inline]
pub fn product<M: Modulo>(a: &[ModInt<M>]) -> ModInt<M> {
    let mut offset = (a.as_ptr()).align_offset(std::mem::align_of::<u32x4>());
    if a.len() <= offset {
        return a.iter().sum();
    }
    let head_prod: ModInt<M> = a[..offset].iter().product();
    let mut acc = u64x4::splat(1);
    while offset + 4 <= a.len() {
        let chunk: &[u32] = unsafe { std::mem::transmute(&a[offset..offset + 4]) };
        let v = u32x4::from_slice(chunk);
        acc = acc * v.cast::<u64>() % u64x4::splat(M::modulo() as u64);
        offset += 4;
    }
    let body_prod: ModInt<M> = acc
        .as_array()
        .iter()
        .map(|x| ModInt::new(*x as u32))
        .product();
    let tail_prod: ModInt<M> = a[offset..].iter().product();
    head_prod * body_prod * tail_prod
}

#[inline]
fn add_simd(x: i32x8, y: i32x8, m: i32) -> i32x8 {
    let sum = x + y;
    let mods = i32x8::splat(m);
    let mask = (mods - sum) >> i32x8::splat(31);
    sum - (mods & mask)
}

#[inline]
pub fn dot<M: Modulo>(a: &[ModInt<M>], b: &[ModInt<M>]) -> ModInt<M> {
    assert_eq!(a.len(), b.len());
    let mut offset = a.as_ptr().align_offset(std::mem::align_of::<u32x4>());
    let head_dot: ModInt<M> = dot_naive(&a[..offset], &b[..offset]);
    let mut body_dot = 0u64;
    while offset + 4 <= a.len() {
        let ca: &[u32] = unsafe { std::mem::transmute(&a[offset..offset + 4]) };
        let cb: &[u32] = unsafe { std::mem::transmute(&b[offset..offset + 4]) };
        let p = u32x4::from_slice(ca).cast::<u64>() * u32x4::from_slice(cb).cast::<u64>();
        body_dot += p.reduce_sum();
        offset += 4;
    }
    let body_dot = ModInt::new((body_dot % M::modulo() as u64) as u32);
    let tail_dot: ModInt<M> = dot_naive(&a[offset..], &b[offset..]);
    head_dot + body_dot + tail_dot
}

#[inline]
fn dot_naive<M: Modulo>(a: &[ModInt<M>], b: &[ModInt<M>]) -> ModInt<M> {
    let sum: u64 = a
        .iter()
        .zip(b)
        .map(|(a, b)| a.get() as u64 * b.get() as u64)
        .sum();
    ModInt::new((sum % M::modulo() as u64) as u32)
}
