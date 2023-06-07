pub fn next_permutation<T: Ord>(p: &mut [T]) -> bool {
    next_prev_permutation_impl(p, |x, y| x < y)
}

pub fn prev_permutation<T: Ord>(p: &mut [T]) -> bool {
    next_prev_permutation_impl(p, |x, y| x > y)
}

#[inline]
pub fn next_prev_permutation_impl<T>(p: &mut [T], f: impl Fn(&T, &T) -> bool) -> bool {
    for i in (0..p.len() - 1).rev() {
        if f(&p[i], &p[i + 1]) {
            for j in (0..p.len()).rev() {
                if f(&p[i], &p[j]) {
                    p.swap(i, j);
                    p[i + 1..].reverse();
                    return true;
                }
            }
        }
    }
    p.reverse();
    false
}

pub trait Permutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

impl<T: Ord> Permutation for [T] {
    fn next_permutation(&mut self) -> bool {
        next_permutation(self)
    }
    fn prev_permutation(&mut self) -> bool {
        prev_permutation(self)
    }
}
