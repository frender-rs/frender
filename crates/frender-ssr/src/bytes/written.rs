use super::AsyncWritableBytes;

pub struct BytesWritten<'a, B: AsRef<[u8]>>(pub B, pub &'a mut usize);

impl<'a, B: AsRef<[u8]>> AsRef<[u8]> for BytesWritten<'a, B> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0.as_ref()[*self.1..]
    }
}

impl<'a, B: AsRef<[u8]>> AsyncWritableBytes for BytesWritten<'a, B> {
    fn truncate_start_at(&mut self, n: usize) {
        *self.1 = n
    }
}
