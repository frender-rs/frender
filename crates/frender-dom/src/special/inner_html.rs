use frender_html_common::maybe_str::MaybeStr;
use frender_ssr::{html::verbatim_html::DangerousVerbatimHtml, SsrElement};

/// In `csr`, this sets `element.innerHTML = self.0` only if `self.0` changed.
/// Invalid html will cause [`SyntaxError`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML#syntaxerror).
///
/// In `ssr`, this uses [`DangerousVerbatimHtml`] which is more dangerous than `csr`
/// because invalid html is allowed.
pub struct DangerousInnerHtml<S: MaybeStr>(pub S);

impl<S: MaybeStr> SsrElement for DangerousInnerHtml<S> {
    type HtmlChildren = DangerousVerbatimHtml<S::OneStringOrEmpty>;

    fn into_html_children(self) -> Self::HtmlChildren {
        DangerousVerbatimHtml::new(S::into_one_string_or_empty(self.0))
    }
}
