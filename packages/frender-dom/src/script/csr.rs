use frender_common::reactive_value::ReactiveValueWithKind;

use crate::{csr::render_from::str::ValueKindForStr, Empty};

mod no_inner_text;

pub trait CsrScriptContent {
    type IntoScriptInnerText: ReactiveValueWithKind<ReactiveValueKind: ValueKindForStr>;
    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText;
}

impl CsrScriptContent for Empty {
    type IntoScriptInnerText = no_inner_text::ScriptContentNoInnerText;

    fn into_script_inner_text(Self: Self) -> Self::IntoScriptInnerText {
        no_inner_text::ScriptContentNoInnerText
    }
}

/// Updates `script.innerText` reactively.
pub struct ScriptInnerTextCsrOnly<S: ReactiveValueWithKind<ReactiveValueKind: ValueKindForStr>>(
    pub S,
);

impl<S: ReactiveValueWithKind<ReactiveValueKind: ValueKindForStr>> CsrScriptContent
    for ScriptInnerTextCsrOnly<S>
{
    type IntoScriptInnerText = S;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        this.0
    }
}
