#[derive(Clone, Debug)]
pub struct CoordComp<T>(Vec<T>);
impl<T: Ord> CoordComp<T> {
    pub fn position(&self, key: &T) -> Option<usize> {
        let i = self.lower_bound(key);
        if self.get(i) == Some(key) {
            Some(i)
        } else {
            None
        }
    }
    pub fn lower_bound(&self, key: &T) -> usize {
        let mut l = 0;
        let mut r = self.len() + 1;
        while r - l > 1 {
            let h = l + (r - l) / 2;
            if &self.0[h - 1] < key {
                l = h;
            } else {
                r = h;
            }
        }
        l
    }
    pub fn insert(&mut self, key: T) -> usize {
        let i = self.lower_bound(&key);
        if self.0.get(i) != Some(&key) {
            self.0.insert(i, key);
        }
        i
    }
}
impl<T> std::ops::Deref for CoordComp<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}
impl<T: Ord> From<Vec<T>> for CoordComp<T> {
    fn from(mut a: Vec<T>) -> Self {
        a.sort();
        a.dedup();
        Self(a)
    }
}
impl<T: Ord> std::iter::FromIterator<T> for CoordComp<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let a: Vec<_> = iter.into_iter().collect();
        a.into()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_comp() {
        use crate::random::*;
        const N: usize = 100;
        const M: u32 = 1000000;
        let mut rand = Pcg::seed_from_u64(1);
        let mut a: Vec<_> = (0..N).map(|_| rand.range(0, M)).collect();
        for _ in 0..N {
            a.push(*rand.choose(&a));
        }
        rand.shuffle(&mut a);
        let cc = CoordinateComp::from(&a[..]);
        a.sort();
        a.dedup();
        for (i, x) in a.iter().enumerate() {
            assert_eq!(cc.position(&x), Some(i));
            assert_eq!(cc.lower_bound(&x), i);
        }
        for _ in 0..N {
            let x = rand.range(0, M);
            assert_eq!(a.iter().position(|a| a == &x), cc.position(&x));
            assert_eq!(
                a.iter().position(|a| a >= &x).unwrap_or(a.len()),
                cc.lower_bound(&x)
            );
        }
        assert_eq!(cc.lower_bound(&M), cc.len());
    }
}
*/
