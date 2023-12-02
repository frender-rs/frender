use frender_html_common::maybe_str::MaybeStr;

pub struct ScriptInnerTextWronglyEncoded<S: MaybeStr>(pub S);

impl<S: MaybeStr> frender_ssr::html::script::SsrElementScriptContent
    for ScriptInnerTextWronglyEncoded<S>
{
    type ScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<S::OneStringOrEmpty>;

    fn into_script_content(self) -> Self::ScriptContent {
        Self::ScriptContent::new(S::into_one_string_or_empty(self.0))
    }
}
