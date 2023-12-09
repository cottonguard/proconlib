pub fn bisect<F: FnMut(f64) -> bool>(mut l: f64, mut r: f64, mut f: F) -> f64 {
    const EPS: f64 = 1e-100;

    #[inline]
    fn to_bits(x: f64) -> i64 {
        let bits = x.to_bits() as i64;
        bits ^ ((bits >> (i64::BITS - 1)) as u64 >> 1) as i64
    }

    #[inline]
    fn from_bits(bits: i64) -> f64 {
        f64::from_bits((bits ^ ((bits >> (i64::BITS - 1)) as u64 >> 1) as i64) as u64)
    }

    let l_orig = l;

    loop {
        let h = if l * r < 0.0 {
            0.0
        } else {
            let l_bits = to_bits(if 0.0 <= l && l < EPS { EPS } else { l });
            let r_bits = to_bits(if -EPS < r && r <= 0.0 { -EPS } else { r });
            if r - l < EPS || r_bits.wrapping_sub(l_bits) <= 1 {
                break;
            }
            let h_bits = l_bits + (r_bits - l_bits) / 2;
            let h = from_bits(h_bits);

            if h.is_nan() {
                return h;
            }
            h
        };

        if f(h) {
            r = h;
        } else {
            l = h;
        }
    }

    if l == l_orig && f(l) {
        l
    } else {
        r
    }
}
