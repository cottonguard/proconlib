/*use std::ops::{Index, IndexMut, Range};

type Idx = u32;

pub struct Graph {
    g: RawGraph<Idx>,
}

impl Index<usize> for Graph {
    type Output = [usize];
    fn index(&self, index: usize) -> &Self::Output {
        self.g.index(index)
    }
}

pub struct GraphBuilder {}

pub struct LabeledGraph<T> {
    g: RawGraph<(Idx, T)>,
}

impl<T> Index<usize> for LabeledGraph<T> {
    type Output = [(usize, T)];
    fn index(&self, index: usize) -> &Self::Output {
        self.g.index(index)
    }
}

pub struct RawGraph<T> {
    edges: Box<[T]>,
    heads: Box<[Idx]>,
}

impl<T> RawGraph<T> {
    fn num_node(&self) -> usize {
        self.heads.len() - 1
    }

    fn range(&self, i: usize) -> Range<usize> {
        let [l, r] = self.heads[i..i + 2];
        l as usize..r as usize
    }
}

impl<T> Index<usize> for RawGraph<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.edges[self.range(i)]
    }
}

pub struct RawGraphBuilder<T> {
    edges: Vec<(T, Idx)>,
    heads: Box<[Idx]>,
}

impl<T> RawGraphBuilder<T> {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![],
            heads: vec![!0; n].into(),
        }
    }

    fn add_edge(&mut self, src: usize, value: T) -> &mut Self {
        let head = self.heads[src];
        self.heads[src] = self.edges.len() as Idx;
        self.edges.push((value, head));
        self
    }

    fn build(self) -> Self {
        let mut edges = Vec::with_capacity(self.edges.len());
        edges.spare_capacity_mut();
    }
}
*/
