use std::{cmp::Ordering, mem};

pub struct RbsTree<K, V> {
    root: Node<K, V>,
    rng: Rng,
}

struct Node<K, V>(Option<Box<NodeInner<K, V>>>);

struct NodeInner<K, V> {
    key: K,
    value: V,
    size: usize,
    left: Node<K, V>,
    right: Node<K, V>,
}

impl<K, V> RbsTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: Node(None),
            rng: Rng::new(),
        }
    }
}
impl<K: Ord, V> RbsTree<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.root.get_mut(key)
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (root, prev) = self.root.take().insert(key, value, self.rng.next());
        self.root = Node(Some(root));
        prev
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let root = self.root.take();
        root.remove(key)
    }
    pub fn merge(&mut self, other: Self) {
        self.root = self.root.take().merge(other.root, &mut self.rng);
    }
    pub fn split(&mut self, key: &K) -> Self {
        let (l, r) = self.root.take().split(key);
        self.root = l;
        Self {
            root: r,
            rng: self.rng.create_rng(),
        }
    }
}
/*
    p
   / \
  l   r
 /\   /\
a  b c  d

  l
 / \
a   p
    /\
   b  r
      /\
     c  d
 */

impl<K: Ord, V> Node<K, V> {
    fn nil() -> Self {
        Self(None)
    }
    fn take(&mut self) -> Self {
        Self(self.0.take())
    }
    /*
    fn replace(&mut self, inner: Box<NodeInner<K, V>>) -> Self {
        Self(self.0.replace(inner))
    } */
    fn size(&self) -> usize {
        self.0.as_ref().map(|node| node.size).unwrap_or(0)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.0.as_ref().and_then(|node| match key.cmp(&node.key) {
            Ordering::Equal => Some(&node.value),
            Ordering::Less => node.left.get(key),
            Ordering::Greater => node.right.get(key),
        })
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.as_mut().and_then(|node| match key.cmp(&node.key) {
            Ordering::Equal => Some(&mut node.value),
            Ordering::Less => node.left.get_mut(key),
            Ordering::Greater => node.right.get_mut(key),
        })
    }
    fn insert(self, key: K, value: V, pri: u32) -> (Box<NodeInner<K, V>>, Option<V>) {
        if let Some(mut node) = self.0 {
            match key.cmp(&node.key) {
                Ordering::Equal => {
                    let prev = mem::replace(&mut node.value, value);
                    return (node, Some(prev));
                }
                Ordering::Less => {
                    let (left, prev) = node.left.take().insert(key, value, pri);
                    if prev.is_some() {
                        node.left = left.into_node();
                        (node, prev)
                    } else {
                        node.size += 1;
                        (node.rot_right(left, pri), None)
                    }
                }
                Ordering::Greater => {
                    let (right, prev) = node.right.take().insert(key, value, pri);
                    if prev.is_some() {
                        node.right = right.into_node();
                        (node, prev)
                    } else {
                        node.size += 1;
                        (node.rot_left(right, pri), None)
                    }
                }
            }
        } else {
            (
                Box::new(NodeInner {
                    key,
                    value,
                    size: 1,
                    left: Node(None),
                    right: Node(None),
                }),
                None,
            )
        }
    }
    fn remove(self, key: &K) -> Option<V> {
        if let Some(node) = self.0 {
            match key.cmp(&node.key) {
                Ordering::Equal => todo!(),
                Ordering::Less => node.left.remove(key),
                Ordering::Greater => node.right.remove(key),
            }
        } else {
            None
        }
    }
    fn merge(self, other: Self, rng: &mut Rng) -> Self {
        match (self.0, other.0) {
            (s, None) => Node(s),
            (None, t) => Node(t),
            (Some(mut s), Some(mut t)) => {
                if s.key == t.key {
                    return s.left.merge(s.right, rng).merge(t.into_node(), rng);
                }
                if s.key > t.key {
                    mem::swap(&mut s, &mut t);
                }
                // p = |s| / (|s| + |t|)
                // x: [0, 1)
                // p < x
                if ((((s.size + t.size) as u64 * rng.next() as u64) >> 32) as usize) < s.size {
                    let s_right = s.right.take();
                    s.right = s_right.merge(t.into_node(), rng);
                    s.into_node()
                } else {
                    let t_left = t.left.take();
                    t.left = s.into_node().merge(t_left, rng);
                    t.into_node()
                }
            }
        }
    }
    // fn join(self, other: Self) -> Self {}
    fn split(mut self, key: &K) -> (Self, Self) {
        if let Some(ref mut node) = self.0 {
            if *key <= node.key {
                let (lt, rt) = node.left.take().split(key);
                node.left = rt;
                node.size -= lt.size();
                (lt, self)
            } else {
                let (lt, rt) = node.right.take().split(key);
                node.right = lt;
                node.size -= rt.size();
                (self, rt)
            }
        } else {
            (Self::nil(), Self::nil())
        }
    }
}

impl<K: Ord, V> NodeInner<K, V> {
    fn rot_right(mut self: Box<Self>, mut left: Box<Self>, pri: u32) -> Box<Self> {
        if (((pri as u64 * self.size as u64) >> 32) as usize) < left.size {
            dbg!(pri);
            self.left = left.right.take();
            left.right = self.into_node();
            left
        } else {
            self.left = left.into_node();
            self
        }
    }
    fn rot_left(mut self: Box<Self>, mut right: Box<Self>, pri: u32) -> Box<Self> {
        if (((pri as u64 * self.size as u64) >> 32) as usize) < right.size {
            dbg!(pri);
            self.right = right.left.take();
            right.left = self.into_node();
            right
        } else {
            self.right = right.into_node();
            self
        }
    }
    fn into_node(self: Box<Self>) -> Node<K, V> {
        Node(Some(self))
    }
}

// p = 1 / (N + 1)
// (N + 1) * [0, 1) = [0, N + 1)

struct Rng([u32; 4]);

impl Rng {
    fn new() -> Self {
        Self([111, 222, 333, 444])
    }
    fn create_rng(&mut self) -> Rng {
        let mut state = [0; 4];
        for s in &mut state {
            *s = self.next();
        }
        Self(state)
    }
    fn next(&mut self) -> u32 {
        let res = self.0[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.0[1] << 9;
        self.0[2] ^= self.0[0];
        self.0[3] ^= self.0[1];
        self.0[1] ^= self.0[2];
        self.0[0] ^= self.0[3];
        self.0[2] ^= t;
        self.0[3] = self.0[3].rotate_left(11);
        res
    }
}

#[allow(unused)]
pub(crate) fn dump_keys<K: std::fmt::Debug, V, W: std::io::Write>(t: &RbsTree<K, V>, mut dst: W) {
    fn rec<K: std::fmt::Debug, V, W: std::io::Write>(
        t: &Node<K, V>,
        dst: &mut W,
    ) -> std::io::Result<()> {
        if let Some(ref node) = t.0 {
            write!(dst, "(")?;
            rec(&node.left, dst)?;
            write!(dst, " {:?} ", &node.key)?;
            rec(&node.right, dst)?;
            write!(dst, ")")
        } else {
            write!(dst, ".")
        }
    }
    rec(&t.root, &mut dst).unwrap();
    writeln!(dst);
}
