use async_str_iter::any_str::IterAnyStr;
use frender_common::{strings::SsrStr, IntoStaticStr};
use frender_ssr::{html::verbatim_html::DangerousVerbatimHtml, SsrElement};

/// In `csr`, this sets `element.innerHTML = self.0` only if `self.0` changed.
/// Invalid html will cause [`SyntaxError`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML#syntaxerror).
///
/// In `ssr`, this uses [`DangerousVerbatimHtml`] which is more dangerous than `csr`
/// because invalid html is allowed.
pub struct DangerousInnerHtml<S>(pub S);

impl<S: SsrStr> SsrElement for DangerousInnerHtml<S> {
    type HtmlChildren = DangerousVerbatimHtml<IterAnyStr<S::StaticStr>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        DangerousVerbatimHtml::new(IterAnyStr::new(
            self.0.into_into_static_str().into_static_str(),
        ))
    }
}
