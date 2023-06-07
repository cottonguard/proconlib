/*
pub struct DisjointSparseTable<T, F> {
    len: usize,
    f: F,
    data: Vec<T>,
}

impl DisjointSparseTable<T, F: Fn(&T, &T) -> T> {
    pub fn new(mut a: Vec<T>, f: F) -> Self {
        let len = a.len();
        let h = (0usize.leading_zeros() - (len - 1).leading_zeros()).max(1);
    }
}
*/
