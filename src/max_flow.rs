use std::collections::VecDeque;

pub struct MaxFlow {
    g: Vec<Vec<Edge>>,
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeToken {
    node: u32,
    edge: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeState {
    pub u: usize,
    pub v: usize,
    pub capacity: u64,
    pub flow: u64,
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

#[derive(Debug)]
struct State {
    time: u32,
    nodes: Vec<NodeState>,
    t: usize,
}

#[derive(Clone, Copy, Default, Debug)]
struct NodeState {
    time: u32,
    dist: u32,
    cur_edge: u32,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        Self { g: vec![vec![]; n] }
    }

    pub fn edge(&mut self, u: usize, v: usize, cap: u64) -> EdgeToken {
        let iu = self.g[u].len();
        let iv = self.g[v].len();
        self.g[u].push(Edge::new(v, iv, cap));
        self.g[v].push(Edge::new(u, iu, 0));
        EdgeToken {
            node: u as _,
            edge: iu as _,
        }
    }

    pub fn edge_state(&self, token: EdgeToken) -> EdgeState {
        let u = token.node as usize;
        let euv = self.g[u][token.edge as usize];
        let v = euv.dest as usize;
        let evu = self.g[v][euv.rev as usize];
        EdgeState {
            u,
            v,
            capacity: euv.cap + evu.cap,
            flow: evu.cap,
        }
    }

    pub fn run(&mut self, s: usize, t: usize) -> u64 {
        let mut que = VecDeque::new();
        let mut state = State {
            time: 0,
            nodes: vec![NodeState::default(); self.g.len()],
            t,
        };
        let mut flow = 0;
        loop {
            state.time += 1;
            state.nodes[s] = NodeState {
                time: state.time,
                dist: 0,
                cur_edge: 0,
            };
            que.clear();
            que.push_back(s as u32);
            'bfs: while let Some(u) = que.pop_front() {
                let u = u as usize;
                let dist_u = state.nodes[u].dist;
                for e in &self.g[u] {
                    let state_v = &mut state.nodes[e.dest as usize];
                    if e.cap > 0 && state_v.time < state.time {
                        *state_v = NodeState {
                            time: state.time,
                            dist: dist_u + 1,
                            cur_edge: 0,
                        };
                        if e.dest as usize == t {
                            break 'bfs;
                        }
                        que.push_back(e.dest);
                    }
                }
            }
            if state.nodes[t].time < state.time {
                break;
            }
            flow += self.dfs(&mut state, s, u64::max_value());
        }
        flow
    }

    fn dfs(&mut self, state: &mut State, u: usize, cap: u64) -> u64 {
        if u == state.t {
            return cap;
        }

        let mut cur_edge = state.nodes[u].cur_edge as usize;
        let mut flow = 0;
        while cap > flow && cur_edge < self.g[u].len() {
            let e = self.g[u][cur_edge];
            let v = e.dest as usize;
            let rev = e.rev as usize;
            if e.cap > 0
                && state.nodes[v].time == state.time
                && state.nodes[u].dist + 1 == state.nodes[v].dist
            {
                let flow_vt = self.dfs(state, v, (cap - flow).min(e.cap));
                flow += flow_vt;
                self.g[u][cur_edge].cap -= flow_vt;
                self.g[v][rev].cap += flow_vt;
            }
            cur_edge += 1;
        }
        state.nodes[u].cur_edge = cur_edge as _;
        if flow == 0 {
            state.nodes[u].dist = !0;
        }
        flow
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut que = VecDeque::new();
        que.push_back(s as u32);
        let mut visited = vec![false; self.g.len()];
        visited[s] = true;
        while let Some(u) = que.pop_front() {
            for e in &self.g[u as usize] {
                if e.cap > 0 && !visited[e.dest as usize] {
                    que.push_back(e.dest);
                    visited[e.dest as usize] = true;
                }
            }
        }
        visited
    }
}
