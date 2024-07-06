use std::{pin::Pin, task::Poll};

use crate::{AsyncStrIterator, IntoAsyncStrIterator};

pin_project_lite::pin_project!(
    pub struct IterOption<T> {
        #[pin]
        inner: Option<T>,
        // TODO: fuse?
        // ended: bool,
    }
);

impl<T: AsyncStrIterator> AsyncStrIterator for IterOption<T> {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        match self.project().inner.as_pin_mut() {
            Some(this) => this.poll_next_str(cx),
            None => Poll::Ready(None),
        }
    }
}

impl<T: IntoAsyncStrIterator> IntoAsyncStrIterator for Option<T> {
    type IntoAsyncStrIterator = IterOption<T::IntoAsyncStrIterator>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        IterOption {
            inner: self.map(IntoAsyncStrIterator::into_async_str_iterator),
        }
    }
}
