use crate::AsyncStrIterator;

impl<'a, 'b> AsyncStrIterator for &'a [&'b str] {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        let this = self.get_mut();

        if let Some((v, new_this)) = this.split_first() {
            *this = new_this;
            std::task::Poll::Ready(Some(v))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}
