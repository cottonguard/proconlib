use super::scc;

pub fn two_sat<I: IntoIterator<Item = (usize, bool, usize, bool)>>(
    num_vars: usize,
    clauses: I,
) -> Option<Assignment> {
    let mut ts = TwoSat::new(num_vars);
    ts.extend(clauses);
    ts.run()
}

pub struct TwoSat {
    g: Vec<Vec<usize>>,
}

impl TwoSat {
    pub fn new(num_vars: usize) -> Self {
        Self {
            g: vec![vec![]; 2 * num_vars],
        }
    }

    pub fn add_clause(&mut self, x: usize, f: bool, y: usize, g: bool) {
        let u = (x << 1) | f as usize;
        let v = (y << 1) | g as usize;
        self.g[u ^ 1].push(v);
        self.g[v ^ 1].push(u);
    }

    pub fn run(&self) -> Option<Assignment> {
        let (_m, scc) = scc::scc(self.g.len(), |u| self.g[u].iter().copied());
        if scc.chunks_exact(2).rev().all(|c| c[0] != c[1]) {
            Some(Assignment { scc, i: 0 })
        } else {
            None
        }
    }
}

impl Extend<(usize, bool, usize, bool)> for TwoSat {
    fn extend<T: IntoIterator<Item = (usize, bool, usize, bool)>>(&mut self, clauses: T) {
        for (x, f, y, g) in clauses {
            self.add_clause(x, f, y, g);
        }
    }
}

pub struct Assignment {
    scc: Vec<usize>,
    i: usize,
}

impl Iterator for Assignment {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if let Some(c) = self.scc.get(self.i..self.i + 2) {
            debug_assert_ne!(c[0], c[1]);
            self.i += 2;
            Some(c[0] <= c[1])
        } else {
            None
        }
    }
}
