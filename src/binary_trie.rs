pub struct BinaryTrie(Vec<[usize; 2]>);
impl BinaryTrie {
    pub fn new() -> Self {
        Self(vec![[!0; 2]])
    }
    fn height() -> usize {
        8 * std::mem::size_of::<u32>()
    }
    pub fn add(&mut self, x: u32) -> bool {
        let mut i = 0;
        let mut added = false;
        for j in (0..Self::height()).rev() {
            let f = (x >> j & 1) as usize;
            if self.0[i][f] > !0 >> 1 {
                added = true;
                self.0[i][f] = if self.0[i][f] == !0 {
                    self.0.push([!0; 2]);
                    self.0.len() - 1
                } else {
                    !self.0[i][f]
                };
            }
            i = self.0[i][f];
        }
        added
    }
    pub fn remove(&mut self, x: u32) -> bool {
        let mut path = vec![0; Self::height()];
        let mut i = 0;
        for j in (0..Self::height()).rev() {
            path[j] = i;
            let f = (x >> j & 1) as usize;
            if self.0[i][f] > !0 >> 1 {
                return false;
            }
            i = self.0[i][f];
        }
        for j in 0..Self::height() {
            let i = path[j];
            let f = (x >> j & 1) as usize;
            self.0[i][f] = !self.0[i][f];
            if self.0[i][f ^ 1] <= !0 >> 1 {
                break;
            }
        }
        true
    }
    pub fn xor_min(&self, mut x: u32) -> u32 {
        let mut i = 0;
        for j in (0..Self::height()).rev() {
            let mut f = (x >> j & 1) as usize;
            if self.0[i][f] > !0 >> 1 {
                f ^= 1;
            }
            x ^= (f as u32) << j;
            i = self.0[i][f];
        }
        x
    }
}
