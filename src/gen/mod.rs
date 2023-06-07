use std::collections::BinaryHeap;

use crate::random::*;

pub fn seed() -> u64 {
    use std::time::SystemTime;
    let dur = SystemTime::UNIX_EPOCH
        .elapsed()
        .map_or(0, |dur| dur.as_nanos());
    dur as u64
}

pub fn random_perm<R: Rng>(rng: &mut R, n: usize, min: usize) -> Vec<usize> {
    let mut p: Vec<usize> = (min..min + n).collect();
    rng.shuffle(&mut p);
    p
}

pub fn random_tree<R: Rng>(rng: &mut R, n: usize) -> Vec<usize> {
    let pcode: Vec<usize> = (0..n - 2).map(|_| rng.range(0, n)).collect();
    let mut cnt = vec![0; n];
    for &i in &pcode {
        cnt[i] += 1;
    }
    let mut leaves = BinaryHeap::new();
    for i in 0..n {
        if cnt[i] == 0 {
            leaves.push(i);
        }
    }
    let mut par = vec![0; n];
    for p in pcode {
        let c = leaves.pop().unwrap();
        par[c] = p;
        cnt[p] -= 1;
        if cnt[p] == 0 {
            leaves.push(p);
        }
    }
    let c = leaves.pop().unwrap();
    par[c] = 0;
    par
}
