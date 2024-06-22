use std::{
    cell::RefCell,
    hash::{BuildHasher, Hasher, RandomState},
};

pub struct Rbst<K, V> {
    root: Node<K, V>,
}

struct Node<K, V>(Option<Box<NodeInner<K, V>>>);

struct NodeInner<K, V> {
    key: K,
    value: V,
    size: usize,
    left: Node<K, V>,
    right: Node<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    const NIL: Self = Self(None);

    #[inline]
    fn size(&self) -> usize {
        self.0.as_ref().map(|node| node.size).unwrap_or(0)
    }

    #[inline]
    fn join(self, mut right: Self, rng: &mut Rng) -> Self {
        let mut left = self;
        match (&mut left.0, &mut right.0) {
            (Some(l), Some(r)) => {
                if rng.bool(l.size as u32, r.size as u32) {
                    l.right = l.right.take().join(right, rng);
                    left
                } else {
                    r.left = left.join(r.left.take(), rng);
                    right
                }
            }
            (None, Some(_)) => right,
            _ => left,
        }
    }

    #[inline]
    fn split(mut self, key: &K) -> (Self, Self) {
        if let Some(ref mut node) = self.0 {
            if *key < node.key {
                let (ll, lr) = node.left.take().split(key);
                node.left = lr;
                (ll, self)
            } else {
                let (rl, rr) = node.right.take().split(key);
                node.right = rl;
                (self, rr)
            }
        } else {
            (Self::NIL, Self::NIL)
        }
    }

    fn merge(self, other: Self, rng: &mut Rng) -> Self {
        let s = match (self.0, other.0) {
            (Some(s), Some(t)) => {
                let (mut s, t) = if rng.bool(s.size as u32, t.size as u32) {
                    (s, t)
                } else {
                    (t, s)
                };
                s.size += t.size;
                let (tl, tr) = split(t, &s.key);
                s.left = s.left.take().merge(tl, rng);
                s.right = s.right.take().merge(tr, rng);
                s
            }
            (Some(s), _) => s,
            (_, Some(t)) => t,
            _ => return Self::NIL,
        };
        Self(Some(s))
    }

    fn take(&mut self) -> Self {
        Self(self.0.take())
    }
}

#[inline]
fn split<K: Ord, V>(mut node: Box<NodeInner<K, V>>, key: &K) -> (Node<K, V>, Node<K, V>) {
    if *key < node.key {
        let (ll, lr) = node.left.take().split(key);
        node.left = lr;
        node.size -= ll.size();
        (ll, Node(Some(node)))
    } else {
        let (rl, rr) = node.right.take().split(key);
        node.right = rl;
        node.size -= rr.size();
        (Node(Some(node)), rr)
    }
}

struct Rng(pub [u64; 2]);

/// <https://prng.di.unimi.it/xoroshiro128starstar.c>
impl Rng {
    fn new() -> Self {
        THREAD_RNG.with(|t| Self::seed_from_u64(t.borrow_mut().u64()))
    }
    fn seed_from_u64(mut seed: u64) -> Self {
        /// <https://prng.di.unimi.it/splitmix64.c>
        fn splitmix(x: &mut u64) -> u64 {
            *x = x.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *x;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            return z ^ (z >> 31);
        }
        Self([splitmix(&mut seed), splitmix(&mut seed)])
    }
    fn u64(&mut self) -> u64 {
        let [s0, mut s1] = self.0;
        s1 ^= s0;
        self.0 = [s0.rotate_left(24) ^ s1 ^ (s1 << 16), s1.rotate_left(37)];
        s0.wrapping_mul(5).rotate_left(7).wrapping_mul(5)
    }
    fn u32(&mut self) -> u32 {
        (self.u64() >> 32) as u32
    }
    fn bool(&mut self, w_true: u32, w_false: u32) -> bool {
        // w_true / (w_true + w_false) > [0, 1)
        let r = self.u32();
        ((w_true as u64 * r as u64) >> 32) as u32 > w_false + w_true
    }
}

thread_local! {
    static THREAD_RNG: RefCell<Rng> = RefCell::new(Rng::seed_from_u64(RandomState::new().build_hasher().finish()));
}
