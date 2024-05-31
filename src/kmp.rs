pub struct Kmp<'a, T> {
    pat: &'a [T],
    back: Vec<u32>,
}

impl<'a, T: Eq> Kmp<'a, T> {
    pub fn new(pat: &'a [T]) -> Self {
        let mut res = Self {
            back: vec![0; pat.len()],
            pat,
        };
        let mut j = 0;
        for i in 1..res.pat.len() {
            j = res.eat(j, &res.pat[i]);
            res.back[i] = j as u32;
        }
        res
    }

    pub fn positions<I: IntoIterator<Item = &'a T>>(&self, s: I) -> Positions<T, I::IntoIter> {
        Positions {
            kmp: self,
            state: 0,
            position: 0,
            s: s.into_iter(),
        }
    }

    pub fn position<I: IntoIterator<Item = &'a T>>(&'a self, s: I) -> Option<usize> {
        self.positions(s).next()
    }

    fn eat(&self, mut state: usize, c: &T) -> usize {
        loop {
            if Some(c) == self.pat.as_ref().get(state) {
                state += 1;
                break;
            }
            if state == 0 {
                break;
            }
            state = self.back[state - 1] as usize;
        }
        state
    }

    pub fn state(&self, id: usize) -> State<T> {
        State {
            kmp: self,
            state: id,
        }
    }
}

pub struct State<'a, T> {
    kmp: &'a Kmp<'a, T>,
    state: usize,
}

impl<'a, T: Eq> State<'a, T> {
    pub fn eat(&mut self, c: &T) -> &mut Self {
        self.state = self.kmp.eat(self.state, c);
        self
    }

    pub fn eat_iter<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) -> &mut Self {
        for c in iter {
            self.eat(c);
        }
        self
    }

    pub fn state(&self) -> usize {
        self.state
    }
}

pub struct Positions<'a, T, I> {
    kmp: &'a Kmp<'a, T>,
    state: usize,
    position: usize,
    s: I,
}

impl<'a, T: Eq, I: Iterator<Item = &'a T>> Iterator for Positions<'a, T, I> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.s.next() {
            self.position += 1;
            self.state = self.kmp.eat(self.state, c);
            if self.state == self.kmp.pat.len() {
                return Some(self.position - self.kmp.pat.len());
            }
        }
        None
    }
}
