use std::{
    borrow::{Borrow, Cow},
    rc::Rc,
    sync::Arc,
};

pub enum MaybeOwned<'a, T: ToOwned + ?Sized> {
    Borrowed(&'a T),
    Owned(T::Owned),
    Rc(Rc<T>),
    Arc(Arc<T>),
}

impl<'a> MaybeOwned<'a, str> {
    pub fn encode_with(self, encode: impl FnOnce(&str) -> Cow<str>) -> Self {
        fn encode_cow<S: std::ops::Deref<Target = str> + Into<MaybeOwned<'static, str>>>(
            v: S,
            encode: impl FnOnce(&str) -> Cow<str>,
        ) -> MaybeOwned<'static, str> {
            let s: &str = &v;
            match encode(s) {
                Cow::Borrowed(r) => {
                    assert!(std::ptr::eq(s, r));
                    v.into()
                }
                Cow::Owned(v) => MaybeOwned::Owned(v),
            }
        }

        match self {
            MaybeOwned::Borrowed(v) => encode(v).into(),
            MaybeOwned::Owned(v) => encode_cow(v, encode),
            MaybeOwned::Rc(v) => encode_cow(v, encode),
            MaybeOwned::Arc(v) => encode_cow(v, encode),
        }
    }
}

impl<'a, T: ToOwned + ?Sized> std::ops::Deref for MaybeOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeOwned::Borrowed(v) => v,
            MaybeOwned::Owned(v) => v.borrow(),
            MaybeOwned::Rc(v) => v,
            MaybeOwned::Arc(v) => v,
        }
    }
}

impl<'a, T: ToOwned + ?Sized> AsRef<T> for MaybeOwned<'a, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ToOwned + ?Sized> From<Arc<T>> for MaybeOwned<'a, T> {
    fn from(v: Arc<T>) -> Self {
        Self::Arc(v)
    }
}

impl<'a, T: ToOwned + ?Sized> From<Rc<T>> for MaybeOwned<'a, T> {
    fn from(v: Rc<T>) -> Self {
        Self::Rc(v)
    }
}

impl<'a, T: ToOwned + ?Sized> From<&'a T> for MaybeOwned<'a, T> {
    fn from(v: &'a T) -> Self {
        Self::Borrowed(v)
    }
}

impl<'a, T: ToOwned + ?Sized> From<Cow<'a, T>> for MaybeOwned<'a, T> {
    fn from(v: Cow<'a, T>) -> Self {
        match v {
            Cow::Borrowed(v) => Self::Borrowed(v),
            Cow::Owned(v) => Self::Owned(v),
        }
    }
}

macro_rules! impl_from_owned_for {
    ($(($owned_ty:ty, $ty:ty)),* $(,)?) => {$(
        impl<'a> From<$owned_ty> for MaybeOwned<'a, $ty> {
            fn from(v: $owned_ty) -> Self {
                Self::Owned(v)
            }
        }
    )*};
}

impl_from_owned_for![
    //
    (String, str),
    (Vec<u8>, [u8]),
];

impl<'a> crate::bytes::IntoAsyncWritableBytes for MaybeOwned<'a, [u8]> {
    type Bytes = crate::bytes::AnyBytes<'a>;

    fn into_async_writable_bytes(self) -> Self::Bytes {
        use crate::bytes::SlicedBytes;
        match self {
            MaybeOwned::Borrowed(b) => crate::bytes::AnyBytes::Borrowed(b),
            MaybeOwned::Owned(b) => crate::bytes::AnyBytes::Owned(SlicedBytes::new(b)),
            MaybeOwned::Rc(b) => crate::bytes::AnyBytes::Rc(SlicedBytes::new(b)),
            MaybeOwned::Arc(b) => crate::bytes::AnyBytes::Arc(SlicedBytes::new(b)),
        }
    }
}
