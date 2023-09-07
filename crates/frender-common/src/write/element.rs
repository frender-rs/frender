use std::{io, ops::RangeFrom, pin::Pin, task::Poll};

use super::{attrs::write_traits::AsyncWriteAttrName, str::StrWriteState};

pub struct WriteElementTagStartState {
    lt_unwritten: RangeFrom<usize>,
    tag_write_state: StrWriteState,
}

pub struct WriteElementTagCloseState {
    lt_unwritten: RangeFrom<usize>,
    tag_write_state: StrWriteState,
}

impl Default for WriteElementTagStartState {
    fn default() -> Self {
        Self {
            lt_unwritten: 0..,
            tag_write_state: Default::default(),
        }
    }
}

pub struct WriteVoidOrSelfClosingState {
    unwritten: RangeFrom<usize>,
}

pub trait AsyncWriteChildren {
    type AsyncWriteElementTagStart;
    fn poll_write_children<E: AsyncWritableElement>(
        self,
        children: E,
        tag: &str,
        state: &mut WriteElementTagCloseState,
    ) -> Poll<io::Result<Self::AsyncWriteElementTagStart>>;

    fn poll_write_void(
        self,
        state: &mut WriteVoidOrSelfClosingState,
    ) -> Poll<io::Result<Self::AsyncWriteElementTagStart>>;

    fn poll_write_self_closing(
        self,
        state: &mut WriteVoidOrSelfClosingState,
    ) -> Poll<io::Result<Self::AsyncWriteElementTagStart>>;
}

pub trait AsyncWritableElement {
    fn poll_write_element_into<W: AsyncWriteElementTagStart>(
        self,
        writer: W,
    ) -> Poll<io::Result<W>>;
}

pub trait AsyncWriteElementTagStart {
    type AsyncWriteElementAttrsAndChildren: AsyncWriteAttrName
        + AsyncWriteChildren<AsyncWriteElementTagStart = Self>;

    /// `<div`
    fn poll_write_element_tag_start(
        self,
        tag: &str,
        state: &mut WriteElementTagStartState,
    ) -> Poll<io::Result<Self::AsyncWriteElementAttrsAndChildren>>;
}
