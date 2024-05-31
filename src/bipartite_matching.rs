use std::collections::VecDeque;

const NIL: Idx = !0;
const INF: Idx = !0;

type Idx = u32;

pub fn bipartite_matching(g: &[Vec<usize>], n_end: usize) -> Vec<(usize, usize)> {
    let mut f = BipertiteMatching::new(g, n_end);
    f.run();
    f.matches()
}

struct BipertiteMatching<'a> {
    g: &'a [Vec<usize>],
    rtol: Vec<Idx>,
    dist: Vec<Idx>,
    que: VecDeque<Idx>,
}

impl<'a> BipertiteMatching<'a> {
    #[inline]
    fn new(g: &'a [Vec<usize>], n_end: usize) -> Self {
        let n = g.len();
        Self {
            g,
            rtol: vec![NIL; n_end],
            dist: vec![0; n],
            que: VecDeque::new(),
        }
    }

    #[inline]
    fn run(&mut self) {
        loop {
            if !self.bfs() {
                break;
            }
            for l in 0..self.g.len() {
                if self.dist[l] == 0 && self.dfs(l as Idx) {
                    self.dist[l] = INF;
                }
            }
        }
    }

    #[inline]
    fn matches(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (r, &l) in self.rtol.iter().enumerate() {
            if l != NIL {
                res.push((l as usize, r));
            }
        }
        res
    }

    #[inline]
    fn bfs(&mut self) -> bool {
        self.que.clear();
        for l in 0..self.g.len() {
            if self.dist[l] == 0 {
                self.que.push_back(l as Idx);
            } else {
                self.dist[l] = INF;
            }
        }
        let mut reached = false;
        while let Some(l) = self.que.pop_front() {
            for &r in &self.g[l as usize] {
                let lb = self.rtol[r];
                reached |= lb == NIL;
                if lb != NIL && self.dist[lb as usize] == INF {
                    self.que.push_back(lb);
                    self.dist[lb as usize] = self.dist[l as usize] + 1;
                }
            }
        }
        reached
    }

    #[inline]
    fn dfs(&mut self, l: Idx) -> bool {
        let dist_l = self.dist[l as usize];
        self.dist[l as usize] = if dist_l != 0 { INF } else { 0 };
        for &r in &self.g[l as usize] {
            let lb = self.rtol[r];
            if lb == NIL || (dist_l + 1 == self.dist[lb as usize] && self.dfs(lb)) {
                self.rtol[r] = l;
                return true;
            }
        }
        false
    }
}
