/*

pub struct BitVec {
    buf: Vec<usize>,
    len: usize,
}

const BITS: usize = usize::BITS as usize;

#[inline]
const fn div(i: usize) -> usize {
    i / BITS
}

#[inline]
const fn rem(i: usize) -> usize {
    i % BITS
}

impl BitVec {
    pub fn zeros(len: usize) -> Self {
        Self {
            len,
            buf: vec![0; (len + BITS - 1) / BITS],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> bool {
        (self.get_chunk(i) >> rem(i)) & 1 == 1
    }

    pub fn set(&mut self, i: usize, f: bool) -> bool {
        let orig = self.get(i);
        let c = self.get_chunk_mut(i);
        if f {
            *c |= 1 << rem(i);
        } else {
            *c &= !(1 << rem(i));
        }
        orig
    }

    pub fn range_chunks(&self, start: usize, end: usize) -> RangeChunks {
        assert!(start <= end);
        assert!(end <= self.len());
    }

    #[inline]
    fn get_chunk(&self, i: usize) -> usize {
        self.assert_chunk(i);
        unsafe { *self.buf.get_unchecked(div(i)) }
    }

    #[inline]
    fn get_chunk_mut(&mut self, i: usize) -> &mut usize {
        self.assert_chunk(i);
        unsafe { self.buf.get_unchecked_mut(div(i)) }
    }

    #[inline]
    fn assert_chunk(&self, i: usize) {
        assert!(
            i < self.len(),
            "out of range (index = {i}, len = {})",
            self.len()
        );
    }
}

pub struct RangeChunks<'a> {
    front_mask: usize,
    back_mask: usize,
    inner: std::slice::Iter<'a, usize>,
}

impl<'a> Iterator for RangeChunks<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&c) = self.inner.next() {
            let mut c = c & self.start_mask;
            if self.inner.as_slice().is_empty() {
                c &= self.end_mask;
                self.end_mask = !0;
            }
            Some(c)
        } else {
            None
        }
    }
}

 */
