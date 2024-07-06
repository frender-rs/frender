use std::{pin::Pin, task::Poll};

use async_str_iter::AsyncStrIterator;

use crate::escape_safe::EscapeSafe;

pin_project_lite::pin_project!(
    pub struct Encode<E: EscapeSafe, S: AsyncStrIterator> {
        escape: E,
        #[pin]
        s: S,
        encoded: String,
    }
);

impl<E: EscapeSafe, S: AsyncStrIterator> Encode<E, S> {
    pub const fn new(escape: E, s: S) -> Self {
        Self {
            escape,
            s,
            encoded: String::new(),
        }
    }
}

impl<E: EscapeSafe, S: AsyncStrIterator> AsyncStrIterator for Encode<E, S> {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let this = self.project();
        this.s.poll_next_str(cx).map(|v| {
            v.map(|v| match this.escape.escape_safe(v) {
                std::borrow::Cow::Borrowed(v) => v,
                std::borrow::Cow::Owned(v) => {
                    *this.encoded = v;
                    &*this.encoded
                }
            })
        })
    }
}
