use std::borrow::Borrow;

use crate::{AsyncStrIterator, IntoAsyncStrIterator};

pub struct IterBorrowStr<S: Borrow<str>> {
    s: S,
    taken: bool,
}

impl<S: Borrow<str>> IterBorrowStr<S> {
    pub fn new(s: S) -> Self {
        IterBorrowStr { s, taken: false }
    }
}

impl<S: Borrow<str>> Unpin for IterBorrowStr<S> {}

impl<S: Borrow<str>> AsyncStrIterator for IterBorrowStr<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.get_mut();
        std::task::Poll::Ready(if this.taken {
            None
        } else {
            this.taken = true;
            Some(this.s.borrow())
        })
    }
}

pub struct BorrowStr<S: Borrow<str>>(pub S);

impl<S: Borrow<str>> IntoAsyncStrIterator for BorrowStr<S> {
    type IntoAsyncStrIterator = IterBorrowStr<S>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        IterBorrowStr::new(self.0)
    }
}
