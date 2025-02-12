use async_str_iter::any_str::IterAnyStr;
use frender_common::{
    reactive_value::ReactiveValueWithKind,
    strings::{CsrStr, NonReactiveStr, SsrStr},
    IntoStaticStr,
};
use frender_ssr::html::assert;

use crate::render_from::str::ValueKindForStr;

mod no_inner_text {
    use frender_common::{
        proxy_reactive_value,
        reactive_value::{non_reactive::Uncached, ReactiveValue, ReactiveValueWithKind},
        value_kind::KindOfOwned,
    };

    use crate::render_from::str::ValueForStr;

    pub struct ScriptContentNoInnerText;

    #[derive(Clone, Copy, Default)]
    pub struct EmptyStr;

    impl ValueForStr for EmptyStr {
        fn render_str_from_self(self, renderer: impl crate::render_from::str::RenderFromKnownStr) {
            renderer.render_from("");
        }
    }

    type KindOfEmptyStr = KindOfOwned<EmptyStr>;

    impl ReactiveValue<KindOfEmptyStr> for ScriptContentNoInnerText {
        proxy_reactive_value!(
            for<ValueKind = KindOfEmptyStr> |self| -> Uncached<EmptyStr> { Uncached(EmptyStr) }
        );
    }

    impl ReactiveValueWithKind for ScriptContentNoInnerText {
        type ReactiveValueKind = KindOfEmptyStr;
    }
}

pub trait IntoScriptContent {
    type IntoScriptContent: assert::ScriptContent;
    fn into_script_content(this: Self) -> Self::IntoScriptContent;

    type IntoScriptInnerText: ReactiveValueWithKind<ReactiveValueKind: ValueKindForStr>;
    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText;
}

impl IntoScriptContent for crate::Empty {
    type IntoScriptContent = async_str_iter::empty::Empty;

    fn into_script_content(Self: Self) -> Self::IntoScriptContent {
        async_str_iter::empty::Empty
    }

    type IntoScriptInnerText = no_inner_text::ScriptContentNoInnerText;

    fn into_script_inner_text(Self: Self) -> Self::IntoScriptInnerText {
        no_inner_text::ScriptContentNoInnerText
    }
}

pub struct ScriptInnerTextWronglyEncoded<S: SsrStr + CsrStr>(pub S);

impl<S: SsrStr + CsrStr> IntoScriptContent for ScriptInnerTextWronglyEncoded<S> {
    type IntoScriptContent =
        frender_ssr::html::script::IterScriptInnerTextWronglyEncoded<IterAnyStr<S::StaticStr>>;

    fn into_script_content(this: Self) -> Self::IntoScriptContent {
        Self::IntoScriptContent::new(IterAnyStr::new(
            this.0.into_into_static_str().into_static_str(),
        ))
    }

    type IntoScriptInnerText = NonReactiveStr<S>;

    fn into_script_inner_text(this: Self) -> Self::IntoScriptInnerText {
        NonReactiveStr(this.0)
    }
}
