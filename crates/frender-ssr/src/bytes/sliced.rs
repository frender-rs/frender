use std::{
    borrow::{Borrow, BorrowMut},
    ops::RangeFrom,
};

pub struct SlicedBytes<
    Inner: Borrow<[u8]> = Vec<u8>,
    Range: BorrowMut<RangeFrom<usize>> = RangeFrom<usize>,
> {
    inner: Inner,
    range: Range,
}

impl<Inner: Borrow<[u8]>, Range: BorrowMut<RangeFrom<usize>>> SlicedBytes<Inner, Range> {
    pub fn new_with_range(inner: Inner, range: Range) -> Self {
        Self { inner, range }
    }
}

impl<Inner: Borrow<[u8]>> SlicedBytes<Inner> {
    pub fn new(inner: Inner) -> Self {
        Self { inner, range: 0.. }
    }
}

// TODO: remove
impl From<Vec<u8>> for SlicedBytes {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl<Inner: Borrow<[u8]>, Range: BorrowMut<RangeFrom<usize>>> AsRef<[u8]>
    for SlicedBytes<Inner, Range>
{
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner.borrow()[self.range.borrow().clone()]
    }
}

impl<Inner: Borrow<[u8]>, Range: BorrowMut<RangeFrom<usize>>> super::AsyncWritableBytes
    for SlicedBytes<Inner, Range>
{
    fn truncate_start_at(&mut self, n: usize) {
        let range = self.range.borrow_mut();
        assert!(n + range.start <= self.inner.borrow().len());
        range.start += n;
    }
}
