use std::{pin::Pin, task::Poll};

use crate::{AsyncStrIterator, IntoAsyncStrIterator};

pin_project_lite::pin_project!(
    pub struct Flat<I: Iterator>
    where
        I::Item: IntoAsyncStrIterator,
    {
        iter: I,
        #[pin]
        current: Option<<I::Item as IntoAsyncStrIterator>::IntoAsyncStrIterator>,
        current_is_over: bool,
    }
);

impl<I: Iterator> Flat<I>
where
    I::Item: IntoAsyncStrIterator,
{
    pub fn new(mut iter: I) -> Self {
        let current = iter
            .next()
            .map(IntoAsyncStrIterator::into_async_str_iterator);
        Self {
            iter,
            current,
            current_is_over: false,
        }
    }
}

impl<I: Iterator> AsyncStrIterator for Flat<I>
where
    I::Item: IntoAsyncStrIterator,
{
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let mut this = self.project();

        if *this.current_is_over || this.current.is_none() {
            if let Some(v) = this.iter.next() {
                *this.current_is_over = false;
                this.current.set(Some(v.into_async_str_iterator()));

                let () = frender_common::ready_none!(this
                    .current
                    .as_pin_mut()
                    .unwrap()
                    .poll_next_str(cx));
                // this.current.set(None);
                *this.current_is_over = true;

                Poll::Ready(Some(""))
            } else {
                this.current.set(None);
                Poll::Ready(None)
            }
        } else {
            let () =
                frender_common::ready_none!(this.current.as_pin_mut().unwrap().poll_next_str(cx));
            // this.current.set(None);
            *this.current_is_over = true;
            Poll::Ready(Some(""))
        }
    }
}
