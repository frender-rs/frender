// use frender_html_common::attr::HtmlAttributeValue;
// use futures_io::AsyncWrite;

// use super::{
//     bytes::{AsyncWritableBytes, IntoAsyncWritableBytes, SlicedBytes},
//     element::html::HtmlAttrPair,
//     MaybeOwned,
// };

pub use imps::{AsyncWritableAttrValueBooleanTrue, AsyncWritableAttrValueStr};
pub use into::IntoAsyncWritableAttrs;
pub use writable::AsyncWritableAttrs;

pub mod write_traits {
    use std::{ops::RangeFrom, task::Poll};

    use crate::write::str::AsyncWritableStr;

    use super::states::{AsyncWriteAttrNameState, AsyncWriteAttrValueState};

    pub trait AsyncWriteAttrName {
        type AsyncWriteAttrValue: AsyncWriteAttrValue<AsyncWriteAttrName = Self>;

        fn poll_write_attr_name<N: AsyncWritableStr>(
            self,
            cx: &mut std::task::Context,
            attr_name: &mut N,
            state: &mut AsyncWriteAttrNameState,
        ) -> Poll<std::io::Result<Self::AsyncWriteAttrValue>>;
    }

    pub trait AsyncWriteAttrValue {
        type AsyncWriteAttrName: AsyncWriteAttrName<AsyncWriteAttrValue = Self>;
        fn poll_write_attr_value_boolean_true(
            self,
            cx: &mut std::task::Context,
        ) -> Poll<std::io::Result<Self::AsyncWriteAttrName>>;
        fn poll_write_attr_value_str<S: AsyncWritableStr>(
            self,
            cx: &mut std::task::Context,
            state: &mut AsyncWriteAttrValueState<S>,
        ) -> Poll<std::io::Result<Self::AsyncWriteAttrName>>;
    }
}

pub mod states {
    use std::ops::RangeFrom;

    // use futures_io::AsyncWrite;

    use crate::write::str::AsyncWritableStr;

    // use super::super::bytes::{AsyncWritableBytes, SlicedBytes};
    use super::super::str::StrWriteState;

    pub struct AsyncWriteAttrNameState {
        pub space: RangeFrom<usize>,
    }

    impl Default for AsyncWriteAttrNameState {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AsyncWriteAttrNameState {
        fn new() -> Self {
            Self { space: 0.. }
        }
    }

    pub struct AsyncWriteAttrValueState<AttrValueStr: AsyncWritableStr> {
        pub eq_dq: RangeFrom<usize>,
        pub attr_value: AttrValueStr,
        pub dq: RangeFrom<usize>,
    }

    impl<AttrValueStr: AsyncWritableStr> AsyncWriteAttrValueState<AttrValueStr> {
        pub fn new(attr_value: AttrValueStr) -> Self {
            Self {
                eq_dq: 0..,
                attr_value,
                dq: 0..,
            }
        }
    }

    impl<S: std::borrow::Borrow<str>> AsyncWriteAttrValueState<crate::write::str::StrWriting<S>> {
        pub fn new_from_str(s: S) -> Self {
            Self::new(crate::write::str::StrWriting::new(s))
        }
    }
}

pub mod writable {
    use std::task::Poll;

    use crate::write::str::{AsyncWritableStr, AsyncWriteStr};

    use super::write_traits::{AsyncWriteAttrName, AsyncWriteAttrValue};

    pub trait AsyncWritableAttrValue {
        fn poll_write_attr_value_into<W: AsyncWriteAttrValue>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> Poll<std::io::Result<W::AsyncWriteAttrName>>;
    }

    pub trait AsyncWritableAttrs {
        fn poll_write_attrs_into<W: AsyncWriteAttrName>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> Poll<std::io::Result<W>>;
    }

    impl<Attrs: AsyncWritableAttrs> AsyncWritableAttrs for Option<Attrs> {
        fn poll_write_attrs_into<W: AsyncWriteAttrName>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> Poll<std::io::Result<W>> {
            if let Some(attrs) = self {
                attrs.poll_write_attrs_into(cx, write)
            } else {
                Poll::Ready(Ok(write))
            }
        }
    }

    impl AsyncWritableAttrs for () {
        fn poll_write_attrs_into<W: AsyncWriteAttrName>(
            &mut self,
            _: &mut std::task::Context,
            write: W,
        ) -> Poll<std::io::Result<W>> {
            Poll::Ready(Ok(write))
        }
    }

    impl<A: AsyncWritableAttrs, B: AsyncWritableAttrs> AsyncWritableAttrs for (A, B) {
        fn poll_write_attrs_into<W: AsyncWriteAttrName>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> Poll<std::io::Result<W>> {
            let write = crate::ready_ok!(A::poll_write_attrs_into(&mut self.0, cx, write));
            B::poll_write_attrs_into(&mut self.1, cx, write)
        }
    }
}

mod into {
    pub trait IntoAsyncWritableAttrs {
        type AsyncWritableAttrs: super::writable::AsyncWritableAttrs;

        fn into_async_writable_attrs(this: Self) -> Self::AsyncWritableAttrs;
    }

    impl IntoAsyncWritableAttrs for () {
        type AsyncWritableAttrs = ();

        #[inline(always)]
        fn into_async_writable_attrs(this: Self) -> Self::AsyncWritableAttrs {
            this
        }
    }

    impl<A: IntoAsyncWritableAttrs, B: IntoAsyncWritableAttrs> IntoAsyncWritableAttrs for (A, B) {
        type AsyncWritableAttrs = (A::AsyncWritableAttrs, B::AsyncWritableAttrs);

        fn into_async_writable_attrs(this: Self) -> Self::AsyncWritableAttrs {
            (
                A::into_async_writable_attrs(this.0),
                B::into_async_writable_attrs(this.1),
            )
        }
    }
}

mod imps {
    use crate::write::str::{AsyncWritableStr, StrWriting};

    use super::{states::AsyncWriteAttrValueState, writable::AsyncWritableAttrValue};

    pub struct AsyncWritableAttrValueBooleanTrue;

    impl AsyncWritableAttrValue for AsyncWritableAttrValueBooleanTrue {
        fn poll_write_attr_value_into<W: super::write_traits::AsyncWriteAttrValue>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> std::task::Poll<std::io::Result<W::AsyncWriteAttrName>> {
            write.poll_write_attr_value_boolean_true(cx)
        }
    }
    pub struct AsyncWritableAttrValueStr<S: AsyncWritableStr> {
        write_state: AsyncWriteAttrValueState<S>,
    }

    impl<S: AsyncWritableStr> AsyncWritableAttrValueStr<S> {
        pub fn new(attr_value: S) -> Self {
            Self {
                write_state: AsyncWriteAttrValueState::new(attr_value),
            }
        }
    }

    impl<S: std::borrow::Borrow<str>> AsyncWritableAttrValueStr<StrWriting<S>> {
        pub fn new_from_str(attr_value: S) -> Self {
            Self::new(StrWriting::new(attr_value))
        }
    }

    impl<S: AsyncWritableStr> AsyncWritableAttrValue for AsyncWritableAttrValueStr<S> {
        fn poll_write_attr_value_into<W: super::write_traits::AsyncWriteAttrValue>(
            &mut self,
            cx: &mut std::task::Context,
            write: W,
        ) -> std::task::Poll<std::io::Result<W::AsyncWriteAttrName>> {
            write.poll_write_attr_value_str(cx, &mut self.write_state)
        }
    }
}
