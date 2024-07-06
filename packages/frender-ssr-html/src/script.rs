use async_str_iter::AsyncStrIterator;

use crate::encode::Encode;

pin_project_lite::pin_project!(
    /// Currently this might not work as expected due to the following reasons:
    ///
    ///  - [`html_escape::encode_script`] is not perfect, see [`crate::escape_safe::Script`].
    ///  - `S: AsyncStrIterator` is not encoded if it yields more than one string chunks
    ///    and `</script>` is split into multiple chunks.
    pub struct IterScriptInnerTextWronglyEncoded<S: AsyncStrIterator> {
        #[pin]
        encode: Encode<crate::escape_safe::Script, S>,
    }
);

impl<S: AsyncStrIterator> AsyncStrIterator for IterScriptInnerTextWronglyEncoded<S> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        self.project().encode.poll_next_str(cx)
    }
}

impl<S: AsyncStrIterator> IterScriptInnerTextWronglyEncoded<S> {
    pub fn new(s: S) -> Self {
        Self {
            encode: Encode::new(crate::escape_safe::Script, s),
        }
    }
}
