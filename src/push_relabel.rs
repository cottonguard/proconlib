pub struct PushRelabel {
    g: Vec<Vec<Edge>>,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    dest: u32,
    rev: u32,
    cap: u64,
}

impl Edge {
    fn new(dest: usize, rev: usize, cap: u64) -> Self {
        Self {
            dest: dest as _,
            rev: rev as _,
            cap,
        }
    }
}

impl PushRelabel {
    pub fn new(n: usize) -> Self {
        Self { g: vec![vec![]; n] }
    }

    pub fn edge(&mut self, u: usize, v: usize, cap: u64) {
        let iu = self.g[u].len();
        let iv = self.g[v].len();
        self.g[u].push(Edge::new(v, iv, cap));
        self.g[v].push(Edge::new(u, iu, 0));
        /*
        EdgeToken {
            node: u as _,
            edge: iu as _,
        }
         */
    }

    pub fn run(&mut self, s: usize, t: usize) -> u64 {
        // use crate::*;
        let mut label = vec![0; self.g.len()];
        label[s] = self.g.len() as u32;
        let mut excess = vec![0; self.g.len()];
        let mut cur = vec![!0; self.g.len()];
        let mut buckets = Buckets::new(2 * self.g.len() - 1, self.g.len());
        for cur in 0..self.g[s].len() {
            let Edge {
                dest,
                rev,
                ref mut cap,
            } = self.g[s][cur];
            let v = dest as usize;
            let flow = *cap;
            *cap = 0;
            self.g[v][rev as usize].cap = flow;
            if excess[v] == 0 && v != t {
                buckets.push(0, v as _);
            }
            excess[v] += flow;
        }
        while let Some(u) = buckets.pop() {
            let u = u as usize;
            // d!(&excess);
            while excess[u] > 0 {
                if cur[u] < self.g[u].len() {
                    let e = &mut self.g[u][cur[u]];
                    let v = e.dest as usize;
                    if label[u] != label[v] + 1 {
                        cur[u] += 1;
                        continue;
                    }
                    let flow = e.cap.min(excess[u]);
                    e.cap -= flow;
                    excess[u] -= flow;
                    if e.cap == 0 {
                        cur[u] += 1;
                    }
                    let rev = e.rev as usize;
                    self.g[v][rev].cap += flow;
                    if excess[v] == 0 && v != s && v != t {
                        buckets.push(label[v], v as _);
                    }
                    excess[v] += flow;
                } else {
                    label[u] = !0;
                    for (i, e) in self.g[u].iter().enumerate() {
                        if e.cap > 0 && label[e.dest as usize] + 1 < label[u] {
                            label[u] = label[e.dest as usize] + 1;
                            cur[u] = i;
                        }
                    }
                    debug_assert_ne!(label[u], !0);
                }
            }
        }
        excess[t]
    }
}

struct Buckets {
    front: Vec<u32>,
    back: Vec<u32>,
    next: Vec<u32>,
    cur: u32,
}

impl Buckets {
    fn new(n_buckets: usize, n_nodes: usize) -> Self {
        Self {
            front: vec![!0; n_buckets],
            back: vec![!0; n_buckets],
            next: vec![!0; n_nodes],
            cur: 0,
        }
    }

    fn push(&mut self, label: u32, u: u32) {
        if self.next[u as usize] != !0 {
            return;
        }
        if self.front[label as usize] == !0 {
            self.front[label as usize] = u;
            self.back[label as usize] = u;
        } else {
            self.next[self.back[label as usize] as usize] = u;
            self.back[label as usize] = u;
        }
        self.cur = self.cur.max(label);
    }

    fn pop(&mut self) -> Option<u32> {
        loop {
            let u = self.front[self.cur as usize];
            if u != !0 {
                self.front[self.cur as usize] = self.next[u as usize];
                if self.next[u as usize] == !0 {
                    self.back[self.cur as usize] = !0;
                }
                self.next[u as usize] = !0;
                return Some(u);
            }
            if self.cur == 0 {
                return None;
            }
            self.cur -= 1;
        }
    }
}
