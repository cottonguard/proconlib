use crate::modint2::*;

pub fn dft<M: Modulo>(a: &mut [ModInt<M>]) {
    dft_impl(a, false);
}

pub fn idft<M: Modulo>(a: &mut [ModInt<M>]) {
    dft_impl(a, true);
    let ninv = ModInt::new(a.len() as u32).inv();
    for a in a {
        *a *= ninv;
    }
}

#[inline]
fn dft_impl<M: Modulo>(a: &mut [ModInt<M>], inv: bool) {
    assert!(a.len().is_power_of_two());
    if a.len() <= 2 {
        if a.len() == 2 {
            let x = a[0];
            let y = a[1];
            a[0] = x + y;
            a[1] = x - y;
        }
        return;
    }
    let sh = a.len().leading_zeros() + 1;
    for i in 0..a.len() {
        let j = i.reverse_bits() >> sh;
        if j < i {
            a.swap(i, j);
        }
    }
    // w[i] == 1^2^-i
    let e = a.len().trailing_zeros() as usize;
    let mut w = [ModInt::<M>::new(0); 32];
    let pr = ModInt::new(primitive_root::<M>());
    let pr = if inv { pr.inv() } else { pr };
    w[e] = pr.pow((M::modulo() - 1) >> e);
    for i in (0..e).rev() {
        w[i] = w[i + 1] * w[i + 1];
    }
    for r in (4..=a.len()).step_by(4) {
        let l = r - 4;
        let s0 = a[l] + a[l + 1];
        let s1 = a[l + 2] + a[l + 3];
        let d0 = a[l] - a[l + 1];
        let d1w = w[2] * (a[l + 2] - a[l + 3]);
        a[l] = s0 + s1;
        a[l + 1] = d0 + d1w;
        a[l + 2] = s0 - s1;
        a[l + 3] = d0 - d1w;
        for e in 3..=r.trailing_zeros() {
            let l = r - (1 << e);
            let m = r - (1 << (e - 1));

            let x = a[l];
            let y = a[m];
            a[l] = x + y;
            a[m] = x - y;

            let wb = w[e as usize];
            let mut wi = wb;
            for j in m + 1..r {
                let i = j - (1 << (e - 1));
                let x = a[i].get() as u64;
                let y = wi.get() as u64 * a[j].get() as u64;
                a[i] = ModInt::from(x + y);
                a[j] = ModInt::from(x as i64 - y as i64);
                wi *= wb;
            }
        }
    }
}

/*
#[inline]
fn dft_radix4<M: Modulo>(a: &mut [ModInt<M>], inv: bool) {
    assert!(a.len().is_power_of_two());
    if a.len() <= 2 {
        if a.len() == 2 {
            let x = a[0];
            let y = a[1];
            a[0] = x + y;
            a[1] = x - y;
        }
        return;
    }
    let sh = a.len().leading_zeros() + 1;
    for i in 0..a.len() {
        let j = i.reverse_bits() >> sh;
        if j < i {
            a.swap(i, j);
        }
    }
    // w[i] == 1^2^-i
    let e = a.len().trailing_zeros() as usize;
    let mut w = vec![ModInt::<M>::new(0); e + 1];
    let pr = ModInt::new(primitive_root::<M>());
    let pr = if inv { pr.inv() } else { pr };
    w[e] = pr.pow((M::modulo() - 1) >> e);
    for i in (0..e).rev() {
        w[i] = w[i + 1] * w[i + 1];
    }
    for r in (4..=a.len()).step_by(4) {
        let l = r - 4;
        let s0 = a[l] + a[l + 1];
        let s1 = a[l + 2] + a[l + 3];
        let d0 = a[l] - a[l + 1];
        let d1w = w[2] * (a[l + 2] - a[l + 3]);
        a[l] = s0 + s1;
        a[l + 1] = d0 + d1w;
        a[l + 2] = s0 - s1;
        a[l + 3] = d0 - d1w;
        for e in (4..=r.trailing_zeros()).step_by(2) {
            let l = r - (1 << e);
            let ofs = 1 << (e - 2);
            let wb = w[e as usize];
            let mut w1 = ModInt::new(1);
            for i in l..l + ofs {
                let w2 = w1 * w1;
                let w3 = w1 * w2;
                let a0 = a[i];
                let a1 = w2 * a[i + ofs];
                let a2 = w1 * a[i + 2 * ofs];
                let a3 = w3 * a[i + 3 * ofs];
                let s0 = a0 + a1;
                let d0 = a0 - a1;
                let s1 = a2 + a3;
                let d1 = w[2] * (a2 - a3);
                a[i] = s0 + s1;
                a[i + ofs] = d0 + d1;
                a[i + 2 * ofs] = s0 - s1;
                a[i + 3 * ofs] = d0 - d1;
                w1 *= wb;
            }
        }
    }
    if a.len().trailing_zeros() % 2 == 1 {
        let wb = w[a.len().trailing_zeros() as usize];
        let mut wi = ModInt::new(1);
        for i in 0..a.len() / 2 {
            let j = i + a.len() / 2;
            let x = a[i];
            let y = wi * a[j];
            a[i] = x + y;
            a[j] = x - y;
            wi *= wb;
        }
    }
}
 */
// fk = sum w^ik xi
//    = sum w^4ik x4i + sum w^(4i+1)k + sum w^(4i+2)k + sum w^(4i+3)k
//    = f_0(k) + w^k f_1(k) + w^2k f_2(k) + w^3k f_3(k)
// w_N^N/4  = 1^1/2  = w_2
// w_N^N/2  = -1
// w_N^3N/4 = -1^1/2 = -w_2

// f(k + N/4)  = f_0(k) + w_2 w^k f_1(k) - w^2k f_2(k) - w_2 w^3k f_3(k)
// f(k + N/2)  = f_0(k) - w^k f_1(k)     + w^2k f_2(k) - w^3k f_3(k)
// f(k + 3N/4) = f_0(k) - w_2 w^k f_1(k) - w^2k f_2(k) + w_2 w^3k f_3(k)

fn primitive_root<M: Modulo>() -> u32 {
    match M::modulo() {
        998244353 => 3,
        _ => todo!(),
    }
}
