use async_str_iter::IntoAsyncStrIterator;
use frender_html_common::MaybeValue;
use frender_ssr::{html::verbatim_html::DangerousVerbatimHtml, SsrElement};

/// In `csr`, this sets `element.innerHTML = self.0` only if `self.0` changed.
/// Invalid html will cause [`SyntaxError`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML#syntaxerror).
///
/// In `ssr`, this uses [`DangerousVerbatimHtml`] which is more dangerous than `csr`
/// because invalid html is allowed.
pub struct DangerousInnerHtml<S: MaybeValue<str> + IntoAsyncStrIterator>(pub S);

impl<S: MaybeValue<str> + IntoAsyncStrIterator> SsrElement for DangerousInnerHtml<S> {
    type HtmlChildren = DangerousVerbatimHtml<S::IntoAsyncStrIterator>;

    fn into_html_children(self) -> Self::HtmlChildren {
        DangerousVerbatimHtml::new(self.0.into_async_str_iterator())
    }
}
