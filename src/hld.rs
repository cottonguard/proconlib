pub struct Hld<G> {
    g: G,
    nodes: Vec<Node>,
    map: Vec<(usize, usize)>,
}

struct HldBuild {
    nodes: Vec<Node>,
    map: Vec<(usize, usize)>,
    depth: usize,
}

struct Node {
    par: usize,
    path: Vec<usize>,
    depth: usize,
}

impl<G, I> Hld<G>
where
    G: FnMut(usize) -> I,
    I: IntoIterator<Item = usize>,
{
    pub fn new(g: G, root: usize) -> Self {
        HldBuild {
            nodes: vec![],
            map: vec![],
            depth: 0,
        }
        .build(g, root)
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        self.map[u];
        todo!()
    }
}

impl HldBuild {
    fn build<G, I>(mut self, mut g: G, root: usize) -> Hld<G>
    where
        G: FnMut(usize) -> I,
        I: IntoIterator<Item = usize>,
    {
        self.rec(&mut g, root, root);
        Hld {
            g,
            nodes: self.nodes,
            map: self.map,
        }
    }
    fn rec<G, I>(&mut self, g: &mut G, u: usize, p: usize) -> usize
    where
        G: FnMut(usize) -> I,
        I: IntoIterator<Item = usize>,
    {
        let mut size_u = 0;
        let mut heavy_v = 0;
        let mut size_v_max = 0;
        self.depth += 1;
        for v in g(u) {
            if v == p {
                continue;
            }
            let size_v = self.rec(g, v, u);
            size_u += size_v;
            if size_v > size_v_max {
                heavy_v = v;
                size_v_max = size_v;
            }
        }
        self.depth -= 1;
        if size_v_max == 0 {
            self.map[u] = (self.nodes.len(), 0);
            self.nodes.push(Node {
                par: p,
                path: vec![u],
                depth: self.depth,
            });
        } else {
            self.map[u] = self.map[heavy_v];
            self.map[u].1 += 1;
            let node = &mut self.nodes[self.map[u].0];
            node.path.push(u);
            node.par = p;
            node.depth -= 1;
        }
        size_u
    }
}
