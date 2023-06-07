use std::mem;

pub fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    if s.is_empty() {
        return vec![];
    }

    let mut sa: Vec<_> = (0..s.len()).collect();
    sa.sort_by_key(|&i| &s[i]);

    let mut rank = vec![0usize; s.len()];
    rank[sa[0]] = 1;
    let mut r = 1;
    for sa in sa.windows(2) {
        if s[sa[0]] != s[sa[1]] {
            r += 1;
        }
        rank[sa[1]] = r;
    }

    let mut temp = vec![0; s.len()];

    for h in (0..).map(|e| 1 << e) {
        let key = |i: usize| (rank[i], *rank.get(i + h).unwrap_or(&0));
        sa.sort_by_key(|&i| key(i));

        if h >= s.len() {
            break;
        }

        temp[sa[0]] = 1;
        let mut r = 1;
        for sa in sa.windows(2) {
            if key(sa[0]) != key(sa[1]) {
                r += 1;
            }
            temp[sa[1]] = r;
        }
        mem::swap(&mut rank, &mut temp);
    }

    sa
}

pub fn lcp_array<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
    let mut rank = vec![0; sa.len()];
    for (i, &sa) in sa.iter().enumerate() {
        rank[sa] = i;
    }
    let mut lcp = vec![0; sa.len() - 1];
    let mut c = 0usize;
    for (i, rank) in rank.into_iter().enumerate() {
        c = c.saturating_sub(1);
        if rank == 0 {
            continue;
        }
        let j = sa[rank - 1];
        while s.get(i + c) == s.get(j + c) {
            c += 1;
        }
        lcp[rank - 1] = c;
    }
    lcp
}
