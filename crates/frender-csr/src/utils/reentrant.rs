use std::future::Future;

pin_project_lite::pin_project! {
    pub(crate) struct Reentrant<F> {
        #[pin]
        fut: F,
        finished: bool,
    }
}

impl<F: Future<Output = ()>> Future for Reentrant<F> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        if *this.finished {
            std::task::Poll::Ready(())
        } else {
            let res = this.fut.poll(cx);
            if res.is_ready() {
                *this.finished = true;
            }
            res
        }
    }
}

#[inline]
pub(crate) fn reentrant<F: Future<Output = ()>>(fut: F) -> Reentrant<F> {
    Reentrant {
        fut,
        finished: false,
    }
}
