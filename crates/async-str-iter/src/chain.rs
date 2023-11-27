use std::{pin::Pin, task::Poll};

use crate::AsyncStrIterator;

pin_project_lite::pin_project!(
    #[project = ChainProj]
    pub struct Chain<A, B> {
        #[pin]
        a: A,
        a_ready: bool,
        #[pin]
        b: B,
    }
);

impl<A, B> Chain<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a_ready: false,
            a,
            b,
        }
    }
}

impl<A: AsyncStrIterator, B: AsyncStrIterator> AsyncStrIterator for Chain<A, B> {
    fn poll_next_str(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<&str>> {
        let ChainProj { a_ready, a, b } = self.project();

        if !*a_ready {
            let () = crate::__private::ready_none!(a.poll_next_str(cx));
            *a_ready = true;
        }

        b.poll_next_str(cx)
    }
}
