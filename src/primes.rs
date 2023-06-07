pub fn primes(n: usize) -> Vec<usize> {
    // 1, 7, 11, 13, 17, 19, 23, 29
    const SKIP: [u8; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
    const XTOI: [u8; 15] = [0, 0, 0, 1, 0, 2, 3, 0, 4, 5, 0, 6, 0, 0, 7];
    let mut sieve = vec![0u8; n / 30 + 1];
    let mut ps = vec![2, 3, 5];
    if n <= 4 {
        ps.truncate([0, 0, 1, 2, 2][n]);
        return ps;
    }
    let mut x = 7;
    let mut i = 1;
    while x <= n {
        if sieve[i / 8] & 1 << i % 8 == 0 {
            ps.push(x);
            let mut j = i;
            let mut y = x * x;
            while y <= n {
                sieve[y / 30] |= 1 << XTOI[y / 2 % 15];
                y += x * SKIP[j % 8] as usize;
                j += 1;
            }
        }
        x += SKIP[i % 8] as usize;
        i += 1;
    }
    ps
}
