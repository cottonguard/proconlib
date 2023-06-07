use std::{
    cmp::Ordering,
    collections::{hash_map::Entry::*, BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

pub fn dijkstra<V, W, I>(s: V, mut adj: impl FnMut(V) -> I) -> HashMap<V, W>
where
    V: Clone + Eq + Hash,
    W: Clone + Zero + Ord + Add<Output = W>,
    I: IntoIterator<Item = (V, W)>,
{
    let mut dist = HashMap::new();
    dist.insert(s.clone(), W::zero());
    let mut que = BinaryHeap::new();
    que.push(Entry(s, W::zero()));
    while let Some(Entry(u, du)) = que.pop() {
        if &du > dist.get(&u).unwrap() {
            continue;
        }
        for (v, wuv) in adj(u) {
            let dv = du.clone() + wuv;
            match dist.entry(v.clone()) {
                Occupied(mut e) => {
                    let dv_cur = e.get_mut();
                    if dv < *dv_cur {
                        *dv_cur = dv.clone();
                        que.push(Entry(v, dv));
                    }
                }
                Vacant(e) => {
                    e.insert(dv.clone());
                    que.push(Entry(v, dv));
                }
            }
        }
    }
    dist
}

#[derive(Debug)]
struct Entry<V, W>(V, W);

impl<V, W: PartialEq> PartialEq for Entry<V, W> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<V, W: Eq> Eq for Entry<V, W> {}

impl<V, W: PartialOrd> PartialOrd for Entry<V, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl<V, W: Ord> Ord for Entry<V, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! zero {
    ($($ty:ty),*) => {$(
        impl Zero for $ty {
            fn zero() -> Self {
                0
            }
        }
    )*};
}

zero!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);
