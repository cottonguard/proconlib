pub trait Bisect<T> {
    fn bisect<F: Fn(&T) -> bool>(&self, cond: F) -> (usize, Option<&T>);
    fn lower_bound(&self, x: &T) -> (usize, Option<&T>)
    where
        T: Ord,
    {
        self.bisect(|v| v >= x)
    }
    fn upper_bound(&self, x: &T) -> (usize, Option<&T>)
    where
        T: Ord,
    {
        self.bisect(|v| v > x)
    }
}
impl<T> Bisect<T> for [T] {
    fn bisect<F: Fn(&T) -> bool>(&self, cond: F) -> (usize, Option<&T>) {
        let mut l = -1;
        let mut r = self.len() as isize;
        while r - l > 1 {
            let m = (l + r) / 2;
            if cond(&self[m as usize]) {
                r = m;
            } else {
                l = m;
            }
        }
        let i = r as usize;
        (i, self.get(i))
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lower_bound() {
        let a = [1, 1, 2, 2, 2, 3, 4, 10, 10, 11];
        assert_eq!(a.lower_bound(&2), 2);
        assert_eq!(a.lower_bound(&1), 0);
        assert_eq!(a.lower_bound(&7), 7);
        assert_eq!(a.lower_bound(&100), a.len());
    }
    #[test]
    fn upper_bound() {
        let a = [1, 1, 2, 2, 2, 3, 4, 10, 10, 11];
        assert_eq!(a.upper_bound(&2), 5);
        assert_eq!(a.upper_bound(&1), 2);
        assert_eq!(a.upper_bound(&7), 7);
        assert_eq!(a.upper_bound(&100), a.len());
    }
}
*/
