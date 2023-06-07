use crate::graph::*;

pub fn tsort<'a, G: Graph<'a>>(g: &'a G) -> Option<Vec<usize>> {
    TSort::new(g).run()
}

struct TSort<'a, G> {
    g: &'a G,
    res: Vec<usize>,
    state: Vec<u8>,
}

impl<'a, G: Graph<'a>> TSort<'a, G> {
    fn new(g: &'a G) -> Self {
        let n = g.num_verts();
        Self {
            g,
            res: Vec::with_capacity(n),
            state: vec![0; (n + 3) / 4],
        }
    }

    fn run(mut self) -> Option<Vec<usize>> {
        let n = self.g.num_verts();
        for s in self.g.verts().rev() {
            if self.state(s) == 0 {
                if !self.dfs(s) {
                    return None;
                }
            }
        }
        debug_assert_eq!(self.res.len(), n);
        self.res.reverse();
        Some(self.res)
    }

    fn dfs(&mut self, u: usize) -> bool {
        debug_assert_eq!(self.state(u), 0);
        self.set_state(u, 1);
        for v in self.g.adj(u) {
            match self.state(v) {
                0 => {
                    if !self.dfs(v) {
                        return false;
                    }
                }
                1 => return false,
                _ => {}
            }
        }
        self.res.push(u);
        self.set_state(u, 2);
        true
    }

    #[inline]
    fn state(&self, u: usize) -> u8 {
        (self.state[u / 4] >> 2 * (u % 4)) & 3
    }

    #[inline]
    fn set_state(&mut self, u: usize, s: u8) {
        self.state[u / 4] |= s << 2 * (u % 4);
    }
}
