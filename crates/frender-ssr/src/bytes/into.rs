use std::borrow::Cow;

pub trait IntoAsyncWritableBytes {
    type Bytes: super::AsyncWritableBytes;
    fn into_async_writable_bytes(self) -> Self::Bytes;
}

impl<'a> IntoAsyncWritableBytes for &'a [u8] {
    type Bytes = Self;

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        self
    }
}

impl<'a> IntoAsyncWritableBytes for Vec<u8> {
    type Bytes = super::SlicedBytes;

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        self.into()
    }
}

impl<'a> IntoAsyncWritableBytes for Cow<'a, [u8]> {
    type Bytes = super::CowSlicedBytes<'a>;

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        self.into()
    }
}

impl<'a> IntoAsyncWritableBytes for &'a str {
    type Bytes = &'a [u8];

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        self.as_bytes()
    }
}

impl IntoAsyncWritableBytes for String {
    type Bytes = super::SlicedBytes;

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        self.into_bytes().into()
    }
}

impl<'a> IntoAsyncWritableBytes for Cow<'a, str> {
    type Bytes = super::CowSlicedBytes<'a>;

    #[inline]
    fn into_async_writable_bytes(self) -> Self::Bytes {
        match self {
            Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
            Cow::Owned(s) => Cow::Owned(s.into_bytes()),
        }
        .into()
    }
}
