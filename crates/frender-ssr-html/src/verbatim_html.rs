use async_str_iter::AsyncStrIterator;

pin_project_lite::pin_project!(
    pub struct DangerousVerbatimHtml<S: AsyncStrIterator> {
        #[pin]
        html: S,
    }
);

impl<S: AsyncStrIterator> DangerousVerbatimHtml<S> {
    pub fn new(html: S) -> Self {
        Self { html }
    }
}

impl<S: AsyncStrIterator> AsyncStrIterator for DangerousVerbatimHtml<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        self.project().html.poll_next_str(cx)
    }
}
