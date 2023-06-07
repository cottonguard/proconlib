use std::{iter, ops, ptr, slice};

pub trait Graph<T>
where
    for<'a> Self: GraphRef<'a, T>,
{
}

pub trait GraphRef<'a, T> {
    type Adj: Iterator<Item = usize>;

    fn adj(&'a self, v: usize) -> Self::Adj;

    fn num_verteces(&self) -> usize;

    fn num_edges(&self) -> usize;
}

trait LabeledGraphRef<'a, T: 'a> {
    type LabeledAdj: Iterator<Item = (usize, &'a T)>;

    fn labeled_adj(&'a self) -> Self::LabeledAdj;
}

#[derive(Clone, Default)]
pub struct AdjList<T> {
    indeces: Vec<usize>,
    edges: Vec<usize>,
    labels: Vec<T>,
}

impl<T> AdjList<T> {
    pub fn len(&self) -> usize {
        self.indeces.len() - 1
    }
}

impl<T> ops::Index<usize> for AdjList<T> {
    type Output = [usize];

    fn index(&self, i: usize) -> &Self::Output {
        &self.edges[self.indeces[i]..self.indeces[i + 1]]
    }
}

impl<'a, T> GraphRef<'a, T> for AdjList<T> {
    type Adj = Adj<'a>;

    fn adj(&'a self, v: usize) -> Self::Adj {
        Adj(self[v].iter())
    }

    fn num_verteces(&self) -> usize {
        self.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Clone, Debug)]
pub struct Adj<'a>(std::slice::Iter<'a, usize>);

impl<'a> Iterator for Adj<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Adj<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().copied()
    }
}

impl<'a> ExactSizeIterator for Adj<'a> {}

pub struct LabeledAdj<'a, T> {
    edges: &'a [usize],
    labels: &'a [T],
}

pub struct AdjListBuilder<T> {
    heads: Vec<usize>,
    edges: Vec<(usize, usize)>,
    labels: Vec<T>,
}

const NIL: usize = !0;

impl AdjListBuilder<()> {
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.add_labeled_edge(u, v, ());
    }

    pub fn add_bi_edge(&mut self, u: usize, v: usize) {
        self.add_labeled_bi_edge(u, v, ());
    }
}

impl<T> AdjListBuilder<T> {
    pub fn new(num_verteces: usize) -> Self {
        Self::with_capacity(num_verteces, 0)
    }

    pub fn with_capacity(num_verteces: usize, cap_edges: usize) -> Self {
        let mut heads = Vec::with_capacity(num_verteces + 1);
        heads.resize(num_verteces, NIL);
        Self {
            heads,
            edges: Vec::with_capacity(cap_edges),
            labels: Vec::with_capacity(cap_edges),
        }
    }

    pub fn num_verteces(&self) -> usize {
        self.heads.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn add_labeled_edge(&mut self, u: usize, v: usize, label: T) {
        self.edges.push((v, self.heads[u]));
        self.heads[u] = self.edges.len() - 1;
        self.labels.push(label);
    }

    pub fn add_labeled_bi_edge(&mut self, u: usize, v: usize, label: T)
    where
        T: Clone,
    {
        self.add_labeled_edge(u, v, label.clone());
        self.add_labeled_edge(v, u, label);
    }

    pub fn build(mut self) -> AdjList<T> {
        let mut edges = Vec::<usize>::with_capacity(self.edges.len());
        let edges_ptr = edges.as_mut_ptr();
        let mut labels = Vec::<T>::with_capacity(self.labels.len());
        let labels_ptr = labels.as_mut_ptr();
        let mut rest = self.num_edges();
        for u in (0..self.num_verteces()).rev() {
            let mut i = self.heads[u];
            while let Some(&(v, next_i)) = self.edges.get(i) {
                unsafe {
                    rest -= 1;
                    *edges_ptr.add(rest) = v;
                    ptr::copy_nonoverlapping(self.labels.get_unchecked(i), labels_ptr.add(rest), 1);
                }
                i = next_i;
            }
            self.heads[u] = i;
        }
        self.heads.push(self.num_edges());
        debug_assert_eq!(rest, 0);
        unsafe {
            let num_edges = self.num_edges();
            self.labels.set_len(0);
            edges.set_len(num_edges);
            labels.set_len(num_edges);
        }
        AdjList {
            indeces: self.heads,
            edges,
            labels,
        }
    }
}
