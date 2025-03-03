use frender_reactive_value::{
    non_reactive::Uncached, proxy_reactive_value, value_kind::KindOfOwned, ReactiveValue,
    ReactiveValueWithKind,
};

use crate::csr::render_from::str::{RenderFromKnownStr, ValueForStr};

pub struct ScriptContentNoInnerText;

#[derive(Clone, Copy, Default)]
pub struct EmptyStr;

impl ValueForStr for EmptyStr {
    fn render_str_from_self(self, renderer: impl RenderFromKnownStr) {
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
