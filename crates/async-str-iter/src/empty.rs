use std::{pin::Pin, task::Poll};

use crate::AsyncStrIterator;

pub struct Empty;

impl AsyncStrIterator for Empty {
    fn poll_next_str(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        Poll::Ready(None)
    }
}
