use std::future::Future;

use crate::AsyncStrIterator;

pin_project_lite::pin_project!(
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Collect<S: AsyncStrIterator, R: Default> {
        #[pin]
        str_iter: Option<S>,
        result: R,
    }
);

impl<S: AsyncStrIterator, R: Default> Collect<S, R> {
    pub fn new(str_iter: S) -> Self {
        Self {
            str_iter: Some(str_iter),
            result: Default::default(),
        }
    }
}

/// On ready, future polls would return default result.
impl<S: AsyncStrIterator, R: Default + for<'a> Extend<&'a str>> Future for Collect<S, R> {
    type Output = R;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();
        if let Some(mut str_iter) = this.str_iter.as_mut().as_pin_mut() {
            loop {
                match str_iter.as_mut().poll_next_str(cx) {
                    std::task::Poll::Ready(Some(s)) => this.result.extend(Some(s)),
                    std::task::Poll::Ready(None) => break,
                    std::task::Poll::Pending => return std::task::Poll::Pending,
                }
            }
        }

        this.str_iter.set(None);

        std::task::Poll::Ready(std::mem::take(this.result))
    }
}
