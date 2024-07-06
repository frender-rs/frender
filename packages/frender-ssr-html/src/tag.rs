use async_str_iter::AsyncStrIterator;

pin_project_lite::pin_project!(
    /// See [https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name].
    #[derive(Clone, Copy)]
    pub struct AssertTagName<V: AsyncStrIterator> {
        #[pin]
        v: V,
    }
);

impl<'a> AssertTagName<&'a str> {
    pub const fn try_from_str(v: &'a str) -> Option<Self> {
        if crate::utils::bytes_all!(v.as_bytes(), |b| match b {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => true,
            _ => false,
        }) {
            Some(Self { v })
        } else {
            None
        }
    }

    pub const fn new_from_str(v: &'a str) -> Self {
        match Self::try_from_str(v) {
            Some(this) => this,
            None => panic!("invalid TagName"),
        }
    }
}

impl<S: AsyncStrIterator> AsyncStrIterator for AssertTagName<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        self.project().v.poll_next_str(cx)
    }
}
