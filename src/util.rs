use std::fmt::Debug;

pub fn pow<T: Clone, F: FnMut(&T, &T) -> T, K: TryInto<u64>>(x: T, k: K, id: T, mut mul: F) -> T
where
    K::Error: Debug,
{
    let mut k = k.try_into().unwrap();
    if k == 0 {
        return id;
    }
    let mut y = id;
    let mut x = x;
    while k > 1 {
        if k & 1 == 1 {
            y = mul(&y, &x);
        }
        x = mul(&x, &x);
        k >>= 1;
    }
    mul(&y, &x)
}
