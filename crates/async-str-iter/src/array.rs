use crate::{AsyncStrIterator, IntoAsyncStrIterator};

pub struct IterArray<T, const N: usize> {
    array: [T; N],
    current: usize,
}

impl<T: AsyncStrIterator + Unpin, const N: usize> AsyncStrIterator for IterArray<T, N> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.get_mut();
        loop {
            // if this.current
        }
    }
}

impl<'a, 'b> AsyncStrIterator for std::slice::Iter<'a, &'b str> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        std::task::Poll::Ready(self.get_mut().next().copied())
    }
}

// impl<T: IntoAsyncStrIterator, const N: usize> IntoAsyncStrIterator for [T; N] {
//     type IntoAsyncStrIterator;

//     fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
//         todo!()
//     }
// }
