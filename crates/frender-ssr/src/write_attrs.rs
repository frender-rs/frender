use std::{borrow::Cow, ops::RangeFrom, pin::Pin, task::Poll};

use frender_common::write::{
    attrs::{
        self,
        states::{AsyncWriteAttrNameState, AsyncWriteAttrValueState},
    },
    str::{AsyncWritableStr, AsyncWriteStr, StrWriteState, StrWritingState},
};
use futures_io::AsyncWrite;

use crate::{
    bytes::{AsyncWritableBytes, SlicedBytes},
    EscapeSafe,
};

fn str_writing_state_as_mut_sliced_bytes<'a>(
    this: &'a mut StrWritingState,
    original_str: &'a str,
) -> SlicedBytes<&'a [u8], &'a mut RangeFrom<usize>> {
    SlicedBytes::new_with_range(this.encoded.as_bytes(original_str), &mut this.unwritten)
}

fn encode_and_poll_write<W: AsyncWrite + ?Sized>(
    this: &mut StrWriteState,
    encode: impl FnOnce(&str) -> Cow<str>,
    unencoded: &str,
    cx: &mut std::task::Context,
    writer: Pin<&mut W>,
) -> Poll<std::io::Result<()>> {
    str_writing_state_as_mut_sliced_bytes(this.encode_if_not(encode, unencoded), unencoded)
        .poll_write_bytes(writer, cx)
}

pub struct AsyncWriteAttrName<W: AsyncWrite + Unpin> {
    write: W,
}

impl<W: AsyncWrite + Unpin> AsyncWriteAttrName<W> {
    pub(crate) fn new(write: W) -> Self {
        Self { write }
    }
}

pub struct AsyncWriteAttrValue<W: AsyncWrite + Unpin> {
    write: W,
}

struct WriteAttrValueStrDoubleQuoted<W: AsyncWrite + Unpin> {
    write: W,
}

impl<W: AsyncWrite + Unpin> attrs::write_traits::AsyncWriteAttrValue for AsyncWriteAttrValue<W> {
    type AsyncWriteAttrName = AsyncWriteAttrName<W>;

    fn poll_write_attr_value_boolean_true(
        self,
        _: &mut std::task::Context,
    ) -> Poll<std::io::Result<Self::AsyncWriteAttrName>> {
        Poll::Ready(Ok(AsyncWriteAttrName { write: self.write }))
    }

    fn poll_write_attr_value_str<S: AsyncWritableStr>(
        mut self,
        cx: &mut std::task::Context,
        state: &mut AsyncWriteAttrValueState<S>,
    ) -> Poll<std::io::Result<Self::AsyncWriteAttrName>> {
        crate::ready_ok_rewrap_err!(SlicedBytes::new_with_range(*b"=\"", &mut state.eq_dq)
            .poll_write_bytes(Pin::new(&mut self.write), cx));
        crate::ready_ok_rewrap_err!(state.attr_value.poll_write_str_into(
            cx,
            &mut WriteAttrValueStrDoubleQuoted {
                write: Pin::new(&mut self.write)
            }
        ));
        SlicedBytes::new_with_range(*b"\"", &mut state.dq)
            .poll_write_bytes(Pin::new(&mut self.write), cx)
            .map_ok(|()| AsyncWriteAttrName { write: self.write })
    }
}

impl<W: AsyncWrite + Unpin> attrs::write_traits::AsyncWriteAttrName for AsyncWriteAttrName<W> {
    type AsyncWriteAttrValue = AsyncWriteAttrValue<W>;

    fn poll_write_attr_name<N: AsyncWritableStr>(
        mut self,
        cx: &mut std::task::Context,
        attr_name: &mut N,
        state: &mut AsyncWriteAttrNameState,
    ) -> Poll<std::io::Result<Self::AsyncWriteAttrValue>> {
        crate::ready_ok_rewrap_err!(SlicedBytes::new_with_range(*b" ", &mut state.space)
            .poll_write_bytes(Pin::new(&mut self.write), cx));

        let mut write = AsyncWriteEncodedStr {
            write: self.write,
            encode: html_escape::encode_safe,
        };

        attr_name
            .poll_write_str_into(cx, &mut write)
            .map_ok(|()| AsyncWriteAttrValue { write: write.write })
    }
}

impl<W: AsyncWrite + Unpin> frender_common::write::str::AsyncWriteStr
    for WriteAttrValueStrDoubleQuoted<W>
{
    fn poll_write_str(
        &mut self,
        cx: &mut std::task::Context,
        chunk: &str,
        state: &mut StrWriteState,
    ) -> Poll<std::io::Result<()>> {
        encode_and_poll_write(
            state,
            html_escape::encode_double_quoted_attribute,
            chunk,
            cx,
            Pin::new(&mut self.write),
        )
    }
}

pub(crate) struct AsyncWriteEncodedStr<W: AsyncWrite + Unpin, E: EscapeSafe> {
    pub(crate) write: W,
    pub(crate) encode: E,
}

impl<W: AsyncWrite + Unpin, E: EscapeSafe> AsyncWriteStr for AsyncWriteEncodedStr<W, E> {
    fn poll_write_str(
        &mut self,
        cx: &mut std::task::Context,
        s: &str,
        write_state: &mut StrWriteState,
    ) -> Poll<std::io::Result<()>> {
        encode_and_poll_write(
            write_state,
            |input| self.encode.escape_safe(input),
            s,
            cx,
            Pin::new(&mut self.write),
        )
    }
}
