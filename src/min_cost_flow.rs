use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
pub struct MinCostFlow {
    g: Vec<Vec<Edge>>,
    dual: Vec<i64>,
    dist: Vec<i64>,
    pque: BinaryHeap<Reverse<(i64, usize)>>,
    prev: Vec<(u32, u32)>,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    dest: u32,
    rev: u32,
    cap: u64,
    weight: i64,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            dual: vec![0; n],
            dist: vec![i64::max_value(); n],
            pque: BinaryHeap::new(),
            prev: vec![(!0, !0); n],
        }
    }

    pub fn edge(&mut self, u: usize, v: usize, cap: u64, weight: i64) -> &mut Self {
        let iu = self.g[u].len() as _;
        let iv = self.g[v].len() as _;
        self.g[u].push(Edge {
            dest: v as _,
            rev: iv,
            cap,
            weight,
        });
        self.g[v].push(Edge {
            dest: u as _,
            rev: iu,
            cap: 0,
            weight: -weight,
        });
        self
    }

    pub fn run(&mut self, s: usize, t: usize, limit: u64) -> (u64, i64) {
        let mut flow = 0;
        let mut cost = 0;
        while flow < limit {
            let (add_flow, add_cost) = self.single_flow(s, t, limit - flow);
            if add_flow == 0 {
                break;
            }
            flow += add_flow;
            cost += add_cost;
        }
        (flow, cost)
    }

    fn single_flow(&mut self, s: usize, t: usize, limit: u64) -> (u64, i64) {
        self.dist[s] = 0;
        self.pque.clear();
        self.pque.push(Reverse((0, s)));
        while let Some(Reverse((dist_u, u))) = self.pque.pop() {
            if dist_u > self.dist[u] {
                continue;
            }
            debug_assert_eq!(dist_u, self.dist[u]);
            if u == t {
                break;
            }
            for (i, e) in self.g[u].iter().enumerate() {
                if e.cap > 0 {
                    let v = e.dest as usize;
                    let dist_v = dist_u + e.weight - self.dual[v] + self.dual[u];
                    if dist_v < self.dist[v] {
                        self.dist[v] = dist_v;
                        self.prev[v] = (u as _, i as _);
                        self.pque.push(Reverse((dist_v, v)));
                    }
                }
            }
        }
        if self.dist[t] == i64::max_value() {
            return (0, 0);
        }
        let dist_t = self.dist[t];
        for (dual, dist) in self.dual.iter_mut().zip(self.dist.iter_mut()) {
            *dual += (*dist).min(dist_t);
            *dist = i64::max_value();
        }
        let mut u = t;
        let mut flow = limit;
        while u != s {
            let (v, i) = self.prev[u];
            flow = flow.min(self.g[v as usize][i as usize].cap);
            u = v as usize;
        }
        debug_assert!(flow > 0);
        let mut u = t;
        let mut cost = 0;
        while u != s {
            let (v, i) = self.prev[u];
            let e = &mut self.g[v as usize][i as usize];
            e.cap -= flow;
            cost += flow as i64 * e.weight;
            let rev = e.rev as usize;
            self.g[u][rev].cap += flow;
            u = v as usize;
        }
        (flow, cost)
    }
}
