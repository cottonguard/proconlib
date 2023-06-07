const BITS: usize = usize::BITS as _;

pub struct BitSet {
    len: usize,
    buf: Box<[usize]>,
}

impl BitSet {
    pub fn zeros(len: usize) -> Self {
        Self {
            len,
            buf: vec![0; (len + BITS - 1) / BITS].into(),
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.len());
        (unsafe { self.buf.get_unchecked(i / BITS) }) >> (i % BITS) & 1 == 1
    }
    pub fn set(&mut self, i: usize, b: bool) -> bool {
        let orig = self.get(i);
        let d = unsafe { self.buf.get_unchecked_mut(i / BITS) };
        let bit = 1 << (i % BITS);
        if b {
            *d |= bit;
        } else {
            *d &= !bit;
        }
        orig
    }
    pub fn count_ones(&self) -> usize {
        self.buf.iter().map(|d| d.count_ones() as usize).sum()
    }
    pub fn range_chunks(&self, start: usize, end: usize) -> RangeChunks {
        assert!(start <= end);
        RangeChunks {
            chunks: &self.buf[start / BITS..(end + BITS - 1) / BITS],
            discard_head: start % BITS,
            discard_tail: end.wrapping_neg() % BITS,
        }
    }
}

pub struct RangeChunks<'a> {
    chunks: &'a [usize],
    discard_head: usize,
    discard_tail: usize,
}

impl<'a> Iterator for RangeChunks<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&c, rest)) = self.chunks.split_first() {
            self.chunks = rest;
            let mut c = c;
            if self.discard_head != 0 {
                c &= !0 << self.discard_head;
                self.discard_head = 0;
            }
            if rest.is_empty() {
                c &= !0 >> self.discard_tail;
            }
            Some(c)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}
impl<'a> ExactSizeIterator for RangeChunks<'a> {
    fn len(&self) -> usize {
        self.chunks.len()
    }
}
