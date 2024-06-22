// X: [0, 2^k)
// Range_m: floor(mX / 2^k)
// r <= mx / 2^k < r + 1
// r2^k / m <= x < (r + 1)2^k / m
// (r + 1)2^k / m - r2^k / m
// = 2^k / m
// recip(2^k / m)
// recip(mx / 2^k) < recip(2^k / m) = b
// 2^k recip(mx / 2^k) < 2^k b

pub struct Rng(u64);
