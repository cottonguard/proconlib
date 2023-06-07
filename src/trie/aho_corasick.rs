use super::*;
use std::collections::VecDeque;
use std::iter::FromIterator;
pub struct AhoCorasick<T> {
    trie: Trie<T>,
    suf_links: Vec<Index>,
    dict_suf_links: Vec<Index>,
    depth: Vec<Index>,
}

impl<T> AhoCorasick<T> {
    pub fn new(trie: Trie<T>) -> Self {
        let mut ac = Self {
            suf_links: vec![0; trie.num_nodes()],
            dict_suf_links: vec![!0; trie.num_nodes()],
            depth: vec![0; trie.num_nodes()],
            trie,
        };
        let mut que = VecDeque::new();
        que.push_back(0);
        while let Some(i) = que.pop_front() {
            for (c, j) in ac.trie.dests(i) {
                if i != 0 {
                    let s = ac.dest(ac.suf_links[i] as usize, c).0;
                    ac.suf_links[j] = s as Index;
                    ac.dict_suf_links[j] = if ac.trie.get(s).is_some() {
                        s as Index
                    } else {
                        ac.dict_suf_links[s]
                    };
                }
                ac.depth[j] = ac.depth[i] + 1;
                que.push_back(j);
            }
        }
        ac
    }

    #[inline]
    pub fn dest(&self, mut i: usize, c: u8) -> (usize, bool) {
        let mut back = false;
        loop {
            if let Some(j) = self.trie.dest(i, c) {
                return (j, back);
            }
            back = true;
            if i == 0 {
                return (0, true);
            }
            i = self.suf_links[i] as usize;
        }
    }

    pub fn trie(&self) -> &Trie<T> {
        &self.trie
    }

    pub fn find_longest_suffix<'a, 'b>(&'a self, s: &'b [u8]) -> FindLongestSuffix<'a, 'b, T> {
        FindLongestSuffix {
            ac: self,
            s,
            i: 0,
            j: 0,
        }
    }

    pub fn suffixes(&self, s: &[u8]) -> Suffixes<T> {
        let mut i = 0;
        for &c in s {
            i = self.dest(i, c).0;
        }
        self.suffixes_by_node_id(i)
    }

    pub fn suffixes_by_node_id(&self, i: usize) -> Suffixes<T> {
        Suffixes {
            ac: self,
            i: i as Index,
        }
    }

    pub fn suffixes_all(&self, _i: usize) {
        todo!();
    }
}

impl<T> From<Trie<T>> for AhoCorasick<T> {
    fn from(value: Trie<T>) -> Self {
        Self::new(value)
    }
}

impl<S: AsRef<[u8]>, T> FromIterator<(S, T)> for AhoCorasick<T> {
    fn from_iter<I: IntoIterator<Item = (S, T)>>(iter: I) -> Self {
        AhoCorasick::new(Trie::from_iter(iter))
    }
}

pub struct FindLongestSuffix<'a, 'b, T> {
    ac: &'a AhoCorasick<T>,
    s: &'b [u8],
    i: Index,
    j: Index,
}

impl<'a, 'b, T> Iterator for FindLongestSuffix<'a, 'b, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let res = self
                .ac
                .trie
                .get(self.i as usize)
                .map(|v| (self.i, v))
                .or_else(|| {
                    self.ac
                        .dict_suf_links
                        .get(self.i as usize)
                        .and_then(|&i| self.ac.trie.get(i as usize).map(|v| (i, v)))
                })
                .map(|(i, v)| {
                    (
                        (self.j - self.ac.depth[i as usize]) as usize,
                        self.j as usize,
                        v,
                    )
                });

            let c = if let Some(&c) = self.s.get(self.j as usize) {
                c
            } else {
                self.i = !0;
                return res;
            };

            self.j += 1;
            self.i = self.ac.dest(self.i as usize, c).0 as Index;

            if res.is_some() {
                return res;
            }
        }
    }
}

pub struct Suffixes<'a, T> {
    ac: &'a AhoCorasick<T>,
    i: Index,
}

impl<'a, T> Iterator for Suffixes<'a, T> {
    type Item = (usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.i == !0 {
                return None;
            }

            let i = self.i as usize;
            self.i = self.ac.dict_suf_links[i];
            if let Some(v) = self.ac.trie.get(i) {
                return Some((i, v));
            }
        }
    }
}
