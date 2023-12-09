use std::{
    collections::{
        hash_map::{self, Entry},
        HashMap,
    },
    hash::Hash,
    ops::{Add, Sub},
};

#[derive(Clone, Debug)]
pub struct Counter<T, C = usize>(HashMap<T, C>);

impl<T, C> Counter<T, C> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn num_kind(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl<T: Eq + Hash, C: Count> Counter<T, C> {
    pub fn get(&self, value: &T) -> C {
        self.0.get(value).copied().unwrap_or(C::ZERO)
    }

    pub fn map<F: FnMut(C) -> C>(&mut self, value: T, mut f: F) -> C {
        match self.0.entry(value) {
            Entry::Vacant(e) => {
                let res = f(C::ZERO);
                if res != C::ZERO {
                    e.insert(res);
                }
                res
            }
            Entry::Occupied(mut e) => {
                let count = e.get_mut();
                *count = f(*count);
                let res = *count;
                if res == C::ZERO {
                    e.remove();
                }
                res
            }
        }
    }

    pub fn set(&mut self, value: T, count: C) -> C {
        self.map(value, |_| count)
    }

    pub fn inc(&mut self, value: T) -> C {
        self.inc_by(value, C::ONE)
    }

    pub fn dec(&mut self, value: T) -> C {
        self.dec_by(value, C::ONE)
    }

    pub fn inc_by(&mut self, value: T, count: C) -> C {
        self.map(value, |c| c + count)
    }

    pub fn dec_by(&mut self, value: T, count: C) -> C {
        self.map(value, |c| c - count)
    }

    pub fn iter(&self) -> Iter<T, C> {
        self.into_iter()
    }
}

/*
impl<T: Eq + Hash, C: Count> Index<T> for Counter<T, C> {
    type Output = C;
    fn index(&self, value: T) -> &Self::Output {
        if let Some(c) = self.0.get(&value) {
            c
        } else {
            &C::ZERO
        }
    }
}
 */

impl<T: Hash + Eq, C: Count> From<HashMap<T, C>> for Counter<T, C> {
    fn from(mut map: HashMap<T, C>) -> Self {
        map.retain(|_, c| *c != C::ZERO);
        Self(map)
    }
}

impl<T: Hash + Eq, C: Count> FromIterator<(T, C)> for Counter<T, C> {
    fn from_iter<I: IntoIterator<Item = (T, C)>>(iter: I) -> Self {
        Self(iter.into_iter().filter(|(_, c)| *c != C::ZERO).collect())
    }
}

impl<'a, T, C: Count> IntoIterator for &'a Counter<T, C> {
    type Item = (&'a T, C);
    type IntoIter = Iter<'a, T, C>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

impl<T, C: Count> IntoIterator for Counter<T, C> {
    type Item = (T, C);
    type IntoIter = IntoIter<T, C>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

pub struct Iter<'a, T, C>(hash_map::Iter<'a, T, C>);

impl<'a, T, C: Count> Iterator for Iter<'a, T, C> {
    type Item = (&'a T, C);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(v, c)| (v, *c))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T, C: Count> ExactSizeIterator for Iter<'a, T, C> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct IntoIter<T, C>(hash_map::IntoIter<T, C>);

impl<T, C: Count> Iterator for IntoIter<T, C> {
    type Item = (T, C);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<T, C: Count> ExactSizeIterator for IntoIter<T, C> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

pub trait Count: Copy + Eq + Add<Output = Self> + Sub<Output = Self> {
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! count {
    ($ty: ident) => {
        impl Count for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

count!(usize);
count!(isize);
count!(u8);
count!(i8);
count!(u16);
count!(i16);
count!(u32);
count!(i32);
count!(u64);
count!(i64);
count!(u128);
count!(i128);
