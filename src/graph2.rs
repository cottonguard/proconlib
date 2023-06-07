use std::{collections::VecDeque, iter, ops::Range, slice};

pub trait Graph {
    type Node: Copy;
    type Nodes<'a>: Iterator<Item = Self::Node> + ExactSizeIterator
    where
        Self: 'a;
    type Neighbors<'a>: Iterator<Item = Self::Node>
    where
        Self: 'a;

    fn nodes(&self) -> Self::Nodes<'_>;
    fn neighbors(&self, u: Self::Node) -> Self::Neighbors<'_>;

    fn num_nodes(&self) -> usize {
        self.nodes().len()
    }
}

pub trait IndexedGraph: Graph<Node = usize> {
    fn dist_bfs(&self, s: usize) -> Vec<usize> {
        dist_bfs(self, s)
    }

    fn tsort(&self) -> Option<Vec<usize>> {
        tsort(self)
    }
}

impl<T: Graph<Node = usize> + ?Sized> IndexedGraph for T {}

type Neighbors<'a> = iter::Copied<slice::Iter<'a, usize>>;

impl Graph for [Vec<usize>] {
    type Node = usize;
    type Nodes<'a> = Range<usize>
    where
    Self: 'a;
    type Neighbors<'a> = Neighbors<'a>
    where
    Self: 'a;

    fn nodes(&self) -> Self::Nodes<'_> {
        0..self.len()
    }

    fn neighbors(&self, u: Self::Node) -> Self::Neighbors<'_> {
        self[u].iter().copied()
    }
}

impl<T> Graph for [Vec<(usize, T)>] {
    type Node = usize;
    type Nodes<'a> = Range<usize>
    where
    Self: 'a;
    type Neighbors<'a> = iter::Map<slice::Iter<'a, (usize, T)>, fn (&'a (usize, T)) -> usize>
    where
    Self: 'a;

    fn nodes(&self) -> Self::Nodes<'_> {
        0..self.len()
    }

    fn neighbors(&self, u: Self::Node) -> Self::Neighbors<'_> {
        self[u].iter().map(|(u, _)| *u)
    }
}

fn dist_bfs<G: IndexedGraph + ?Sized>(g: &G, u: usize) -> Vec<usize> {
    let mut que = VecDeque::new();
    que.push_back(u);
    let mut dist = vec![!0; g.num_nodes()];
    dist[u] = 0;
    while let Some(u) = que.pop_front() {
        for v in g.neighbors(u) {
            if dist[v] == !0 {
                dist[v] = dist[u] + 1;
                que.push_back(v);
            }
        }
    }
    dist
}

const fn ceil_div(x: usize, y: usize) -> usize {
    x / y + (x % y != 0) as usize
}

fn tsort<G: IndexedGraph + ?Sized>(g: &G) -> Option<Vec<usize>> {
    fn dfs_neighbors<G: IndexedGraph + ?Sized>(
        g: &G,
        res: &mut Vec<usize>,
        states: &mut [usize],
        u: usize,
    ) -> bool {
        for v in g.neighbors(u) {
            if !visit(g, res, states, v) {
                return false;
            }
        }
        res.push(u);
        true
    }

    #[inline]
    fn visit<G: IndexedGraph + ?Sized>(
        g: &G,
        res: &mut Vec<usize>,
        states: &mut [usize],
        v: usize,
    ) -> bool {
        let i = 2 * v / usize::BITS as usize;
        let j = 2 * v % usize::BITS as usize;
        let state = (states[i] >> j) & 3;
        if state & 1 != 0 {
            return state & 2 != 0;
        }
        states[i] |= 1 << j;
        if !dfs_neighbors(g, res, states, v) {
            return false;
        }
        states[i] |= 1 << (j + 1);
        true
    }

    let mut res = Vec::with_capacity(g.num_nodes());
    let mut states = vec![0usize; ceil_div(2 * g.num_nodes(), usize::BITS as usize)];
    for s in g.nodes() {
        if !visit(g, &mut res, &mut states, s) {
            return None;
        }
    }
    res.reverse();
    Some(res)
}
