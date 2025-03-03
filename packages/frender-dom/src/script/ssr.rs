use frender_reactive_value::ssr::SsrStr;
use frender_ssr::html::assert;

use super::ScriptContent;

pub trait SsrScriptContent: ScriptContent {
    type IntoScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::IntoScriptContent;
}

impl SsrScriptContent for crate::Empty {
    type IntoScriptContent = async_str_iter::empty::Empty;

    fn into_script_content(Self: Self) -> Self::IntoScriptContent {
        async_str_iter::empty::Empty
    }
}

/// Requires `S: SsrStr` because this type is only need in server side rendering.
/// Use [`ScriptInnerTextCsrOnly`](super::ScriptInnerTextCsrOnly) for just csr.
pub struct ScriptInnerTextWronglyEncoded<S: SsrStr>(pub S);

impl<S: SsrStr> ScriptContent for ScriptInnerTextWronglyEncoded<S> {}

impl<S: SsrStr> SsrScriptContent for ScriptInnerTextWronglyEncoded<S> {
    type IntoScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<S::SsrStrIntoAsyncStrIterator>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        Self::IntoScriptContent::new(this.0.ssr_str_into_async_str_iterator())
    }
}

#[cfg(feature = "csr")]
mod csr {
    use frender_reactive_value::{ssr::SsrStr, ReactiveValueWithKind};

    use crate::csr::render_from::str::ValueKindForStr;

    use super::{super::csr::CsrScriptContent, ScriptInnerTextWronglyEncoded};

    impl<S: SsrStr + ReactiveValueWithKind<ReactiveValueKind: ValueKindForStr>> CsrScriptContent
        for ScriptInnerTextWronglyEncoded<S>
    {
        type IntoScriptInnerText = S;

        fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
            this.0
        }
    }
}
