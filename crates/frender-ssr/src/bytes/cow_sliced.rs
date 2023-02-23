use std::borrow::Cow;

pub enum CowSlicedBytes<'a> {
    Borrowed(&'a [u8]),
    Owned(super::SlicedBytes),
}

impl<'a> std::ops::Deref for CowSlicedBytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            CowSlicedBytes::Borrowed(b) => b,
            CowSlicedBytes::Owned(b) => b.as_ref(),
        }
    }
}

impl<'a> From<Cow<'a, str>> for CowSlicedBytes<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(value) => CowSlicedBytes::Borrowed(value.as_bytes()),
            Cow::Owned(value) => CowSlicedBytes::Owned(value.into_bytes().into()),
        }
    }
}

impl<'a> From<Cow<'a, [u8]>> for CowSlicedBytes<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        match value {
            Cow::Borrowed(b) => CowSlicedBytes::Borrowed(b),
            Cow::Owned(b) => CowSlicedBytes::Owned(b.into()),
        }
    }
}

impl<'a> AsRef<[u8]> for CowSlicedBytes<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            CowSlicedBytes::Borrowed(b) => b,
            CowSlicedBytes::Owned(b) => b.as_ref(),
        }
    }
}

impl super::AsyncWritableBytes for CowSlicedBytes<'_> {
    fn truncate_start_at(&mut self, n: usize) {
        match self {
            CowSlicedBytes::Borrowed(b) => b.truncate_start_at(n),
            CowSlicedBytes::Owned(b) => b.truncate_start_at(n),
        }
    }
}
