type Idx = u32;
const NIL: Idx = !0;

pub struct AdjList<T> {
    heads: Vec<Idx>,
    tails: Vec<Idx>,
    edges: Vec<EdgeData<T>>,
}

pub struct EdgeData<T> {
    dst: Idx,
    next: Idx,
    label: T,
}

#[derive(Clone, Copy /*, Debug*/)]
pub struct Edge<'a, T> {
    g: &'a AdjList<T>,
    i: Idx,
}

impl<T> AdjList<T> {
    pub fn num_nodes(&self) -> usize {
        self.heads.len()
    }

    pub fn edge(&self, i: usize) -> Edge<T> {
        assert!(i < self.edges.len());
        Edge {
            g: self,
            i: i as Idx,
        }
    }

    pub fn out_edges(&self, u: usize) -> OutEdges<T> {
        self.assert_node_idx(u);
        let i = self.heads[u];
        OutEdges {
            e: (i != NIL).then(|| Edge { g: self, i }),
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, label: T) -> Edge<T> {
        self.assert_node_idx(u);
        let idx = self.edges.len() as Idx;
        if let Some(last) = self.edges.get_mut(self.tails[u] as usize) {
            last.next = idx;
        } else {
            self.heads[u] = idx;
        }
        self.tails[u] = idx;
        self.edges.push(EdgeData {
            dst: v as Idx,
            next: NIL,
            label,
        });
        Edge { g: self, i: idx }
    }

    #[inline]
    fn assert_node_idx(&self, u: usize) {
        assert!(
            u < self.heads.len(),
            "out of range (nodes = {}, index = {})",
            self.heads.len(),
            u
        );
        if self.heads.len() != self.tails.len() {
            unsafe { std::hint::unreachable_unchecked() }
        }
    }
}

impl<'a, T> Edge<'a, T> {
    fn new(g: &'a AdjList<T>, i: Idx) -> Self {
        Self { g, i }
    }
    fn data(&self) -> &EdgeData<T> {
        unsafe { self.g.edges.get_unchecked(self.i as usize) }
    }
    pub fn id(&self) -> usize {
        self.i as usize
    }
    pub fn dst(&self) -> usize {
        self.data().dst as usize
    }
    pub fn label(&self) -> &T {
        &self.data().label
    }
    pub fn next(&self) -> Option<Edge<'a, T>> {
        let i = self.data().next;
        (i != NIL).then(|| Self::new(self.g, i))
    }
}

pub struct OutEdges<'a, T> {
    e: Option<Edge<'a, T>>,
}

impl<'a, T> Iterator for OutEdges<'a, T> {
    type Item = Edge<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.e.take() {
            self.e = e.next();
            Some(e)
        } else {
            None
        }
    }
}
