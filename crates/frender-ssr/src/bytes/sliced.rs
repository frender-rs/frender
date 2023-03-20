pub struct SlicedBytes {
    inner: Vec<u8>,
    start: usize,
}

impl From<Vec<u8>> for SlicedBytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            inner: value,
            start: 0,
        }
    }
}

impl AsRef<[u8]> for SlicedBytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner[self.start..]
    }
}

impl super::AsyncWritableBytes for SlicedBytes {
    fn truncate_start_at(&mut self, n: usize) {
        assert!(n + self.start <= self.inner.len());
        self.start += n;
    }
}
