pub struct BytesPool {
    bufs: Vec<Vec<u8>>,
    full: Vec<Vec<u8>>,
}

const BUF_SIZE: usize = 1 << 13;

impl BytesPool {
    pub fn insert(&mut self, s: &[u8]) -> &mut [u8] {
        todo!()
    }
}
