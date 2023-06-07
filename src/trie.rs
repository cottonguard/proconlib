pub mod aho_corasick;

pub use self::aho_corasick::AhoCorasick;
use std::iter::FromIterator;

type Index = u32;

#[derive(Clone, Debug)]
pub struct Trie<V> {
    g: Vec<Node>,
    dict: Vec<Option<V>>,
}

#[derive(Clone, Default, Debug)]
struct Node {
    bits: [u64; 4],
    dests: Vec<Index>,
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Self {
            g: vec![Node::default()],
            dict: vec![None],
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.g.len()
    }

    pub fn get(&self, i: usize) -> Option<&V> {
        self.dict.get(i).and_then(|v| v.as_ref())
    }

    pub fn find(&self, s: &[u8]) -> Option<&V> {
        let mut i = 0;
        for &c in s {
            if let Some(next) = self.dest(i, c) {
                i = next;
            } else {
                return None;
            }
        }
        self.dict[i].as_ref()
    }

    pub fn add(&mut self, s: &[u8], value: V) -> (usize, Option<V>) {
        let mut i = 0;
        for &c in s {
            if let Some(next) = self.dest(i, c) {
                i = next;
            } else {
                let next = self.g.len();
                let node = &mut self.g[i];
                let r = rank(node.bits, c);
                node.bits[(c / 64) as usize] |= 1 << c % 64;
                node.dests.insert(r, next as Index);
                i = next;
                self.g.push(Node::default());
                self.dict.push(None);
            }
        }
        (i, self.dict[i].replace(value))
    }

    pub fn add_node(&mut self, i: usize, c: u8, value: Option<V>) -> (usize, Option<V>) {
        if let Some(j) = self.dest(i, c) {
            (j, std::mem::replace(&mut self.dict[j], value))
        } else {
            let j = self.g.len();
            let node = &mut self.g[i];
            let r = rank(node.bits, c);
            node.bits[(c / 64) as usize] |= 1 << c % 64;
            node.dests.insert(r, j as Index);
            self.g.push(Node::default());
            self.dict.push(value);
            (j, None)
        }
    }

    #[inline]
    pub fn dest(&self, i: usize, c: u8) -> Option<usize> {
        self.g.get(i).and_then(|node| {
            if (node.bits[(c / 64) as usize] >> (c % 64)) & 1 == 0 {
                return None;
            }
            let r = rank(node.bits, c);
            Some(node.dests[r] as usize)
        })
    }

    pub fn dests(&self, i: usize) -> Dests {
        Dests {
            node: &self.g[i],
            c: Some(0),
            i: 0,
        }
    }

    pub fn cursor(&self, i: usize) -> Cursor<V> {
        Cursor { trie: self, i }
    }

    pub fn cursor_root(&self) -> Cursor<V> {
        self.cursor(0)
    }
}

#[inline]
fn rank(bits: [u64; 4], c: u8) -> usize {
    let p1: u32 = bits[..(c / 64) as usize]
        .iter()
        .map(|chunk| chunk.count_ones())
        .sum();

    let p2 = if c % 64 == 0 {
        0
    } else {
        let mask = !0 >> (64 - c % 64);
        let tail = bits[(c / 64) as usize] & mask;
        tail.count_ones()
    };
    (p1 + p2) as usize
}

impl<S: AsRef<[u8]>, T> FromIterator<(S, T)> for Trie<T> {
    fn from_iter<I: IntoIterator<Item = (S, T)>>(iter: I) -> Self {
        let mut trie = Trie::new();
        for (s, v) in iter {
            trie.add(s.as_ref(), v);
        }
        trie
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Trie::new()
    }
}

pub struct Dests<'a> {
    node: &'a Node,
    c: Option<u8>,
    i: usize,
}

impl<'a> Iterator for Dests<'a> {
    type Item = (u8, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.c?;
        let mut q = c / 64;
        let mut r = c % 64;
        loop {
            if q >= 4 {
                self.c = None;
                return None;
            }
            let masked = self.node.bits[q as usize] & (!0 << r);
            if masked != 0 {
                r = masked.trailing_zeros() as _;
                break;
            }
            q += 1;
            r = 0;
        }
        r = (self.node.bits[q as usize] & (!0 << r)).trailing_zeros() as _;
        let c = 64 * q + r;
        let res = (c, self.node.dests[self.i] as usize);
        self.c = c.checked_add(1);
        self.i += 1;
        Some(res)
    }
}

pub struct Cursor<'a, T> {
    trie: &'a Trie<T>,
    i: usize,
}

impl<'a, T> Cursor<'a, T> {
    pub fn node_id(&self) -> usize {
        self.i
    }

    pub fn get(&self) -> Option<&T> {
        self.trie.dict[self.i].as_ref()
    }

    pub fn transition(&self, c: u8) -> Option<Self> {
        self.trie
            .dest(self.i, c)
            .map(|i| Self { trie: self.trie, i })
    }

    pub fn dests(&self) -> Dests<'a> {
        self.trie.dests(self.i)
    }
}
