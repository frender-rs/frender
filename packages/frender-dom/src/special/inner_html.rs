/// In `csr`, this sets `element.innerHTML = self.0`.
/// Invalid html will cause [`SyntaxError`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML#syntaxerror).
///
/// For now, `DangerousInnerHtml` doesn't implement [SsrElement] because
/// frender doesn't want to include a html validator.
/// Implement [SsrElement] for your custom type with [`DangerousVerbatimHtml`] if ssr is needed.
///
/// [`SsrElement`]: frender_ssr::SsrElement
/// [`DangerousVerbatimHtml`]: frender_ssr::html::verbatim_html::DangerousVerbatimHtml
pub struct DangerousInnerHtml<S>(pub S);
